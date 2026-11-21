// Derived from Barretenberg (https://github.com/AztecProtocol/aztec-packages, Copyright 2022 Aztec, Apache-2.0):
// the verification steps of the bb v6.0.0 Solidity verifier. Changed: expressed as a gnark circuit.

// Package circuit is a Groth16 (BN254) circuit that verifies one Barretenberg v6.0.0 UltraHonk proof (UltraFlavor:
// Poseidon2 transcript, no ZK, padded to 25 rounds) against one fixed verification key, step for step like
// honk.Verify. The inner verification key is compiled in as constants, so the Groth16 verifying key of the circuit
// pins the inner circuit; the inner public inputs are the Groth16 public inputs.
package circuit

import (
	"errors"
	"math/big"

	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/std/algebra/emulated/sw_bn254"
	"github.com/consensys/gnark/std/algebra/emulated/sw_emulated"
	"github.com/consensys/gnark/std/math/bits"
	"github.com/consensys/gnark/std/math/emulated"

	"github.com/wstran/ultrahonk-gnark/honk"
)

const logN = honk.ConstProofSizeLogN

// Circuit is built with New; the zero value does not compile.
type Circuit struct {
	Proof        [honk.ProofFieldCount]frontend.Variable
	PublicInputs []frontend.Variable `gnark:",public"`

	vk *honk.VerificationKey
}

// New returns the circuit for vk, used both for compilation and as the assignment template.
func New(vk *honk.VerificationKey) *Circuit {
	return &Circuit{PublicInputs: make([]frontend.Variable, vk.NumUserPublicInputs()), vk: vk}
}

// Assign returns a full assignment for an UltraHonk proof (in field form) and its user public inputs.
func Assign(vk *honk.VerificationKey, proof, publicInputs []fr.Element) (*Circuit, error) {
	if len(proof) != honk.ProofFieldCount || len(publicInputs) != vk.NumUserPublicInputs() {
		return nil, errors.New("circuit: wrong proof or public inputs length")
	}

	a := New(vk)

	for i := range a.Proof {
		a.Proof[i] = proof[i]
	}

	for i := range a.PublicInputs {
		a.PublicInputs[i] = publicInputs[i]
	}

	return a, nil
}

type gpoint struct {
	p     *sw_bn254.G1Affine
	limbs []frontend.Variable
	inf   frontend.Variable
}

type verifier struct {
	api   frontend.API
	fp    *emulated.Field[emulated.BN254Fp]
	fr    *emulated.Field[emulated.BN254Fr]
	curve *sw_emulated.Curve[emulated.BN254Fp, emulated.BN254Fr]
}

func (v *verifier) limb(x frontend.Variable, nbits int) []frontend.Variable {
	return bits.ToBinary(v.api, x, bits.WithNbDigits(nbits))
}

// point decodes [x_lo, x_hi, y_lo, y_hi] (136 and 118-bit limbs, coordinates < q); (0, 0) is the identity,
// any other point must be on the curve (BN254 G1 has cofactor 1).
func (v *verifier) point(limbs []frontend.Variable) gpoint {
	coord := func(lo, hi frontend.Variable) *emulated.Element[emulated.BN254Fp] {
		e := v.fp.FromBits(append(v.limb(lo, 136), v.limb(hi, 118)...)...)

		v.fp.AssertIsInRange(e)

		return e
	}

	x := coord(limbs[0], limbs[1])

	y := coord(limbs[2], limbs[3])

	p := &sw_bn254.G1Affine{X: *x, Y: *y}

	allZero := v.api.And(v.api.And(v.api.IsZero(limbs[0]), v.api.IsZero(limbs[1])), v.api.And(v.api.IsZero(limbs[2]), v.api.IsZero(limbs[3])))

	v.curve.AssertIsOnCurve(v.curve.Select(allZero, v.curve.Generator(), p))

	return gpoint{p: p, limbs: limbs, inf: allZero}
}

// limbsOf returns the transcript limbs of an in-circuit point.
func (v *verifier) limbsOf(p *sw_bn254.G1Affine) []frontend.Variable {
	split := func(e *emulated.Element[emulated.BN254Fp]) (frontend.Variable, frontend.Variable) {
		b := v.fp.ToBitsCanonical(e)

		return bits.FromBinary(v.api, b[:136]), bits.FromBinary(v.api, b[136:254])
	}

	xl, xh := split(&p.X)

	yl, yh := split(&p.Y)

	return []frontend.Variable{xl, xh, yl, yh}
}

// scalar converts a native field element to an emulated BN254 scalar through its canonical bits.
func (v *verifier) scalar(s frontend.Variable) *emulated.Element[emulated.BN254Fr] {
	return v.fr.FromBits(bits.ToBinary(v.api, s)...)
}

func frConst(e fr.Element) *big.Int {
	return e.BigInt(new(big.Int))
}

func (c *Circuit) Define(api frontend.API) error {
	if c.vk == nil || len(c.PublicInputs) != c.vk.NumUserPublicInputs() {
		return errors.New("circuit: build the circuit with New")
	}

	fpField, err := emulated.NewField[emulated.BN254Fp](api)

	if err != nil {
		return err
	}

	frField, err := emulated.NewField[emulated.BN254Fr](api)

	if err != nil {
		return err
	}

	// Classic GLV computes its output instead of hinting it and is complete by default; on BN254 it costs about
	// 76k R1CS per variable-base point against 134k for GLV+fake-GLV (gnark v0.16.4).
	params := sw_emulated.GetBN254Params()

	params.PreferClassicGLV = true

	curve, err := sw_emulated.New[emulated.BN254Fp, emulated.BN254Fr](api, params)

	if err != nil {
		return err
	}

	v := &verifier{api: api, fp: fpField, fr: frField, curve: curve}

	pos := 0

	take := func(n int) []frontend.Variable {
		out := c.Proof[pos : pos+n]

		pos += n

		return out
	}

	ppo := take(honk.PairingPointsSize)

	var wit [8]gpoint

	for i := range wit {
		wit[i] = v.point(take(4))
	}

	w1, w2, w3, readCounts, readTags, w4, lookupInverses, zPerm := wit[0], wit[1], wit[2], wit[3], wit[4], wit[5], wit[6], wit[7]

	univariates := make([][]frontend.Variable, logN)

	for i := range univariates {
		univariates[i] = take(honk.BatchedRelationPartialLength)
	}

	evals := take(honk.NumberOfEntities)

	folds := make([]gpoint, logN-1)

	for i := range folds {
		folds[i] = v.point(take(4))
	}

	aEvals := take(logN)

	shplonkQ := v.point(take(4))

	kzg := v.point(take(4))

	// Transcript (bb v6): full-width challenges, c = H(previous || round data), a second challenge is H(first).
	round0 := []frontend.Variable{frConst(c.vk.Hash)}

	round0 = append(round0, c.PublicInputs...)

	round0 = append(round0, ppo...)

	for _, q := range []gpoint{w1, w2, w3} {
		round0 = append(round0, q.limbs...)
	}

	eta := poseidon2Hash(api, round0)

	romLogupGamma := poseidon2Hash(api, []frontend.Variable{eta})

	round1 := []frontend.Variable{romLogupGamma}

	for _, q := range []gpoint{readCounts, readTags, w4} {
		round1 = append(round1, q.limbs...)
	}

	beta := poseidon2Hash(api, round1)

	gamma := poseidon2Hash(api, []frontend.Variable{beta})

	alpha := poseidon2Hash(api, append(append([]frontend.Variable{gamma}, lookupInverses.limbs...), zPerm.limbs...))

	alphas := make([]frontend.Variable, honk.NumberOfAlphas)

	alphas[0] = alpha

	for i := 1; i < honk.NumberOfAlphas; i++ {
		alphas[i] = api.Mul(alphas[i-1], alpha)
	}

	prev := poseidon2Hash(api, []frontend.Variable{alpha})

	gates := make([]frontend.Variable, logN)

	gates[0] = prev

	for i := 1; i < logN; i++ {
		gates[i] = api.Mul(gates[i-1], gates[i-1])
	}

	us := make([]frontend.Variable, logN)

	for i := 0; i < logN; i++ {
		prev = poseidon2Hash(api, append([]frontend.Variable{prev}, univariates[i]...))

		us[i] = prev
	}

	rho := poseidon2Hash(api, append([]frontend.Variable{prev}, evals...))

	gR := []frontend.Variable{rho}

	for _, f := range folds {
		gR = append(gR, f.limbs...)
	}

	geminiR := poseidon2Hash(api, gR)

	nu := poseidon2Hash(api, append([]frontend.Variable{geminiR}, aEvals...))

	z := poseidon2Hash(api, append([]frontend.Variable{nu}, shplonkQ.limbs...))

	// Public input delta.
	offset := c.vk.PubInputsOffset

	var numerator, denominator frontend.Variable = 1, 1

	numeratorAcc := api.Add(gamma, api.Mul(beta, new(big.Int).SetUint64(honk.PermutationArgumentValueSeparator+offset)))

	denominatorAcc := api.Sub(gamma, api.Mul(beta, new(big.Int).SetUint64(offset+1)))

	for _, pi := range append(append([]frontend.Variable{}, c.PublicInputs...), ppo...) {
		numerator = api.Mul(numerator, api.Add(numeratorAcc, pi))

		denominator = api.Mul(denominator, api.Add(denominatorAcc, pi))

		numeratorAcc = api.Add(numeratorAcc, beta)

		denominatorAcc = api.Sub(denominatorAcc, beta)
	}

	delta := api.Mul(numerator, api.Inverse(denominator))

	// Sumcheck.
	denoms := honk.BarycentricDenominators()

	var target frontend.Variable = 0

	var pow frontend.Variable = 1

	for round := 0; round < logN; round++ {
		u := univariates[round]

		api.AssertIsEqual(api.Add(u[0], u[1]), target)

		chal := us[round]

		var num frontend.Variable = 1

		for i := 0; i < honk.BatchedRelationPartialLength; i++ {
			num = api.Mul(num, api.Sub(chal, i))
		}

		var next frontend.Variable = 0

		for i := 0; i < honk.BatchedRelationPartialLength; i++ {
			next = api.Add(next, api.Mul(u[i], api.Inverse(api.Mul(frConst(denoms[i]), api.Sub(chal, i)))))
		}

		target = api.Mul(next, num)

		pow = api.Mul(pow, api.Add(1, api.Mul(chal, api.Sub(gates[round], 1))))
	}

	api.AssertIsEqual(relations(api, evals, eta, romLogupGamma, beta, gamma, delta, alphas, pow), target)

	// Shplemini.
	powers := make([]frontend.Variable, logN)

	powers[0] = geminiR

	for i := 1; i < logN; i++ {
		powers[i] = api.Mul(powers[i-1], powers[i-1])
	}

	size := honk.NumberUnshifted + logN + 2

	scalars := make([]frontend.Variable, size)

	points := make([]*sw_bn254.G1Affine, size)

	posInv := api.Inverse(api.Sub(z, powers[0]))

	negInv := api.Inverse(api.Add(z, powers[0]))

	unshifted := api.Add(posInv, api.Mul(nu, negInv))

	shifted := api.Mul(api.Inverse(geminiR), api.Sub(posInv, api.Mul(nu, negInv)))

	var batching frontend.Variable = 1

	var batchedEvaluation frontend.Variable = 0

	unshiftedNeg := api.Neg(unshifted)

	shiftedNeg := api.Neg(shifted)

	for i := 1; i <= honk.NumberUnshifted; i++ {
		scalars[i] = api.Mul(unshiftedNeg, batching)

		batchedEvaluation = api.Add(batchedEvaluation, api.Mul(evals[i-1], batching))

		batching = api.Mul(batching, rho)
	}

	for i := 0; i < honk.NumberToBeShifted; i++ {
		off := i + honk.ShiftedCommitmentsStart

		scalars[off] = api.Add(scalars[off], api.Mul(shiftedNeg, batching))

		batchedEvaluation = api.Add(batchedEvaluation, api.Mul(evals[i+honk.NumberUnshifted], batching))

		batching = api.Mul(batching, rho)
	}

	// Identity vk commitments contribute nothing; the others are constants and use gnark's fixed-base comb.
	for i := range c.vk.Commitments {
		if !c.vk.Commitments[i].IsInfinity() {
			p := sw_bn254.NewG1Affine(c.vk.Commitments[i])

			points[1+i] = &p
		}
	}

	for i, q := range []gpoint{w1, w2, w3, w4, zPerm, lookupInverses, readCounts, readTags} {
		points[1+honk.NumVKCommitments+i] = q.p
	}

	fold := make([]frontend.Variable, logN)

	acc := batchedEvaluation

	for i := logN; i > 0; i-- {
		cp := powers[i-1]

		u := us[i-1]

		oneMinusU := api.Sub(1, u)

		roundAcc := api.Sub(api.Mul(cp, acc, 2), api.Mul(aEvals[i-1], api.Sub(api.Mul(cp, oneMinusU), u)))

		acc = api.Mul(roundAcc, api.Inverse(api.Add(api.Mul(cp, oneMinusU), u)))

		fold[i-1] = acc
	}

	constantTerm := api.Add(api.Mul(fold[0], posInv), api.Mul(aEvals[0], nu, negInv))

	nuSqr := api.Mul(nu, nu)

	batching = nuSqr

	for i := 0; i < logN-1; i++ {
		posInv = api.Inverse(api.Sub(z, powers[i+1]))

		negInv = api.Inverse(api.Add(z, powers[i+1]))

		sPos := api.Mul(batching, posInv)

		sNeg := api.Mul(batching, nu, negInv)

		scalars[honk.NumberUnshifted+1+i] = api.Sub(api.Neg(sNeg), sPos)

		constantTerm = api.Add(constantTerm, api.Mul(sNeg, aEvals[i+1]), api.Mul(sPos, fold[i+1]))

		batching = api.Mul(batching, nuSqr)

		points[honk.NumberUnshifted+1+i] = folds[i].p
	}

	_, _, g1, _ := bn254.Generators()

	generator := sw_bn254.NewG1Affine(g1)

	points[honk.NumberUnshifted+logN] = &generator

	scalars[honk.NumberUnshifted+logN] = constantTerm

	points[honk.NumberUnshifted+logN+1] = kzg.p

	scalars[honk.NumberUnshifted+logN+1] = z

	// The shplonk quotient has scalar 1 and is added directly.
	var msmPoints []*sw_bn254.G1Affine

	var msmScalars []*emulated.Element[emulated.BN254Fr]

	for i := 1; i < size; i++ {
		if points[i] == nil {
			continue
		}

		msmPoints = append(msmPoints, points[i])

		msmScalars = append(msmScalars, v.scalar(scalars[i]))
	}

	rest, err := curve.MultiScalarMul(msmPoints, msmScalars)

	if err != nil {
		return err
	}

	p0 := curve.AddUnified(shplonkQ.p, rest)

	p1 := curve.Neg(kzg.p)

	// Pairing points carried in the public inputs: all-zero limbs mean none; otherwise both must be finite points.
	isDefault := api.IsZero(api.Add(api.IsZero(ppo[0]), api.IsZero(ppo[1]), api.IsZero(ppo[2]), api.IsZero(ppo[3]), api.IsZero(ppo[4]), api.IsZero(ppo[5]), api.IsZero(ppo[6]), api.IsZero(ppo[7]), -8))

	otherLhs := v.point(ppo[0:4])

	otherRhs := v.point(ppo[4:8])

	notDefault := api.Sub(1, isDefault)

	api.AssertIsEqual(api.Mul(notDefault, otherLhs.inf), 0)

	api.AssertIsEqual(api.Mul(notDefault, otherRhs.inf), 0)

	sepIn := append(append(append(append([]frontend.Variable{}, otherLhs.limbs...), otherRhs.limbs...), v.limbsOf(p0)...), v.limbsOf(p1)...)

	separator := v.scalar(poseidon2Hash(api, sepIn))

	agg0 := curve.AddUnified(curve.ScalarMul(p0, separator), otherLhs.p)

	agg1 := curve.AddUnified(curve.ScalarMul(p1, separator), otherRhs.p)

	final0 := curve.Select(isDefault, p0, agg0)

	final1 := curve.Select(isDefault, p1, agg1)

	for _, f := range []*sw_bn254.G1Affine{final0, final1} {
		api.AssertIsEqual(api.And(fpField.IsZero(&f.X), fpField.IsZero(&f.Y)), 0)
	}

	pairing, err := sw_bn254.NewPairing(api)

	if err != nil {
		return err
	}

	g2gen, g2x := honk.PairingG2()

	q0 := sw_bn254.NewG2AffineFixed(g2gen)

	q1 := sw_bn254.NewG2AffineFixed(g2x)

	return pairing.PairingCheck([]*sw_bn254.G1Affine{final0, final1}, []*sw_bn254.G2Affine{&q0, &q1})
}
