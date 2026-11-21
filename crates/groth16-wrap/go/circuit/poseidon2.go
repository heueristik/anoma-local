// Derived from Barretenberg (https://github.com/AztecProtocol/aztec-packages, Copyright 2022 Aztec, Apache-2.0):
// the Poseidon2 permutation and sponge of Barretenberg's crypto library. Changed: expressed as a gnark circuit.

package circuit

import (
	"math/big"

	"github.com/consensys/gnark/frontend"

	"github.com/wstran/ultrahonk-gnark/honk"
)

var p2Diag, p2External, p2Internal = honk.Poseidon2Constants()

func sbox(api frontend.API, x frontend.Variable) frontend.Variable {
	x2 := api.Mul(x, x)

	x4 := api.Mul(x2, x2)

	return api.Mul(x4, x)
}

func matmulM4(api frontend.API, s *[4]frontend.Variable) {
	t0 := api.Add(s[0], s[1])

	t1 := api.Add(s[2], s[3])

	t2 := api.Add(api.Mul(s[1], 2), t1)

	t3 := api.Add(api.Mul(s[3], 2), t0)

	t4 := api.Add(api.Mul(t1, 4), t3)

	t5 := api.Add(api.Mul(t0, 4), t2)

	t6 := api.Add(t3, t5)

	t7 := api.Add(t2, t4)

	s[0], s[1], s[2], s[3] = t6, t5, t7, t4
}

func matmulInternal(api frontend.API, s *[4]frontend.Variable) {
	sum := api.Add(s[0], s[1], s[2], s[3])

	for i := range s {
		s[i] = api.Add(api.Mul(s[i], p2Diag[i]), sum)
	}
}

func poseidon2Permute(api frontend.API, state [4]frontend.Variable) [4]frontend.Variable {
	s := state

	matmulM4(api, &s)

	external := func(r int) {
		for i := range s {
			s[i] = sbox(api, api.Add(s[i], p2External[r][i]))
		}

		matmulM4(api, &s)
	}

	for r := 0; r < 4; r++ {
		external(r)
	}

	for r := 0; r < 56; r++ {
		s[0] = sbox(api, api.Add(s[0], p2Internal[r]))

		matmulInternal(api, &s)
	}

	for r := 4; r < 8; r++ {
		external(r)
	}

	return s
}

// poseidon2Hash mirrors honk.Poseidon2Hash: fixed-length sponge, rate 3, IV = len << 64, one output.
func poseidon2Hash(api frontend.API, inputs []frontend.Variable) frontend.Variable {
	iv := new(big.Int).Lsh(big.NewInt(int64(len(inputs))), 64)

	state := [4]frontend.Variable{0, 0, 0, iv}

	cache := []frontend.Variable{}

	duplex := func() {
		for i := 0; i < 3; i++ {
			if i < len(cache) {
				state[i] = api.Add(state[i], cache[i])
			}
		}

		state = poseidon2Permute(api, state)
	}

	for _, in := range inputs {
		if len(cache) == 3 {
			duplex()

			cache = []frontend.Variable{in}
		} else {
			cache = append(cache, in)
		}
	}

	duplex()

	return state[0]
}
