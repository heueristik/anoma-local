// Package honk is a native verifier for Barretenberg v6.0.0 UltraHonk proofs of the UltraFlavor: Poseidon2
// transcript, no zero knowledge, padded to 25 rounds, as produced by `bb prove -t noir-recursive-no-zk`.
// It follows the verification algorithm of the Solidity verifier that bb v6.0.0 generates, with the Keccak
// transcript replaced by bb's Poseidon2 sponge, and it is the reference for the gnark circuit in package circuit.
package honk

// BBVersion is the Barretenberg release whose proof and verification key format this package accepts.
const BBVersion = "6.0.0"

const (
	NumberOfSubrelations              = 31
	NumberOfAlphas                    = NumberOfSubrelations - 1
	BatchedRelationPartialLength      = 8
	NumberOfEntities                  = 41
	NumberUnshifted                   = 36
	NumberToBeShifted                 = 5
	ShiftedCommitmentsStart           = 29
	PairingPointsSize                 = 8
	NumVKCommitments                  = 28
	ConstProofSizeLogN                = 25
	PermutationArgumentValueSeparator = 1 << 28
	VKFieldCount                      = 3 + NumVKCommitments*4
	ProofFieldCount                   = PairingPointsSize + 8*4 + ConstProofSizeLogN*BatchedRelationPartialLength + NumberOfEntities + (ConstProofSizeLogN-1)*4 + ConstProofSizeLogN + 2*4
)

// Entity indices in the order of the sumcheck evaluations and of the verification key commitments.
const (
	Sigma1 = iota
	Sigma2
	Sigma3
	Sigma4
	Id1
	Id2
	Id3
	Id4
	LagrangeFirst
	LagrangeLast
	QLookup
	Table1
	Table2
	Table3
	Table4
	QM
	QR
	QO
	QC
	QL
	Q4
	QArith
	QRange
	QElliptic
	QMemory
	QNnf
	QPoseidon2External
	QPoseidon2Internal
	WL
	WR
	WO
	W4
	ZPerm
	LookupInverses
	LookupReadCounts
	LookupReadTags
	WLShift
	WRShift
	WOShift
	W4Shift
	ZPermShift
)
