package wrap

import (
	"bytes"
	"errors"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend"
	"github.com/consensys/gnark/backend/groth16"
	groth16bn254 "github.com/consensys/gnark/backend/groth16/bn254"
	"github.com/consensys/gnark/backend/solidity"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
)

// squareCircuit proves knowledge of a square root; with commit set, it also makes one BSB22 commitment.
type squareCircuit struct {
	Root   frontend.Variable
	Square frontend.Variable `gnark:",public"`

	commit bool
}

func (c *squareCircuit) Define(api frontend.API) error {
	api.AssertIsEqual(api.Mul(c.Root, c.Root), c.Square)

	if !c.commit {
		return nil
	}

	committer, ok := api.(frontend.Committer)
	if !ok {
		return errors.New("the builder does not make commitments")
	}

	_, err := committer.Commit(c.Root)

	return err
}

func proveSquare(t *testing.T, commit bool) (*groth16bn254.Proof, *groth16bn254.VerifyingKey) {
	t.Helper()

	constraintSystem, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &squareCircuit{commit: commit})
	if err != nil {
		t.Fatal(err)
	}

	provingKey, verifyingKey, err := groth16.Setup(constraintSystem)
	if err != nil {
		t.Fatal(err)
	}

	fullWitness, err := frontend.NewWitness(&squareCircuit{Root: 3, Square: 9}, ecc.BN254.ScalarField())
	if err != nil {
		t.Fatal(err)
	}

	proof, err := groth16.Prove(constraintSystem, provingKey, fullWitness, solidity.WithProverTargetSolidityVerifier(backend.GROTH16))
	if err != nil {
		t.Fatal(err)
	}

	return proof.(*groth16bn254.Proof), verifyingKey.(*groth16bn254.VerifyingKey)
}

func TestDecodeProofReadsTheLayoutOfMarshalSolidity(t *testing.T) {
	for _, commit := range []bool{false, true} {
		proof, verifyingKey := proveSquare(t, commit)

		if commitments := len(verifyingKey.CommitmentKeys); (commitments > 0) != commit {
			t.Fatalf("commit %v: the circuit has %d commitments", commit, commitments)
		}

		decoded, err := decodeProof(verifyingKey, proof.MarshalSolidity())
		if err != nil {
			t.Fatalf("commit %v: %v", commit, err)
		}

		var want, got bytes.Buffer

		if _, err := proof.WriteRawTo(&want); err != nil {
			t.Fatal(err)
		}

		if _, err := decoded.WriteRawTo(&got); err != nil {
			t.Fatal(err)
		}

		if !bytes.Equal(got.Bytes(), want.Bytes()) {
			t.Errorf("commit %v: the decoded proof differs from the proof", commit)
		}
	}
}

func TestDecodeProofRejectsAProofOfTheWrongLength(t *testing.T) {
	proof, verifyingKey := proveSquare(t, true)

	_, err := decodeProof(verifyingKey, proof.MarshalSolidity()[1:])

	var rejection *Rejection
	if !errors.As(err, &rejection) {
		t.Fatalf("got %v, expected a rejection", err)
	}
}
