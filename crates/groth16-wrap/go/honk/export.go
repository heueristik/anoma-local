package honk

import (
	"math/big"

	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
)

// Poseidon2Constants returns the internal diagonal minus one, the external and the internal round constants.
func Poseidon2Constants() ([4]*big.Int, [8][4]*big.Int, [56]*big.Int) {
	var diag [4]*big.Int

	var ext [8][4]*big.Int

	var internal [56]*big.Int

	for i := range diag {
		diag[i] = bigOf(p2.diagM1[i])
	}

	for r := range ext {
		for i := range ext[r] {
			ext[r][i] = bigOf(p2.external[r][i])
		}
	}

	for i := range internal {
		internal[i] = bigOf(p2.internal[i])
	}

	return diag, ext, internal
}

// PairingG2 returns the G2 generator and the SRS point [x]_2 of the final KZG pairing.
func PairingG2() (bn254.G2Affine, bn254.G2Affine) {
	return g2Generator, g2X
}

// BarycentricDenominators returns prod_{j != i} (i - j) over the domain {0, ..., 7}.
func BarycentricDenominators() [BatchedRelationPartialLength]fr.Element {
	return barycentricDenominators
}

// RelationConstants returns -1/2, 2^68, 2^14, the internal Poseidon2 diagonal and 17 (minus the Grumpkin b).
func RelationConstants() (negHalfOut, limbSizeOut, sublimbOut fr.Element, diag [4]fr.Element, grumpkinBNeg fr.Element) {
	return negHalf, limbSize, sublimbShift, internalMatrixDiagonal, grumpkinBNegated
}
