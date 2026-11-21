package honk

import (
	"fmt"

	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
)

// Proof is an UltraFlavor proof padded to ConstProofSizeLogN sumcheck and Gemini rounds.
type Proof struct {
	PairingPointObject  [PairingPointsSize]fr.Element
	W1                  bn254.G1Affine
	W2                  bn254.G1Affine
	W3                  bn254.G1Affine
	LookupReadCounts    bn254.G1Affine
	LookupReadTags      bn254.G1Affine
	W4                  bn254.G1Affine
	LookupInverses      bn254.G1Affine
	ZPerm               bn254.G1Affine
	SumcheckUnivariates [ConstProofSizeLogN][BatchedRelationPartialLength]fr.Element
	SumcheckEvaluations [NumberOfEntities]fr.Element
	GeminiFoldComms     [ConstProofSizeLogN - 1]bn254.G1Affine
	GeminiAEvaluations  [ConstProofSizeLogN]fr.Element
	ShplonkQ            bn254.G1Affine
	KzgQuotient         bn254.G1Affine
}

// ParseProof reads the `proof` file of `bb prove -t noir-recursive-no-zk` (without the user public inputs,
// which bb writes to `public_inputs`). It must contain exactly ProofFieldCount fields.
func ParseProof(data []byte) (*Proof, error) {
	fields, err := FieldsFromBytes(data)

	if err != nil {
		return nil, err
	}

	return ParseProofFields(fields)
}

// ParseProofFields is ParseProof on field elements.
func ParseProofFields(fields []fr.Element) (*Proof, error) {
	if len(fields) != ProofFieldCount {
		return nil, fmt.Errorf("honk: proof has %d fields, expected %d", len(fields), ProofFieldCount)
	}

	r := &fieldReader{data: fields}

	p := &Proof{}

	for i := 0; i < PairingPointsSize; i++ {
		limb, _ := r.next()

		bound := two136

		if i%2 == 1 {
			bound = two118
		}

		if bigOf(limb).Cmp(bound) >= 0 {
			return nil, fmt.Errorf("honk: pairing point limb %d out of range", i)
		}

		p.PairingPointObject[i] = limb
	}

	for _, dst := range []*bn254.G1Affine{&p.W1, &p.W2, &p.W3, &p.LookupReadCounts, &p.LookupReadTags, &p.W4, &p.LookupInverses, &p.ZPerm} {
		pt, err := r.point()

		if err != nil {
			return nil, err
		}

		*dst = pt
	}

	for i := range p.SumcheckUnivariates {
		for j := range p.SumcheckUnivariates[i] {
			p.SumcheckUnivariates[i][j], _ = r.next()
		}
	}

	for i := range p.SumcheckEvaluations {
		p.SumcheckEvaluations[i], _ = r.next()
	}

	for i := range p.GeminiFoldComms {
		pt, err := r.point()

		if err != nil {
			return nil, err
		}

		p.GeminiFoldComms[i] = pt
	}

	for i := range p.GeminiAEvaluations {
		p.GeminiAEvaluations[i], _ = r.next()
	}

	var err error

	if p.ShplonkQ, err = r.point(); err != nil {
		return nil, err
	}

	if p.KzgQuotient, err = r.point(); err != nil {
		return nil, err
	}

	return p, nil
}
