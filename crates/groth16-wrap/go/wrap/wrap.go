// Package wrap proves with Groth16 over BN254 that a bb 6.0.0 UltraHonk proof (noir-recursive-no-zk) verifies
// against one inner verifying key. A keys directory holds the keys of one inner circuit.
package wrap

import (
	"bufio"
	"bytes"
	"encoding/binary"
	"errors"
	"fmt"
	"io"
	"os"
	"path/filepath"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	"github.com/consensys/gnark/backend"
	"github.com/consensys/gnark/backend/groth16"
	groth16bn254 "github.com/consensys/gnark/backend/groth16/bn254"
	"github.com/consensys/gnark/backend/solidity"
	"github.com/consensys/gnark/constraint"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"

	"github.com/wstran/ultrahonk-gnark/circuit"
	"github.com/wstran/ultrahonk-gnark/honk"
)

// The files of a keys directory.
const (
	innerVerifyingKeyFile = "inner_vk"
	constraintSystemFile  = "circuit.r1cs"
	provingKeyFile        = "pk.bin"
	verifyingKeyFile      = "vk.bin"
)

// fieldBytes is the length of one encoded field element: 32 bytes, big-endian.
const fieldBytes = fr.Bytes

// Rejection is an error that shows an invalid proof or invalid public inputs, not a failed computation.
type Rejection struct {
	Reason error
}

func (r *Rejection) Error() string {
	return r.Reason.Error()
}

func (r *Rejection) Unwrap() error {
	return r.Reason
}

func reject(format string, arguments ...any) error {
	return &Rejection{Reason: fmt.Errorf(format, arguments...)}
}

// SinglePartySetup compiles the wrapper circuit for innerVerifyingKey and writes its keys to directory.
// Whoever runs it knows the setup secret and can make false proofs, so its keys are for tests only.
func SinglePartySetup(innerVerifyingKey []byte, directory string) error {
	innerKey, err := honk.ParseVK(innerVerifyingKey)
	if err != nil {
		return fmt.Errorf("parse the inner verifying key: %w", err)
	}

	if innerKey.NumUserPublicInputs() == 0 {
		return errors.New("the inner circuit has no public inputs, and the Solidity verifier of gnark needs at least one")
	}

	if err := os.MkdirAll(directory, 0o755); err != nil {
		return err
	}

	constraintSystem, err := compile(innerKey)
	if err != nil {
		return fmt.Errorf("compile the wrapper circuit: %w", err)
	}

	provingKey, verifyingKey, err := groth16.Setup(constraintSystem)
	if err != nil {
		return fmt.Errorf("set up the Groth16 keys: %w", err)
	}

	if err := writeFile(directory, constraintSystemFile, constraintSystem.WriteTo); err != nil {
		return err
	}

	if err := writeFile(directory, provingKeyFile, provingKey.WriteRawTo); err != nil {
		return err
	}

	if err := writeFile(directory, verifyingKeyFile, verifyingKey.WriteTo); err != nil {
		return err
	}

	// The inner verifying key is written last, so a directory with it holds all keys.
	return writeFile(directory, innerVerifyingKeyFile, bytes.NewReader(innerVerifyingKey).WriteTo)
}

// Prove wraps an inner proof and its user public inputs, in the files that bb writes, into a Groth16 proof in the
// calldata layout of the Solidity verifier. It also returns the Groth16 public inputs, 32 big-endian bytes each.
func Prove(directory string, innerProof, publicInputs []byte) ([]byte, []byte, error) {
	innerKey, err := readInnerVerifyingKey(directory)
	if err != nil {
		return nil, nil, err
	}

	inputs, err := parsePublicInputs(innerKey, publicInputs)
	if err != nil {
		return nil, nil, err
	}

	proofFields, err := honk.FieldsFromBytes(innerProof)
	if err != nil {
		return nil, nil, reject("decode the inner proof: %w", err)
	}

	parsedProof, err := honk.ParseProofFields(proofFields)
	if err != nil {
		return nil, nil, reject("decode the inner proof: %w", err)
	}

	if err := honk.Verify(innerKey, parsedProof, inputs); err != nil {
		return nil, nil, reject("the inner proof does not verify: %w", err)
	}

	constraintSystem := groth16.NewCS(ecc.BN254)
	if err := readFile(directory, constraintSystemFile, constraintSystem.ReadFrom); err != nil {
		return nil, nil, err
	}

	provingKey := groth16.NewProvingKey(ecc.BN254)
	if err := readFile(directory, provingKeyFile, provingKey.UnsafeReadFrom); err != nil {
		return nil, nil, err
	}

	verifyingKey, err := readVerifyingKey(directory)
	if err != nil {
		return nil, nil, err
	}

	assignment, err := circuit.Assign(innerKey, proofFields, inputs)
	if err != nil {
		return nil, nil, err
	}

	fullWitness, err := frontend.NewWitness(assignment, ecc.BN254.ScalarField())
	if err != nil {
		return nil, nil, err
	}

	proof, err := groth16.Prove(constraintSystem, provingKey, fullWitness, solidity.WithProverTargetSolidityVerifier(backend.GROTH16))
	if err != nil {
		return nil, nil, fmt.Errorf("make the Groth16 proof: %w", err)
	}

	publicWitness, err := fullWitness.Public()
	if err != nil {
		return nil, nil, err
	}

	if err := groth16.Verify(proof, verifyingKey, publicWitness, solidity.WithVerifierTargetSolidityVerifier(backend.GROTH16)); err != nil {
		return nil, nil, fmt.Errorf("the new Groth16 proof does not verify: %w", err)
	}

	proofBn254, ok := proof.(*groth16bn254.Proof)
	if !ok {
		return nil, nil, errors.New("gnark returned a Groth16 proof that is not on BN254")
	}

	publicValues, ok := publicWitness.Vector().(fr.Vector)
	if !ok {
		return nil, nil, errors.New("gnark returned public inputs that are not BN254 scalars")
	}

	encodedInputs := make([]byte, 0, len(publicValues)*fieldBytes)
	for _, value := range publicValues {
		encoded := value.Bytes()
		encodedInputs = append(encodedInputs, encoded[:]...)
	}

	return proofBn254.MarshalSolidity(), encodedInputs, nil
}

// Verify checks a Groth16 proof in the calldata layout of the Solidity verifier against its public inputs.
func Verify(directory string, proof, publicInputs []byte) error {
	innerKey, err := readInnerVerifyingKey(directory)
	if err != nil {
		return err
	}

	inputs, err := parsePublicInputs(innerKey, publicInputs)
	if err != nil {
		return err
	}

	verifyingKey, err := readVerifyingKey(directory)
	if err != nil {
		return err
	}

	decodedProof, err := decodeProof(verifyingKey, proof)
	if err != nil {
		return err
	}

	assignment := circuit.New(innerKey)
	for i := range assignment.PublicInputs {
		assignment.PublicInputs[i] = inputs[i]
	}

	publicWitness, err := frontend.NewWitness(assignment, ecc.BN254.ScalarField(), frontend.PublicOnly())
	if err != nil {
		return err
	}

	if err := groth16.Verify(decodedProof, verifyingKey, publicWitness, solidity.WithVerifierTargetSolidityVerifier(backend.GROTH16)); err != nil {
		return &Rejection{Reason: err}
	}

	return nil
}

// SolidityVerifier returns the source of the Solidity verifier for the keys in directory.
func SolidityVerifier(directory string) ([]byte, error) {
	verifyingKey, err := readVerifyingKey(directory)
	if err != nil {
		return nil, err
	}

	var source bytes.Buffer
	if err := verifyingKey.ExportSolidity(&source); err != nil {
		return nil, fmt.Errorf("export the Solidity verifier: %w", err)
	}

	return source.Bytes(), nil
}

// decodeProof reads A | B | C | commitments | proof of knowledge, the layout of MarshalSolidity. That layout is the
// gnark raw layout without the 4-byte commitment count, and without the proof of knowledge if there is no commitment.
func decodeProof(verifyingKey *groth16bn254.VerifyingKey, data []byte) (*groth16bn254.Proof, error) {
	const pointsLength = 256

	const pointLength = 64

	commitments := len(verifyingKey.CommitmentKeys)

	expectedLength := pointsLength
	if commitments > 0 {
		expectedLength += (commitments + 1) * pointLength
	}

	if len(data) != expectedLength {
		return nil, reject("the Groth16 proof has %d bytes, expected %d", len(data), expectedLength)
	}

	raw := make([]byte, 0, expectedLength+4+pointLength)
	raw = append(raw, data[:pointsLength]...)
	raw = binary.BigEndian.AppendUint32(raw, uint32(commitments))
	raw = append(raw, data[pointsLength:]...)

	if commitments == 0 {
		// The raw encoding of the identity is all zero bytes.
		raw = append(raw, make([]byte, pointLength)...)
	}

	proof := new(groth16bn254.Proof)
	if _, err := proof.ReadFrom(bytes.NewReader(raw)); err != nil {
		return nil, reject("decode the Groth16 proof: %w", err)
	}

	return proof, nil
}

func parsePublicInputs(innerKey *honk.VerificationKey, data []byte) ([]fr.Element, error) {
	inputs, err := honk.FieldsFromBytes(data)
	if err != nil {
		return nil, reject("decode the public inputs: %w", err)
	}

	if len(inputs) != innerKey.NumUserPublicInputs() {
		return nil, reject("got %d public inputs, expected %d", len(inputs), innerKey.NumUserPublicInputs())
	}

	return inputs, nil
}

func compile(innerKey *honk.VerificationKey) (constraint.ConstraintSystem, error) {
	return frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, circuit.New(innerKey))
}

func readInnerVerifyingKey(directory string) (*honk.VerificationKey, error) {
	data, err := os.ReadFile(filepath.Join(directory, innerVerifyingKeyFile))
	if err != nil {
		return nil, err
	}

	innerKey, err := honk.ParseVK(data)
	if err != nil {
		return nil, fmt.Errorf("parse %s: %w", filepath.Join(directory, innerVerifyingKeyFile), err)
	}

	return innerKey, nil
}

func readVerifyingKey(directory string) (*groth16bn254.VerifyingKey, error) {
	verifyingKey := new(groth16bn254.VerifyingKey)
	if err := readFile(directory, verifyingKeyFile, verifyingKey.ReadFrom); err != nil {
		return nil, err
	}

	return verifyingKey, nil
}

func writeFile(directory, name string, write func(io.Writer) (int64, error)) error {
	path := filepath.Join(directory, name)

	file, err := os.Create(path)
	if err != nil {
		return err
	}

	buffered := bufio.NewWriterSize(file, 1<<20)

	_, err = write(buffered)
	if err == nil {
		err = buffered.Flush()
	}

	if closeErr := file.Close(); err == nil {
		err = closeErr
	}

	if err != nil {
		return fmt.Errorf("write %s: %w", path, err)
	}

	return nil
}

func readFile(directory, name string, read func(io.Reader) (int64, error)) error {
	path := filepath.Join(directory, name)

	file, err := os.Open(path)
	if err != nil {
		return err
	}

	defer file.Close()

	if _, err := read(bufio.NewReaderSize(file, 1<<20)); err != nil {
		return fmt.Errorf("read %s: %w", path, err)
	}

	return nil
}
