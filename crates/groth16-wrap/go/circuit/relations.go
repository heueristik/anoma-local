// Derived from Barretenberg (https://github.com/AztecProtocol/aztec-packages, Copyright 2022 Aztec, Apache-2.0):
// the subrelations of RelationsLib in the bb v6.0.0 Solidity verifier. Changed: expressed as a gnark circuit.

package circuit

import (
	"github.com/consensys/gnark/frontend"

	"github.com/wstran/ultrahonk-gnark/honk"
)

var negHalfC, limbSizeC, sublimbC, diagC, grumpkinBNegC = honk.RelationConstants()

// relations mirrors honk.AccumulateRelationEvaluations (31 subrelations of bb v6.0.0) over frontend variables.
func relations(api frontend.API, p []frontend.Variable, eta, romLogupGamma, beta, gamma, delta frontend.Variable, alphas []frontend.Variable, ds frontend.Variable) frontend.Variable {
	ev := make([]frontend.Variable, honk.NumberOfSubrelations)

	for i := range ev {
		ev[i] = 0
	}

	w := func(i int) frontend.Variable {
		return p[i]
	}

	// Arithmetic.
	qArith := w(honk.QArith)

	accum := api.Mul(api.Sub(qArith, 3), api.Mul(w(honk.QM), w(honk.WR), w(honk.WL)), frConst(negHalfC))

	accum = api.Add(accum, api.Mul(w(honk.QL), w(honk.WL)), api.Mul(w(honk.QR), w(honk.WR)), api.Mul(w(honk.QO), w(honk.WO)), api.Mul(w(honk.Q4), w(honk.W4)), w(honk.QC))

	accum = api.Add(accum, api.Mul(api.Sub(qArith, 1), w(honk.W4Shift)))

	ev[6] = api.Mul(accum, qArith, ds)

	a2 := api.Add(api.Sub(api.Add(w(honk.WL), w(honk.W4)), w(honk.WLShift)), w(honk.QM))

	ev[7] = api.Mul(a2, api.Sub(qArith, 2), api.Sub(qArith, 1), qArith, ds)

	// Permutation.
	term := func(wire, s int) frontend.Variable {
		return api.Add(w(wire), api.Mul(w(s), beta), gamma)
	}

	num := api.Mul(term(honk.WL, honk.Id1), term(honk.WR, honk.Id2), term(honk.WO, honk.Id3), term(honk.W4, honk.Id4))

	den := api.Mul(term(honk.WL, honk.Sigma1), term(honk.WR, honk.Sigma2), term(honk.WO, honk.Sigma3), term(honk.W4, honk.Sigma4))

	perm := api.Sub(api.Mul(api.Add(w(honk.ZPerm), w(honk.LagrangeFirst)), num), api.Mul(api.Add(w(honk.ZPermShift), api.Mul(w(honk.LagrangeLast), delta)), den))

	ev[0] = api.Mul(perm, ds)

	ev[1] = api.Mul(w(honk.LagrangeLast), w(honk.ZPermShift), ds)

	ev[2] = api.Mul(w(honk.LagrangeFirst), w(honk.ZPerm), ds)

	// Log-derivative lookup.
	betaSqr := api.Mul(beta, beta)

	betaCube := api.Mul(betaSqr, beta)

	tableTerm := api.Add(w(honk.Table1), gamma, api.Mul(w(honk.Table2), beta), api.Mul(w(honk.Table3), betaSqr), api.Mul(w(honk.Table4), betaCube))

	d1 := api.Add(w(honk.WL), gamma, api.Mul(w(honk.QR), w(honk.WLShift)))

	d2 := api.Add(w(honk.WR), api.Mul(w(honk.QM), w(honk.WRShift)))

	d3 := api.Add(w(honk.WO), api.Mul(w(honk.QC), w(honk.WOShift)))

	lookupTerm := api.Add(d1, api.Mul(d2, beta), api.Mul(d3, betaSqr), api.Mul(w(honk.QO), betaCube))

	lookupInverse := api.Mul(w(honk.LookupInverses), tableTerm)

	tableInverse := api.Mul(w(honk.LookupInverses), lookupTerm)

	inverseExistsXor := api.Sub(api.Add(w(honk.LookupReadTags), w(honk.QLookup)), api.Mul(w(honk.LookupReadTags), w(honk.QLookup)))

	ev[3] = api.Mul(api.Sub(api.Mul(lookupTerm, tableTerm, w(honk.LookupInverses)), inverseExistsXor), ds)

	ev[4] = api.Sub(api.Mul(w(honk.QLookup), lookupInverse), api.Mul(w(honk.LookupReadCounts), tableInverse))

	readTag := w(honk.LookupReadTags)

	ev[5] = api.Mul(api.Sub(api.Mul(readTag, readTag), readTag), ds)

	// Delta range.
	deltas := []frontend.Variable{api.Sub(w(honk.WR), w(honk.WL)), api.Sub(w(honk.WO), w(honk.WR)), api.Sub(w(honk.W4), w(honk.WO)), api.Sub(w(honk.WLShift), w(honk.W4))}

	for i, d := range deltas {
		ev[8+i] = api.Mul(d, api.Sub(d, 1), api.Sub(d, 2), api.Sub(d, 3), w(honk.QRange), ds)
	}

	// Elliptic.
	x1, y1 := w(honk.WR), w(honk.WO)

	x2, y2 := w(honk.WLShift), w(honk.W4Shift)

	y3, x3 := w(honk.WOShift), w(honk.WRShift)

	qSign, qIsDouble := w(honk.QL), w(honk.QM)

	xDiff := api.Sub(x2, x1)

	y1Sqr := api.Mul(y1, y1)

	y2Sqr := api.Mul(y2, y2)

	y1y2 := api.Mul(y1, y2, qSign)

	xAdd := api.Mul(api.Add(x3, x2, x1), xDiff, xDiff)

	xAdd = api.Add(api.Sub(xAdd, y2Sqr, y1Sqr), y1y2, y1y2)

	notDouble := api.Sub(1, qIsDouble)

	qEll := w(honk.QElliptic)

	ev[12] = api.Mul(xAdd, ds, qEll, notDouble)

	yAdd := api.Add(api.Mul(api.Add(y1, y3), xDiff), api.Mul(api.Sub(x3, x1), api.Sub(api.Mul(y2, qSign), y1)))

	ev[13] = api.Mul(yAdd, ds, qEll, notDouble)

	xPow4 := api.Mul(api.Add(y1Sqr, frConst(grumpkinBNegC)), x1)

	y1SqrMul4 := api.Mul(y1Sqr, 4)

	xDouble := api.Sub(api.Mul(api.Add(x3, x1, x1), y1SqrMul4), api.Mul(xPow4, 9))

	ev[12] = api.Add(ev[12], api.Mul(xDouble, ds, qEll, qIsDouble))

	x1SqrMul3 := api.Mul(api.Mul(x1, 3), x1)

	yDouble := api.Sub(api.Mul(x1SqrMul3, api.Sub(x1, x3)), api.Mul(api.Add(y1, y1), api.Add(y1, y3)))

	ev[13] = api.Add(ev[13], api.Mul(yDouble, ds, qEll, qIsDouble))

	// Memory.
	eta2 := api.Mul(eta, eta)

	eta3 := api.Mul(eta2, eta)

	recordCheck := api.Add(api.Mul(w(honk.WO), eta3), api.Mul(w(honk.WR), eta2), api.Mul(w(honk.WL), eta), w(honk.QC))

	partialRecordCheck := recordCheck

	recordCheck = api.Sub(recordCheck, w(honk.W4))

	indexDelta := api.Sub(w(honk.WLShift), w(honk.WL))

	recordDelta := api.Sub(w(honk.W4Shift), w(honk.W4))

	indexIsMonotonic := api.Mul(indexDelta, api.Sub(indexDelta, 1))

	oneMinusIndexDelta := api.Sub(1, indexDelta)

	adjacentMatch := api.Mul(oneMinusIndexDelta, recordDelta)

	qMemDs := api.Mul(w(honk.QMemory), ds)

	qlqr := api.Mul(w(honk.QL), w(honk.QR))

	ev[15] = api.Mul(adjacentMatch, qlqr, qMemDs)

	ev[16] = api.Mul(indexIsMonotonic, qlqr, qMemDs)

	romConsistency := api.Mul(recordCheck, qlqr)

	accessType := api.Sub(w(honk.W4), partialRecordCheck)

	accessCheck := api.Mul(accessType, api.Sub(accessType, 1))

	nextAccessType := api.Sub(w(honk.W4Shift), api.Add(api.Mul(w(honk.WOShift), eta3), api.Mul(w(honk.WRShift), eta2), api.Mul(w(honk.WLShift), eta)))

	valueDelta := api.Sub(w(honk.WOShift), w(honk.WO))

	adjacentMatchRead := api.Mul(oneMinusIndexDelta, valueDelta, api.Sub(1, nextAccessType))

	nextAccessBoolean := api.Sub(api.Mul(nextAccessType, nextAccessType), nextAccessType)

	ev[17] = api.Mul(adjacentMatchRead, w(honk.QO), qMemDs)

	ev[18] = api.Mul(indexIsMonotonic, w(honk.QO), qMemDs)

	ev[19] = api.Mul(nextAccessBoolean, w(honk.QO), qMemDs)

	ramConsistency := api.Mul(accessCheck, w(honk.QO))

	timestampDelta := api.Sub(w(honk.WRShift), w(honk.WR))

	ramTimestamp := api.Sub(api.Mul(oneMinusIndexDelta, timestampDelta), w(honk.WO))

	identity := api.Add(romConsistency, api.Mul(ramTimestamp, w(honk.Q4), w(honk.QL)), api.Mul(recordCheck, w(honk.QM), w(honk.QL)), ramConsistency)

	ev[14] = api.Mul(identity, qMemDs)

	// ROM LogUp: table rows q_r (1 - q_l), read rows q_4 (1 - q_l); subrelation 21 is not scaled by the pow polynomial.
	notQL := api.Sub(1, w(honk.QL))

	qTable := api.Mul(w(honk.QR), notQL)

	qRead := api.Mul(w(honk.Q4), notQL)

	denom := api.Add(romLogupGamma, w(honk.WL), api.Mul(w(honk.WR), eta), api.Mul(w(honk.QC), eta2))

	ev[20] = api.Mul(api.Add(qTable, qRead), api.Sub(api.Mul(w(honk.W4), denom), 1), w(honk.QMemory), ds)

	ev[21] = api.Mul(api.Sub(qRead, api.Mul(qTable, w(honk.WO))), w(honk.W4), w(honk.QMemory))

	// Non-native field.
	limbSubproduct := api.Add(api.Mul(w(honk.WL), w(honk.WRShift)), api.Mul(w(honk.WLShift), w(honk.WR)))

	gate2 := api.Sub(api.Add(api.Mul(w(honk.WL), w(honk.W4)), api.Mul(w(honk.WR), w(honk.WO))), w(honk.WOShift))

	gate2 = api.Mul(api.Add(api.Sub(api.Mul(gate2, frConst(limbSizeC)), w(honk.W4Shift)), limbSubproduct), w(honk.Q4))

	limbSubproduct = api.Add(api.Mul(limbSubproduct, frConst(limbSizeC)), api.Mul(w(honk.WLShift), w(honk.WRShift)))

	gate1 := api.Mul(api.Sub(limbSubproduct, api.Add(w(honk.WO), w(honk.W4))), w(honk.QO))

	gate3 := api.Mul(api.Sub(api.Add(limbSubproduct, w(honk.W4)), api.Add(w(honk.WOShift), w(honk.W4Shift))), w(honk.QM))

	nnfIdentity := api.Mul(api.Add(gate1, gate2, gate3), w(honk.QR))

	s := frConst(sublimbC)

	acc1 := api.Mul(w(honk.WRShift), s)

	acc1 = api.Mul(api.Add(acc1, w(honk.WLShift)), s)

	acc1 = api.Mul(api.Add(acc1, w(honk.WO)), s)

	acc1 = api.Mul(api.Add(acc1, w(honk.WR)), s)

	acc1 = api.Mul(api.Sub(api.Add(acc1, w(honk.WL)), w(honk.W4)), w(honk.Q4))

	acc2 := api.Mul(w(honk.WOShift), s)

	acc2 = api.Mul(api.Add(acc2, w(honk.WRShift)), s)

	acc2 = api.Mul(api.Add(acc2, w(honk.WLShift)), s)

	acc2 = api.Mul(api.Add(acc2, w(honk.W4)), s)

	acc2 = api.Mul(api.Sub(api.Add(acc2, w(honk.WO)), w(honk.W4Shift)), w(honk.QM))

	limbIdentity := api.Mul(api.Add(acc1, acc2), w(honk.QO))

	ev[22] = api.Mul(api.Add(nnfIdentity, limbIdentity), w(honk.QNnf), ds)

	// Poseidon2 external.
	pow5 := func(x frontend.Variable) frontend.Variable {
		x2 := api.Mul(x, x)

		return api.Mul(x2, x2, x)
	}

	u1 := pow5(api.Add(w(honk.WL), w(honk.QL)))

	u2 := pow5(api.Add(w(honk.WR), w(honk.QR)))

	u3 := pow5(api.Add(w(honk.WO), w(honk.QO)))

	u4 := pow5(api.Add(w(honk.W4), w(honk.Q4)))

	t0 := api.Add(u1, u2)

	t1 := api.Add(u3, u4)

	t2 := api.Add(api.Mul(u2, 2), t1)

	t3 := api.Add(api.Mul(u4, 2), t0)

	v4 := api.Add(api.Mul(t1, 4), t3)

	v2 := api.Add(api.Mul(t0, 4), t2)

	v1 := api.Add(t3, v2)

	v3 := api.Add(t2, v4)

	qExt := api.Mul(w(honk.QPoseidon2External), ds)

	ev[23] = api.Mul(qExt, api.Sub(v1, w(honk.WLShift)))

	ev[24] = api.Mul(qExt, api.Sub(v2, w(honk.WRShift)))

	ev[25] = api.Mul(qExt, api.Sub(v3, w(honk.WOShift)))

	ev[26] = api.Mul(qExt, api.Sub(v4, w(honk.W4Shift)))

	// Poseidon2 internal.
	iu1 := pow5(api.Add(w(honk.WL), w(honk.QL)))

	iu2, iu3, iu4 := w(honk.WR), w(honk.WO), w(honk.W4)

	uSum := api.Add(iu1, iu2, iu3, iu4)

	qInt := api.Mul(w(honk.QPoseidon2Internal), ds)

	ev[27] = api.Mul(qInt, api.Sub(api.Add(api.Mul(iu1, frConst(diagC[0])), uSum), w(honk.WLShift)))

	ev[28] = api.Mul(qInt, api.Sub(api.Add(api.Mul(iu2, frConst(diagC[1])), uSum), w(honk.WRShift)))

	ev[29] = api.Mul(qInt, api.Sub(api.Add(api.Mul(iu3, frConst(diagC[2])), uSum), w(honk.WOShift)))

	ev[30] = api.Mul(qInt, api.Sub(api.Add(api.Mul(iu4, frConst(diagC[3])), uSum), w(honk.W4Shift)))

	out := ev[0]

	for i := 1; i < honk.NumberOfSubrelations; i++ {
		out = api.Add(out, api.Mul(ev[i], alphas[i-1]))
	}

	return out
}
