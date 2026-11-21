// Derived from Barretenberg (https://github.com/AztecProtocol/aztec-packages, Copyright 2022 Aztec, Apache-2.0):
// the proof and verification key field encoding of bb v6.0.0. Changed: translated to Go.

package honk

import (
	"errors"
	"math/big"

	"github.com/consensys/gnark-crypto/ecc/bn254"
	"github.com/consensys/gnark-crypto/ecc/bn254/fp"
	"github.com/consensys/gnark-crypto/ecc/bn254/fr"
)

// Commitments travel as four field elements [x_lo, x_hi, y_lo, y_hi] with 136-bit low limbs.
const limbBits = 136

var (
	two136 = new(big.Int).Lsh(big.NewInt(1), limbBits)
	two118 = new(big.Int).Lsh(big.NewInt(1), 254-limbBits)
)

var errShort = errors.New("honk: input too short")

type fieldReader struct {
	data []fr.Element
	pos  int
}

func (r *fieldReader) next() (fr.Element, error) {
	if r.pos >= len(r.data) {
		return fr.Element{}, errShort
	}

	e := r.data[r.pos]

	r.pos++

	return e, nil
}

// coordinate reads (lo, hi) and requires lo < 2^136, hi < 2^118 and lo + hi * 2^136 < q, so every
// coordinate has exactly one encoding and the transcript binds the point.
func (r *fieldReader) coordinate() (fp.Element, error) {
	var out fp.Element

	lo, err := r.next()

	if err != nil {
		return out, err
	}

	hi, err := r.next()

	if err != nil {
		return out, err
	}

	l, h := bigOf(lo), bigOf(hi)

	if l.Cmp(two136) >= 0 || h.Cmp(two118) >= 0 {
		return out, errors.New("honk: coordinate limb out of range")
	}

	v := new(big.Int).Or(l, new(big.Int).Lsh(h, limbBits))

	if v.Cmp(fp.Modulus()) >= 0 {
		return out, errors.New("honk: coordinate >= q")
	}

	out.SetBigInt(v)

	return out, nil
}

// point reads a G1 point; (0, 0) encodes the identity, anything else must be on the curve.
func (r *fieldReader) point() (bn254.G1Affine, error) {
	var p bn254.G1Affine

	var err error

	if p.X, err = r.coordinate(); err != nil {
		return p, err
	}

	if p.Y, err = r.coordinate(); err != nil {
		return p, err
	}

	if !p.IsInfinity() && !p.IsOnCurve() {
		return p, errors.New("honk: point not on curve")
	}

	return p, nil
}

func coordinateLimbs(e fp.Element) [2]fr.Element {
	v := e.BigInt(new(big.Int))

	var lo, hi fr.Element

	lo.SetBigInt(new(big.Int).And(v, new(big.Int).Sub(two136, big.NewInt(1))))

	hi.SetBigInt(new(big.Int).Rsh(v, limbBits))

	return [2]fr.Element{lo, hi}
}

// PointLimbs returns the transcript encoding of a point.
func PointLimbs(p bn254.G1Affine) []fr.Element {
	x, y := coordinateLimbs(p.X), coordinateLimbs(p.Y)

	return []fr.Element{x[0], x[1], y[0], y[1]}
}
