// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {SafeCast} from "@openzeppelin-contracts-5.7.0/utils/math/SafeCast.sol";

import {IAnomaVerifier} from "../interfaces/IAnomaVerifier.sol";

/// @title ActionDigest
/// @author Heueristik, 2026
/// @notice The digest that binds the proof of a transaction to its actions. It starts at zero, and each action extends
/// it in the order of the actions, as the step circuits of the final circuit do.
/// @custom:security-contact security@anoma.foundation
library ActionDigest {
    /// @notice Keeps the low 253 bits of a hash, so that the result is smaller than the BN254 scalar field modulus.
    uint256 internal constant _FIELD_MASK = (uint256(1) << 253) - 1;

    /// @notice Extends a digest with an action.
    /// @param digest The digest of the actions before `action`, or zero for the first action.
    /// @param action The action.
    /// @return extended The low 253 bits of the SHA-256 hash of the ABI encoding of `digest` and the fields of
    /// `action`, in their order, with each app data replaced by its hash.
    function extend(bytes32 digest, IAnomaVerifier.Action calldata action) internal pure returns (bytes32 extended) {
        IAnomaVerifier.ResourceInstance calldata consumed = action.consumed;
        IAnomaVerifier.ResourceInstance calldata created = action.created;
        extended = _low253Bits(
            sha256(
                abi.encode(
                    digest,
                    consumed.tag,
                    consumed.logicRef,
                    appDataHash(consumed.appData),
                    created.tag,
                    created.logicRef,
                    appDataHash(created.appData),
                    action.commitmentTreeRoot
                )
            )
        );
    }

    /// @notice Hashes the app data of a resource, as its resource logic does.
    /// @param appData The app data.
    /// @return hash The low 253 bits of the SHA-256 hash of the four payloads in their order. Each payload is a 4-byte
    /// blob count, then for each blob its 1-byte deletion criterion, its 4-byte length, and its bytes.
    function appDataHash(IAnomaVerifier.AppData calldata appData) internal pure returns (bytes32 hash) {
        hash = _low253Bits(
            sha256(
                bytes.concat(
                    _pack(appData.resourcePayload),
                    _pack(appData.discoveryPayload),
                    _pack(appData.externalPayload),
                    _pack(appData.applicationPayload)
                )
            )
        );
    }

    /// @notice Packs the blobs of one payload.
    /// @param blobs The blobs.
    /// @return packed The blob count, then the deletion criterion, length and bytes of each blob, all big-endian.
    function _pack(IAnomaVerifier.ExpirableBlob[] calldata blobs) private pure returns (bytes memory packed) {
        uint256 blobCount = blobs.length;
        packed = abi.encodePacked(SafeCast.toUint32(blobCount));
        for (uint256 i = 0; i < blobCount; ++i) {
            bytes calldata blob = blobs[i].blob;
            packed = bytes.concat(
                packed, bytes1(uint8(blobs[i].deletionCriterion)), bytes4(SafeCast.toUint32(blob.length)), blob
            );
        }
    }

    /// @notice Clears the top 3 bits of a hash.
    /// @param hash The hash.
    /// @return low The low 253 bits of `hash`.
    function _low253Bits(bytes32 hash) private pure returns (bytes32 low) {
        low = hash & bytes32(_FIELD_MASK);
    }
}
