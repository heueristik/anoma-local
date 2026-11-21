// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {IGroth16Verifier} from "../../src/interfaces/IGroth16Verifier.sol";

/// @notice Accepts a proof only if it is the ABI encoding of the public input.
contract Groth16VerifierMock is IGroth16Verifier {
    error ProofInvalid();

    function verifyProof(bytes calldata proof, uint256[1] calldata input) external pure override {
        if (keccak256(proof) != keccak256(abi.encode(input[0]))) revert ProofInvalid();
    }
}
