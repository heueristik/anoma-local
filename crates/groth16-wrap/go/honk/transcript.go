// Derived from Barretenberg (https://github.com/AztecProtocol/aztec-packages, Copyright 2022 Aztec, Apache-2.0):
// the bb v6.0.0 transcript (transcript.hpp, oink_verifier.cpp, TranscriptLib). Changed: translated to Go
// with the Poseidon2 hash of UltraFlavor.

package honk

import (
	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
)

// RelationParameters are the Fiat-Shamir values the relations depend on.
type RelationParameters struct {
	Eta               fr.Element
	RomLogupGamma     fr.Element
	Beta              fr.Element
	Gamma             fr.Element
	PublicInputsDelta fr.Element
}

// Transcript holds every challenge of one verification.
type Transcript struct {
	RelationParameters  RelationParameters
	Alphas              [NumberOfAlphas]fr.Element
	GateChallenges      [ConstProofSizeLogN]fr.Element
	SumcheckUChallenges [ConstProofSizeLogN]fr.Element
	Rho                 fr.Element
	GeminiR             fr.Element
	ShplonkNu           fr.Element
	ShplonkZ            fr.Element
}

// generateTranscript replays bb v6's Poseidon2 transcript. Each challenge is a full field element
// c = H(previous challenge || round data); a second challenge in the same round is H(first).
func generateTranscript(p *Proof, publicInputs []fr.Element, vk *VerificationKey) *Transcript {
	t := &Transcript{}

	rp := &t.RelationParameters

	round0 := []fr.Element{vk.Hash}

	round0 = append(round0, publicInputs...)

	round0 = append(round0, p.PairingPointObject[:]...)

	for _, c := range []bn254.G1Affine{p.W1, p.W2, p.W3} {
		round0 = append(round0, PointLimbs(c)...)
	}

	rp.Eta = Poseidon2Hash(round0)

	rp.RomLogupGamma = Poseidon2Hash([]fr.Element{rp.Eta})

	round1 := []fr.Element{rp.RomLogupGamma}

	for _, c := range []bn254.G1Affine{p.LookupReadCounts, p.LookupReadTags, p.W4} {
		round1 = append(round1, PointLimbs(c)...)
	}

	rp.Beta = Poseidon2Hash(round1)

	rp.Gamma = Poseidon2Hash([]fr.Element{rp.Beta})

	round2 := append([]fr.Element{rp.Gamma}, append(PointLimbs(p.LookupInverses), PointLimbs(p.ZPerm)...)...)

	alpha := Poseidon2Hash(round2)

	t.Alphas[0] = alpha

	for i := 1; i < NumberOfAlphas; i++ {
		t.Alphas[i] = mul(t.Alphas[i-1], alpha)
	}

	prev := Poseidon2Hash([]fr.Element{alpha})

	t.GateChallenges[0] = prev

	for i := 1; i < ConstProofSizeLogN; i++ {
		t.GateChallenges[i] = mul(t.GateChallenges[i-1], t.GateChallenges[i-1])
	}

	for i := 0; i < ConstProofSizeLogN; i++ {
		prev = Poseidon2Hash(append([]fr.Element{prev}, p.SumcheckUnivariates[i][:]...))

		t.SumcheckUChallenges[i] = prev
	}

	t.Rho = Poseidon2Hash(append([]fr.Element{prev}, p.SumcheckEvaluations[:]...))

	gR := []fr.Element{t.Rho}

	for _, c := range p.GeminiFoldComms {
		gR = append(gR, PointLimbs(c)...)
	}

	t.GeminiR = Poseidon2Hash(gR)

	t.ShplonkNu = Poseidon2Hash(append([]fr.Element{t.GeminiR}, p.GeminiAEvaluations[:]...))

	t.ShplonkZ = Poseidon2Hash(append([]fr.Element{t.ShplonkNu}, PointLimbs(p.ShplonkQ)...))

	return t
}
