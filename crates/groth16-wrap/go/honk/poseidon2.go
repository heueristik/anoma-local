// Derived from Barretenberg (https://github.com/AztecProtocol/aztec-packages, Copyright 2022 Aztec, Apache-2.0):
// the Poseidon2 permutation and sponge of Barretenberg's crypto library. Changed: translated to Go.

package honk

import "github.com/consensys/gnark-crypto/ecc/bn254/fr"

type poseidon2Params struct {
	diagM1   [4]fr.Element
	external [8][4]fr.Element
	internal [56]fr.Element
}

var p2 = func() poseidon2Params {
	var p poseidon2Params

	for i, h := range poseidon2DiagM1 {
		p.diagM1[i] = frFromHex(h)
	}

	for r := range poseidon2External {
		for i, h := range poseidon2External[r] {
			p.external[r][i] = frFromHex(h)
		}
	}

	for i, h := range poseidon2Internal {
		p.internal[i] = frFromHex(h)
	}

	return p
}()

func matmulM4(s *[4]fr.Element) {
	t0 := add(s[0], s[1])

	t1 := add(s[2], s[3])

	t2 := add(add(s[1], s[1]), t1)

	t3 := add(add(s[3], s[3]), t0)

	t4 := add(add(add(t1, t1), add(t1, t1)), t3)

	t5 := add(add(add(t0, t0), add(t0, t0)), t2)

	t6 := add(t3, t5)

	t7 := add(t2, t4)

	s[0], s[1], s[2], s[3] = t6, t5, t7, t4
}

func matmulInternal(s *[4]fr.Element) {
	sum := add(add(add(s[0], s[1]), s[2]), s[3])

	for i := range s {
		s[i] = add(mul(s[i], p2.diagM1[i]), sum)
	}
}

// Poseidon2Permutation is the Barretenberg BN254 Poseidon2 permutation (width 4, 8 full and 56 partial rounds).
func Poseidon2Permutation(state [4]fr.Element) [4]fr.Element {
	s := state

	matmulM4(&s)

	external := func(r int) {
		for i := range s {
			s[i] = pow5(add(s[i], p2.external[r][i]))
		}

		matmulM4(&s)
	}

	for r := 0; r < 4; r++ {
		external(r)
	}

	for r := 0; r < 56; r++ {
		s[0] = pow5(add(s[0], p2.internal[r]))

		matmulInternal(&s)
	}

	for r := 4; r < 8; r++ {
		external(r)
	}

	return s
}

// Poseidon2Hash is bb's fixed-length sponge (rate 3, capacity 1, one output) with IV = len << 64.
func Poseidon2Hash(inputs []fr.Element) fr.Element {
	var state [4]fr.Element

	var shift fr.Element

	shift.SetUint64(1 << 32)

	shift.Square(&shift)

	state[3] = mul(frU(uint64(len(inputs))), shift)

	var cache [3]fr.Element

	cacheSize := 0

	duplex := func() {
		for i := cacheSize; i < 3; i++ {
			cache[i] = fr.Element{}
		}

		for i := 0; i < 3; i++ {
			state[i] = add(state[i], cache[i])
		}

		state = Poseidon2Permutation(state)
	}

	for _, in := range inputs {
		if cacheSize == 3 {
			duplex()

			cache[0] = in

			cacheSize = 1
		} else {
			cache[cacheSize] = in

			cacheSize++
		}
	}

	duplex()

	return state[0]
}
