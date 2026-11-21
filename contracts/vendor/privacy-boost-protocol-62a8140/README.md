# Poseidon2T4

`Poseidon2T4.sol` is the Poseidon2 hash of the Privacy Boost protocol by Sunnyside Labs, from [`contracts/src/hash/Poseidon2T4.sol`](https://github.com/sunnyside-io/privacy-boost-protocol/blob/62a8140178a7812c3019fe2845cbda870ad8a1c1/contracts/src/hash/Poseidon2T4.sol) at commit `62a8140`. OpenZeppelin audited the file; the reports are in the [`audits`](https://github.com/sunnyside-io/privacy-boost-protocol/tree/62a8140178a7812c3019fe2845cbda870ad8a1c1/audits) folder of that repository.

The copy differs from the source in one line: the pragma is `^0.8.30` in place of `0.8.34`. The license is Apache-2.0, in `LICENSE`.

The hash reduces each input modulo the BN254 scalar field modulus `p`, so an input of `p` or more gives the hash of a different input. A caller must reject such inputs.
