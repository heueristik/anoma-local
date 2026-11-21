package honk

import (
	"errors"
	"fmt"

	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
)

// VerificationKey is bb's UltraFlavor verification key.
type VerificationKey struct {
	LogCircuitSize   uint64
	PublicInputsSize uint64
	PubInputsOffset  uint64
	Commitments      [NumVKCommitments]bn254.G1Affine
	Hash             fr.Element
}

// ParseVK reads the `vk` file of `bb write_vk -t noir-recursive-no-zk`: log circuit size, number of public
// inputs (including the 8 pairing point limbs), public inputs offset, then 28 commitments of 4 limbs each.
// Hash is bb's vk hash, the Poseidon2 sponge of all 115 fields.
func ParseVK(data []byte) (*VerificationKey, error) {
	fields, err := FieldsFromBytes(data)

	if err != nil {
		return nil, err
	}

	return ParseVKFields(fields)
}

// ParseVKFields is ParseVK on field elements.
func ParseVKFields(fields []fr.Element) (*VerificationKey, error) {
	if len(fields) != VKFieldCount {
		return nil, fmt.Errorf("honk: vk has %d fields, expected %d", len(fields), VKFieldCount)
	}

	var header [3]uint64

	for i := range header {
		v := bigOf(fields[i])

		if !v.IsUint64() {
			return nil, errors.New("honk: vk header too large")
		}

		header[i] = v.Uint64()
	}

	vk := &VerificationKey{LogCircuitSize: header[0], PublicInputsSize: header[1], PubInputsOffset: header[2]}

	if vk.LogCircuitSize < 1 || vk.LogCircuitSize > ConstProofSizeLogN || vk.PublicInputsSize < PairingPointsSize {
		return nil, errors.New("honk: vk header out of range")
	}

	// The public inputs occupy rows offset .. offset + size - 1 of the 2^logN trace.
	if vk.PubInputsOffset+vk.PublicInputsSize > uint64(1)<<vk.LogCircuitSize {
		return nil, errors.New("honk: vk public inputs exceed the circuit size")
	}

	r := &fieldReader{data: fields, pos: 3}

	for i := range vk.Commitments {
		p, err := r.point()

		if err != nil {
			return nil, fmt.Errorf("honk: vk commitment %d: %w", i, err)
		}

		vk.Commitments[i] = p
	}

	vk.Hash = Poseidon2Hash(fields)

	return vk, nil
}

// NumUserPublicInputs is the number of public inputs besides the 8 pairing point limbs.
func (vk *VerificationKey) NumUserPublicInputs() int {
	return int(vk.PublicInputsSize - PairingPointsSize)
}
