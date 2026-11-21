package honk

import (
	"bytes"
	"os"
	"path/filepath"
	"testing"
)

var vectors = []string{"keccak_8", "ecc_1", "mixed", "recurse_1", "pub_3"}

type vector struct {
	vk, vkHash, proof, pi []byte
}

func load(t *testing.T, name string) vector {
	t.Helper()

	read := func(file string) []byte {
		b, err := os.ReadFile(filepath.Join("..", "testdata", name, file))

		if err != nil {
			t.Fatal(err)
		}

		return b
	}

	return vector{vk: read("vk"), vkHash: read("vk_hash"), proof: read("proof"), pi: read("public_inputs")}
}

func flip(data []byte, word int) []byte {
	out := append([]byte{}, data...)

	out[word*32+31] ^= 0x01

	return out
}

func TestVerifiesTestdata(t *testing.T) {
	for _, name := range vectors {
		v := load(t, name)

		if err := VerifyFiles(v.vk, v.proof, v.pi); err != nil {
			t.Errorf("%s: %v", name, err)
		}
	}
}

func TestVKHashMatchesBB(t *testing.T) {
	for _, name := range vectors {
		v := load(t, name)

		vk, err := ParseVK(v.vk)

		if err != nil {
			t.Fatal(err)
		}

		h := vk.Hash.Bytes()

		if !bytes.Equal(h[:], v.vkHash) {
			t.Errorf("%s: vk hash differs from bb's vk_hash", name)
		}
	}
}

func TestRejectsTamperedFields(t *testing.T) {
	stride := 1

	if testing.Short() {
		stride = 17
	}

	for _, name := range vectors {
		v := load(t, name)

		for w := 0; w < len(v.proof)/32; w += stride {
			if VerifyFiles(v.vk, flip(v.proof, w), v.pi) == nil {
				t.Errorf("%s: proof with word %d flipped accepted", name, w)
			}
		}

		for w := 0; w < len(v.pi)/32; w++ {
			if VerifyFiles(v.vk, v.proof, flip(v.pi, w)) == nil {
				t.Errorf("%s: public input %d flipped accepted", name, w)
			}
		}

		for w := 0; w < len(v.vk)/32; w += stride {
			if VerifyFiles(flip(v.vk, w), v.proof, v.pi) == nil {
				t.Errorf("%s: vk with word %d flipped accepted", name, w)
			}
		}
	}
}

func TestRejectsProofForAnotherVK(t *testing.T) {
	for _, a := range vectors {
		for _, b := range vectors {
			if a == b {
				continue
			}

			va, vb := load(t, a), load(t, b)

			if VerifyFiles(va.vk, vb.proof, vb.pi) == nil {
				t.Errorf("proof of %s accepted under the vk of %s", b, a)
			}
		}
	}
}

func TestRejectsMalformedEncodings(t *testing.T) {
	v := load(t, "recurse_1")

	cases := map[string][]byte{
		"short proof":    v.proof[:len(v.proof)-32],
		"trailing field": append(append([]byte{}, v.proof...), make([]byte, 32)...),
		"non-canonical": func() []byte {
			b := append([]byte{}, v.proof...)
			copy(b[64:96], bytes.Repeat([]byte{0xff}, 32))
			return b
		}(),
		"limb too large": func() []byte { b := append([]byte{}, v.proof...); b[32+14] = 0x01; return b }(),
	}

	for label, proof := range cases {
		if VerifyFiles(v.vk, proof, v.pi) == nil {
			t.Errorf("%s accepted", label)
		}
	}

	if VerifyFiles(v.vk, v.proof, append(append([]byte{}, v.pi...), v.pi...)) == nil {
		t.Error("extra public input accepted")
	}
}

// TestSelectorCoverage checks that every gate selector is a non-identity commitment in at least one test vector,
// so each relation is exercised by the differential tests.
func TestSelectorCoverage(t *testing.T) {
	selectors := map[string]int{"lookup": QLookup, "arithmetic": QArith, "delta range": QRange, "elliptic": QElliptic, "memory": QMemory, "non-native field": QNnf, "poseidon2 external": QPoseidon2External, "poseidon2 internal": QPoseidon2Internal}

	for label, index := range selectors {
		covered := false

		for _, name := range vectors {
			vk, err := ParseVK(load(t, name).vk)

			if err != nil {
				t.Fatal(err)
			}

			if !vk.Commitments[index].IsInfinity() {
				covered = true
			}
		}

		if !covered {
			t.Errorf("no test vector uses %s gates", label)
		}
	}

	recursive, err := ParseProof(load(t, "recurse_1").proof)

	if err != nil {
		t.Fatal(err)
	}

	isDefault := true

	for _, limb := range recursive.PairingPointObject {
		if !limb.IsZero() {
			isDefault = false
		}
	}

	if isDefault {
		t.Error("recurse_1 should carry non-default pairing points")
	}
}
