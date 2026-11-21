package circuit

import (
	"crypto/sha256"
	"fmt"
	"math/big"
	"os"
	"path/filepath"
	"runtime"
	"testing"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
	"github.com/consensys/gnark/constraint"
	"github.com/consensys/gnark/constraint/solver"
	"github.com/consensys/gnark/frontend"
	fcs "github.com/consensys/gnark/frontend/cs"
	"github.com/consensys/gnark/frontend/cs/r1cs"

	"github.com/wstran/ultrahonk-gnark/honk"
)

var names = []string{"keccak_8", "ecc_1", "mixed", "recurse_1", "pub_3"}

type vector struct {
	name  string
	vk    *honk.VerificationKey
	proof []fr.Element
	pi    []fr.Element
}

func load(t *testing.T, name string) vector {
	t.Helper()

	fields := func(file string) []fr.Element {
		b, err := os.ReadFile(filepath.Join("..", "testdata", name, file))

		if err != nil {
			t.Fatal(err)
		}

		f, err := honk.FieldsFromBytes(b)

		if err != nil {
			t.Fatal(err)
		}

		return f
	}

	vk, err := honk.ParseVKFields(fields("vk"))

	if err != nil {
		t.Fatal(err)
	}

	return vector{name: name, vk: vk, proof: fields("proof"), pi: fields("public_inputs")}
}

// commitmentOverride replaces gnark's BSB22 commitment placeholder by a hash of the committed values, as the gnark
// test engine does, so satisfiability can be checked without a proving key.
var commitmentOverride = solver.OverrideHint(solver.GetHintID(fcs.Bsb22CommitmentComputePlaceholder), func(mod *big.Int, in []*big.Int, out []*big.Int) error {
	h := sha256.New()

	for _, v := range in {
		b := make([]byte, 32)

		v.FillBytes(b)

		h.Write(b)
	}

	out[0].SetBytes(h.Sum(nil))

	out[0].Mod(out[0], mod)

	return nil
})

func compile(t *testing.T, vk *honk.VerificationKey) constraint.ConstraintSystem {
	t.Helper()

	runtime.GC()

	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, New(vk))

	if err != nil {
		t.Fatal(err)
	}

	return ccs
}

func solves(t *testing.T, ccs constraint.ConstraintSystem, vk *honk.VerificationKey, proof, pi []fr.Element) bool {
	t.Helper()

	a, err := Assign(vk, proof, pi)

	if err != nil {
		t.Fatal(err)
	}

	w, err := frontend.NewWitness(a, ecc.BN254.ScalarField())

	if err != nil {
		t.Fatal(err)
	}

	return ccs.IsSolved(w, commitmentOverride) == nil
}

func flip(src []fr.Element, i int) []fr.Element {
	out := append([]fr.Element{}, src...)

	b := out[i].Bytes()

	b[31] ^= 0x01

	out[i].SetBytes(b[:])

	return out
}

// check requires the circuit verdict to equal the native verdict.
func check(t *testing.T, ccs constraint.ConstraintSystem, v vector, label string, proof, pi []fr.Element) {
	t.Helper()

	want := false

	if pr, err := honk.ParseProofFields(proof); err == nil {
		want = honk.Verify(v.vk, pr, pi) == nil
	}

	if got := solves(t, ccs, v.vk, proof, pi); got != want {
		t.Errorf("%s %s: circuit %v, native %v", v.name, label, got, want)
	}
}

func TestSolvesTestdata(t *testing.T) {
	if testing.Short() {
		t.Skip("compiles a 3.6M-constraint circuit per vector")
	}

	for _, name := range names {
		v := load(t, name)

		ccs := compile(t, v.vk)

		t.Logf("%s: %d R1CS constraints", name, ccs.GetNbConstraints())

		if !solves(t, ccs, v.vk, v.proof, v.pi) {
			t.Errorf("%s: valid proof not accepted", name)
		}

		for _, w := range []int{3, 100, 300, honk.ProofFieldCount - 1} {
			check(t, ccs, v, fmt.Sprintf("proof field %d", w), flip(v.proof, w), v.pi)
		}

		check(t, ccs, v, "public input", v.proof, flip(v.pi, 0))
	}
}

// TestEveryProofField flips every proof field and every public input and tries every other vector's proof; set
// ULTRAHONK_LONG=1 to run it.
func TestEveryProofField(t *testing.T) {
	if os.Getenv("ULTRAHONK_LONG") != "1" {
		t.Skip("set ULTRAHONK_LONG=1")
	}

	vecs := make([]vector, len(names))

	for i, n := range names {
		vecs[i] = load(t, n)
	}

	for _, v := range vecs {
		ccs := compile(t, v.vk)

		check(t, ccs, v, "original", v.proof, v.pi)

		for w := 0; w < honk.ProofFieldCount; w++ {
			check(t, ccs, v, fmt.Sprintf("proof field %d", w), flip(v.proof, w), v.pi)
		}

		for w := range v.pi {
			check(t, ccs, v, fmt.Sprintf("public input %d", w), v.proof, flip(v.pi, w))
		}

		for _, o := range vecs {
			if o.name != v.name && len(o.pi) == len(v.pi) {
				check(t, ccs, v, "proof of "+o.name, o.proof, o.pi)
			}
		}
	}
}
