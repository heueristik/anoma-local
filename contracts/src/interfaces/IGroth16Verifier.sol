// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

/// @title IGroth16Verifier
/// @author Heueristik, 2026
/// @notice The interface of the Groth16 verifier contract that gnark generates for a circuit with one public input.
/// @custom:security-contact security@anoma.foundation
interface IGroth16Verifier {
    /// @notice Reverts unless the proof is valid for the public input.
    /// @param proof The points A, B and C of the proof in the EIP-197 format, followed by the commitments of the
    /// circuit, if it has any.
    /// @param input The public input, which must be smaller than the BN254 scalar field modulus.
    function verifyProof(bytes calldata proof, uint256[1] calldata input) external view;
}
