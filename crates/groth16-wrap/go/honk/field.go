package honk

import (
	"errors"
	"math/big"

	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
)

var one = frU(1)

func frU(v uint64) fr.Element {
	var e fr.Element

	e.SetUint64(v)

	return e
}

func frFromHex(s string) fr.Element {
	v, ok := new(big.Int).SetString(s, 16)

	if !ok {
		panic("honk: bad hex constant")
	}

	var e fr.Element

	e.SetBigInt(v)

	return e
}

func bigOf(e fr.Element) *big.Int {
	return e.BigInt(new(big.Int))
}

func add(a, b fr.Element) fr.Element {
	var c fr.Element

	c.Add(&a, &b)

	return c
}

func sub(a, b fr.Element) fr.Element {
	var c fr.Element

	c.Sub(&a, &b)

	return c
}

func mul(a, b fr.Element) fr.Element {
	var c fr.Element

	c.Mul(&a, &b)

	return c
}

func mul3(a, b, c fr.Element) fr.Element {
	return mul(mul(a, b), c)
}

func pow5(a fr.Element) fr.Element {
	a2 := mul(a, a)

	return mul(mul(a2, a2), a)
}

var errInvertZero = errors.New("honk: inverse of zero")

func inv(a fr.Element) (fr.Element, error) {
	if a.IsZero() {
		return a, errInvertZero
	}

	var b fr.Element

	b.Inverse(&a)

	return b, nil
}

// FieldsFromBytes reads 32-byte big-endian field elements and rejects non-canonical encodings (>= r).
func FieldsFromBytes(data []byte) ([]fr.Element, error) {
	if len(data)%32 != 0 {
		return nil, errors.New("honk: length is not a multiple of 32")
	}

	out := make([]fr.Element, len(data)/32)

	for i := range out {
		if err := out[i].SetBytesCanonical(data[i*32 : (i+1)*32]); err != nil {
			return nil, err
		}
	}

	return out, nil
}
