// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {SafeCast} from "@openzeppelin-contracts-5.7.0/utils/math/SafeCast.sol";

import {IAnomaVerifier} from "../interfaces/IAnomaVerifier.sol";

/// @title ActionDigest
/// @author Heueristik, 2026
/// @notice The digest that binds the proof of a transaction to its actions. It starts at zero, and each action extends
/// it in the order of the actions, as the step circuits of the final circuit do. The root circuit then binds it to one
/// chain and one AnomaVerifier.
/// @custom:security-contact security@anoma.foundation
library ActionDigest {
    /// @notice Keeps the low 253 bits of a hash, so that the result is smaller than the BN254 scalar field modulus.
    uint256 internal constant _FIELD_MASK = (uint256(1) << 253) - 1;

    /// @notice The number of fields that each resource adds to the hash of `extend`.
    uint256 internal constant _RESOURCE_FIELDS = 5;

    /// @notice Extends a digest with an action.
    /// @param digest The digest of the actions before `action`, or zero for the first action.
    /// @param action The action.
    /// @return extended The low 253 bits of the SHA-256 hash of `digest`, then for each resource in the order of the
    /// action its `isConsumed` flag, tag, logic reference, commitment tree root and app data hash, each as 32 bytes.
    function extend(bytes32 digest, IAnomaVerifier.Action calldata action) internal pure returns (bytes32 extended) {
        IAnomaVerifier.Resource[] calldata resources = action.resources;
        uint256 resourceCount = resources.length;
        bytes32[] memory fields = new bytes32[](1 + _RESOURCE_FIELDS * resourceCount);
        fields[0] = digest;
        for (uint256 i = 0; i < resourceCount; ++i) {
            IAnomaVerifier.Resource calldata resource = resources[i];
            uint256 first = 1 + _RESOURCE_FIELDS * i;
            fields[first] = resource.isConsumed ? bytes32(uint256(1)) : bytes32(0);
            fields[first + 1] = resource.tag;
            fields[first + 2] = resource.logicRef;
            fields[first + 3] = resource.commitmentTreeRoot;
            fields[first + 4] = appDataHash(resource.appData);
        }
        // The packed encoding of a `bytes32` array is its elements, without a length.
        extended = _low253Bits(sha256(abi.encodePacked(fields)));
    }

    /// @notice Binds an action digest to one chain and one AnomaVerifier, so that the proof of a transaction verifies
    /// on one AnomaVerifier only.
    /// @param digest The action digest of the transaction.
    /// @param chainId The chain ID.
    /// @param verifier The address of the AnomaVerifier.
    /// @return bound The low 253 bits of the SHA-256 hash of the ABI encoding of `chainId`, `verifier` and `digest`.
    function bind(bytes32 digest, uint256 chainId, address verifier) internal pure returns (bytes32 bound) {
        bound = _low253Bits(sha256(abi.encode(chainId, verifier, digest)));
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
