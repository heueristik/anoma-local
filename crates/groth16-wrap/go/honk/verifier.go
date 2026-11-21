// Derived from Barretenberg (https://github.com/AztecProtocol/aztec-packages, Copyright 2022 Aztec, Apache-2.0):
// the sumcheck, Shplemini and pairing steps of the bb v6.0.0 Solidity verifier. Changed: translated to Go,
// Poseidon2 transcript, stricter pairing point rules.

package honk

import (
	"errors"
	"fmt"
	"math/big"

	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fp"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
)

func fpHex(s string) fp.Element {
	v, ok := new(big.Int).SetString(s, 16)

	if !ok {
		panic("honk: bad hex constant")
	}

	var e fp.Element

	e.SetBigInt(v)

	return e
}

// G2 points of the final pairing: the generator and [x]_2 of the Aztec SRS, as in bb's Solidity verifier.
var (
	g2Generator = bn254.G2Affine{
		X: bn254.E2{A0: fpHex("1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed"), A1: fpHex("198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2")},
		Y: bn254.E2{A0: fpHex("12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa"), A1: fpHex("090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b")},
	}
	g2X = bn254.G2Affine{
		X: bn254.E2{A0: fpHex("0118c4d5b837bcc2bc89b5b398b5974e9f5944073b32078b7e231fec938883b0"), A1: fpHex("260e01b251f6f1c7e7ff4e580791dee8ea51d87a358e038b4efe30fac09383c1")},
		Y: bn254.E2{A0: fpHex("22febda3c0c0632a56475b4214e5615e11e6dd3f96e6cea2854a87d4dacc5e55"), A1: fpHex("04fc6369f7110fe3d25156c1bb9a72859cf2a04641f99ba4ee413c80da6a5fe4")},
	}
)

func computePublicInputDelta(publicInputs []fr.Element, ppo *[PairingPointsSize]fr.Element, beta, gamma fr.Element, offset uint64) (fr.Element, error) {
	numerator, denominator := one, one

	numeratorAcc := add(gamma, mul(beta, frU(PermutationArgumentValueSeparator+offset)))

	denominatorAcc := sub(gamma, mul(beta, frU(offset+1)))

	all := append(append([]fr.Element{}, publicInputs...), ppo[:]...)

	for _, pi := range all {
		numerator = mul(numerator, add(numeratorAcc, pi))

		denominator = mul(denominator, add(denominatorAcc, pi))

		numeratorAcc = add(numeratorAcc, beta)

		denominatorAcc = sub(denominatorAcc, beta)
	}

	d, err := inv(denominator)

	if err != nil {
		return d, err
	}

	return mul(numerator, d), nil
}

var barycentricDenominators = func() [BatchedRelationPartialLength]fr.Element {
	var out [BatchedRelationPartialLength]fr.Element

	for i := 0; i < BatchedRelationPartialLength; i++ {
		d := one

		for j := 0; j < BatchedRelationPartialLength; j++ {
			if j != i {
				d = mul(d, sub(frU(uint64(i)), frU(uint64(j))))
			}
		}

		out[i] = d
	}

	return out
}()

func computeNextTargetSum(univariate *[BatchedRelationPartialLength]fr.Element, chal fr.Element) (fr.Element, error) {
	numerator := one

	for i := 0; i < BatchedRelationPartialLength; i++ {
		numerator = mul(numerator, sub(chal, frU(uint64(i))))
	}

	var target fr.Element

	for i := 0; i < BatchedRelationPartialLength; i++ {
		d, err := inv(mul(barycentricDenominators[i], sub(chal, frU(uint64(i)))))

		if err != nil {
			return target, err
		}

		target = add(target, mul(univariate[i], d))
	}

	return mul(target, numerator), nil
}

func verifySumcheck(p *Proof, t *Transcript) error {
	var roundTarget fr.Element

	pow := one

	for round := 0; round < ConstProofSizeLogN; round++ {
		u := &p.SumcheckUnivariates[round]

		sum := add(u[0], u[1])

		if !sum.Equal(&roundTarget) {
			return fmt.Errorf("honk: sumcheck round %d sum mismatch", round)
		}

		chal := t.SumcheckUChallenges[round]

		next, err := computeNextTargetSum(u, chal)

		if err != nil {
			return err
		}

		roundTarget = next

		pow = mul(pow, add(one, mul(chal, sub(t.GateChallenges[round], one))))
	}

	grand := AccumulateRelationEvaluations(&p.SumcheckEvaluations, &t.RelationParameters, &t.Alphas, pow)

	if !grand.Equal(&roundTarget) {
		return errors.New("honk: sumcheck final relation mismatch")
	}

	return nil
}

func msm(points []bn254.G1Affine, scalars []fr.Element) bn254.G1Affine {
	var acc bn254.G1Jac

	for i := range points {
		if points[i].IsInfinity() || scalars[i].IsZero() {
			continue
		}

		var t bn254.G1Jac

		t.FromAffine(&points[i])

		t.ScalarMultiplication(&t, bigOf(scalars[i]))

		acc.AddAssign(&t)
	}

	var out bn254.G1Affine

	out.FromJacobian(&acc)

	return out
}

func pairingPointsToG1(ppo *[PairingPointsSize]fr.Element) (bn254.G1Affine, bn254.G1Affine, error) {
	coord := func(lo, hi fr.Element) (fp.Element, error) {
		v := new(big.Int).Or(bigOf(lo), new(big.Int).Lsh(bigOf(hi), limbBits))

		var e fp.Element

		if v.Cmp(fp.Modulus()) >= 0 {
			return e, errors.New("honk: pairing point coordinate >= q")
		}

		e.SetBigInt(v)

		return e, nil
	}

	var lhs, rhs bn254.G1Affine

	var err error

	if lhs.X, err = coord(ppo[0], ppo[1]); err != nil {
		return lhs, rhs, err
	}

	if lhs.Y, err = coord(ppo[2], ppo[3]); err != nil {
		return lhs, rhs, err
	}

	if rhs.X, err = coord(ppo[4], ppo[5]); err != nil {
		return lhs, rhs, err
	}

	if rhs.Y, err = coord(ppo[6], ppo[7]); err != nil {
		return lhs, rhs, err
	}

	return lhs, rhs, nil
}

// PairingSeparator derives the recursion separator by Fiat-Shamir over both pairs of points (bb's native
// verifier draws it at random instead; any value fixed after the four points are is sound).
func PairingSeparator(otherLhs, otherRhs, p0, p1 bn254.G1Affine) fr.Element {
	var in []fr.Element

	for _, q := range []bn254.G1Affine{otherLhs, otherRhs, p0, p1} {
		in = append(in, PointLimbs(q)...)
	}

	return Poseidon2Hash(in)
}

func verifyShplemini(p *Proof, vk *VerificationKey, t *Transcript) error {
	const logN = ConstProofSizeLogN

	var powers [logN]fr.Element

	powers[0] = t.GeminiR

	for i := 1; i < logN; i++ {
		powers[i] = mul(powers[i-1], powers[i-1])
	}

	const size = NumberUnshifted + logN + 2

	var scalars [size]fr.Element

	var commitments [size]bn254.G1Affine

	posInv, err := inv(sub(t.ShplonkZ, powers[0]))

	if err != nil {
		return err
	}

	negInv, err := inv(add(t.ShplonkZ, powers[0]))

	if err != nil {
		return err
	}

	rInv, err := inv(t.GeminiR)

	if err != nil {
		return err
	}

	unshiftedScalar := add(posInv, mul(t.ShplonkNu, negInv))

	shiftedScalar := mul(rInv, sub(posInv, mul(t.ShplonkNu, negInv)))

	scalars[0] = one

	commitments[0] = p.ShplonkQ

	batching := one

	var batchedEvaluation fr.Element

	var unshiftedNeg, shiftedNeg fr.Element

	unshiftedNeg.Neg(&unshiftedScalar)

	shiftedNeg.Neg(&shiftedScalar)

	for i := 1; i <= NumberUnshifted; i++ {
		scalars[i] = mul(unshiftedNeg, batching)

		batchedEvaluation = add(batchedEvaluation, mul(p.SumcheckEvaluations[i-1], batching))

		batching = mul(batching, t.Rho)
	}

	for i := 0; i < NumberToBeShifted; i++ {
		off := i + ShiftedCommitmentsStart

		scalars[off] = add(scalars[off], mul(shiftedNeg, batching))

		batchedEvaluation = add(batchedEvaluation, mul(p.SumcheckEvaluations[i+NumberUnshifted], batching))

		batching = mul(batching, t.Rho)
	}

	copy(commitments[1:1+NumVKCommitments], vk.Commitments[:])

	for i, c := range []bn254.G1Affine{p.W1, p.W2, p.W3, p.W4, p.ZPerm, p.LookupInverses, p.LookupReadCounts, p.LookupReadTags} {
		commitments[1+NumVKCommitments+i] = c
	}

	var fold [logN]fr.Element

	acc := batchedEvaluation

	for i := logN; i > 0; i-- {
		cp := powers[i-1]

		u := t.SumcheckUChallenges[i-1]

		round := sub(mul(mul(cp, acc), frU(2)), mul(p.GeminiAEvaluations[i-1], sub(mul(cp, sub(one, u)), u)))

		d, err := inv(add(mul(cp, sub(one, u)), u))

		if err != nil {
			return err
		}

		acc = mul(round, d)

		fold[i-1] = acc
	}

	constantTerm := add(mul(fold[0], posInv), mul(mul(p.GeminiAEvaluations[0], t.ShplonkNu), negInv))

	nuSqr := mul(t.ShplonkNu, t.ShplonkNu)

	batching = nuSqr

	for i := 0; i < logN-1; i++ {
		if posInv, err = inv(sub(t.ShplonkZ, powers[i+1])); err != nil {
			return err
		}

		if negInv, err = inv(add(t.ShplonkZ, powers[i+1])); err != nil {
			return err
		}

		sPos := mul(batching, posInv)

		sNeg := mul(mul(batching, t.ShplonkNu), negInv)

		var s fr.Element

		s.Neg(&sNeg)

		scalars[NumberUnshifted+1+i] = sub(s, sPos)

		constantTerm = add(constantTerm, add(mul(sNeg, p.GeminiAEvaluations[i+1]), mul(sPos, fold[i+1])))

		batching = mul(batching, nuSqr)

		commitments[NumberUnshifted+1+i] = p.GeminiFoldComms[i]
	}

	_, _, g1, _ := bn254.Generators()

	commitments[NumberUnshifted+logN] = g1

	scalars[NumberUnshifted+logN] = constantTerm

	commitments[NumberUnshifted+logN+1] = p.KzgQuotient

	scalars[NumberUnshifted+logN+1] = t.ShplonkZ

	p0 := msm(commitments[:], scalars[:])

	var p1 bn254.G1Affine

	p1.Neg(&p.KzgQuotient)

	isDefault := true

	for _, limb := range p.PairingPointObject {
		if !limb.IsZero() {
			isDefault = false
		}
	}

	// Pairing points carried in the public inputs: all-zero limbs mean none; otherwise both must be finite curve
	// points (this also rejects the mixed-infinity accumulators fixed in bb v5.2.1).
	if !isDefault {
		otherLhs, otherRhs, err := pairingPointsToG1(&p.PairingPointObject)

		if err != nil {
			return err
		}

		if otherLhs.IsInfinity() || otherRhs.IsInfinity() {
			return errors.New("honk: pairing point at infinity")
		}

		if !otherLhs.IsOnCurve() || !otherRhs.IsOnCurve() {
			return errors.New("honk: pairing point not on curve")
		}

		separator := PairingSeparator(otherLhs, otherRhs, p0, p1)

		p0 = msm([]bn254.G1Affine{p0, otherLhs}, []fr.Element{separator, one})

		p1 = msm([]bn254.G1Affine{p1, otherRhs}, []fr.Element{separator, one})
	}

	if p0.IsInfinity() || p1.IsInfinity() {
		return errors.New("honk: pairing input at infinity")
	}

	ok, err := bn254.PairingCheck([]bn254.G1Affine{p0, p1}, []bn254.G2Affine{g2Generator, g2X})

	if err != nil {
		return err
	}

	if !ok {
		return errors.New("honk: pairing check failed")
	}

	return nil
}

// Verify returns nil if and only if proof is a valid bb v6.0.0 UltraFlavor proof for vk and the user public inputs.
func Verify(vk *VerificationKey, proof *Proof, publicInputs []fr.Element) error {
	if len(publicInputs) != vk.NumUserPublicInputs() {
		return errors.New("honk: wrong number of public inputs")
	}

	t := generateTranscript(proof, publicInputs, vk)

	delta, err := computePublicInputDelta(publicInputs, &proof.PairingPointObject, t.RelationParameters.Beta, t.RelationParameters.Gamma, vk.PubInputsOffset)

	if err != nil {
		return err
	}

	t.RelationParameters.PublicInputsDelta = delta

	if err := verifySumcheck(proof, t); err != nil {
		return err
	}

	return verifyShplemini(proof, vk, t)
}

// VerifyFiles verifies the vk, proof and public_inputs files written by bb.
func VerifyFiles(vkData, proofData, publicInputsData []byte) error {
	vk, err := ParseVK(vkData)

	if err != nil {
		return err
	}

	proof, err := ParseProof(proofData)

	if err != nil {
		return err
	}

	pi, err := FieldsFromBytes(publicInputsData)

	if err != nil {
		return err
	}

	return Verify(vk, proof, pi)
}
