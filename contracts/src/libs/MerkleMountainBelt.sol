// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {Poseidon2T4} from "privacy-boost-protocol-62a8140/Poseidon2T4.sol";
import {LibBit} from "solady-0.1.26/src/utils/LibBit.sol";

/// @title MerkleMountainBelt
/// @author Heueristik, 2026
/// @notice An append-only list of leaves with a root that commits to all leaves, in which the proof of the k-th newest
/// leaf has at most 2 * log2(k) + 3 hashes (Cevallos, Hambrock and Stewart, "Merkle Mountain Belts", arXiv 2511.13582).
/// @dev The nodes are Poseidon2 hashes over the BN254 scalar field, so that a circuit can prove membership with few
/// constraints. The belt stores the peaks, and only those range and belt nodes that an append of `_TYPICAL_APPEND_SIZE`
/// leaves does not change.
/// @custom:security-contact security@anoma.foundation
library MerkleMountainBelt {
    /// @notice The nodes of the mountain in one slot: its peak, and the range and belt nodes that end at it.
    /// @dev A range node or belt node is valid only for the mountains that `_sharedMountainCount` counts.
    struct Mountain {
        bytes32 peak;
        bytes32 rangeNode;
        bytes32 beltNode;
    }

    /// @notice The belt state: the leaf count and one mountain for each of the 128 slots, two for each height up to 63.
    /// @dev The slot of a mountain is twice its height, plus one unless it is the right one of two of equal height.
    struct Belt {
        uint256 _leafCount;
        Mountain[128] _mountains;
    }

    /// @notice The mountains of one leaf count, from the bits of the leaf count plus one below its top bit.
    /// @dev Each bit position holds one mountain; the mountains run from the top bit position down to position 0.
    struct Shape {
        uint256 mountainCount;
        uint256 heightBits;
        uint256 rangeStartBits;
        uint256 rightOfPairBits;
    }

    /// @notice The root of the belt without leaves, which is also the root of the belt with the one leaf zero.
    bytes32 internal constant _EMPTY_ROOT = bytes32(0);

    /// @notice The first input of each node hash, which separates the node hashes from the other Poseidon2 hashes.
    uint256 internal constant _NODE_DOMAIN = 9;

    /// @notice The largest leaf count, for which no mountain is higher than 63.
    uint256 internal constant _MAX_LEAF_COUNT = type(uint64).max - 1;

    /// @notice The append size for which the belt stores the range and belt nodes; other sizes give the same root.
    uint256 internal constant _TYPICAL_APPEND_SIZE = 4;

    /// @notice Thrown if a leaf is not smaller than the BN254 scalar field modulus.
    /// @param leaf The leaf.
    error LeafNotInField(bytes32 leaf);

    /// @notice Thrown if an append would make the leaf count larger than `_MAX_LEAF_COUNT`.
    /// @param leafCount The leaf count after the append.
    error LeafCountTooLarge(uint256 leafCount);

    /// @notice Appends leaves in their order and returns the new root.
    /// @param self The belt.
    /// @param leaves The leaves, each smaller than the BN254 scalar field modulus; an empty list leaves the belt
    /// unchanged.
    /// @return beltRoot The root after the append.
    function append(Belt storage self, bytes32[] memory leaves) internal returns (bytes32 beltRoot) {
        uint256 oldCount = self._leafCount;
        if (leaves.length == 0) return root(self);
        uint256 count = oldCount + leaves.length;
        if (count > _MAX_LEAF_COUNT) revert LeafCountTooLarge(count);
        uint256 leafTotal = leaves.length;
        for (uint256 i = 0; i < leafTotal; ++i) {
            // NOTE: The hash reduces its inputs, so a leaf of the modulus or more would stand for another leaf.
            // forge-lint: disable-next-item(require-revert-in-loop)
            if (uint256(leaves[i]) >= Poseidon2T4.PRIME) revert LeafNotInField(leaves[i]);
        }

        Shape memory shape = _shape(count);

        // The append merges in memory and writes only the peaks that are left at its end.
        (bytes32[] memory createdPeaks, uint256 createdSlotBits) = _merge(self, shape, oldCount, leaves);

        uint256 validCount = 0;
        if (oldCount != 0) {
            Shape memory oldShape = _shape(oldCount);
            validCount = _min(
                _sharedMountainCount(oldShape, count), _sharedMountainCount(oldShape, oldCount + _TYPICAL_APPEND_SIZE)
            );
        }
        beltRoot = _bag({
            self: self,
            shape: shape,
            validCount: validCount,
            keptCount: _sharedMountainCount(shape, count + _TYPICAL_APPEND_SIZE),
            createdPeaks: createdPeaks,
            createdSlotBits: createdSlotBits
        });

        while (createdSlotBits != 0) {
            uint256 slot = LibBit.ffs(createdSlotBits);
            self._mountains[slot].peak = createdPeaks[slot];
            createdSlotBits &= createdSlotBits - 1;
        }
        self._leafCount = count;
    }

    /// @notice Computes the root of the belt.
    /// @param self The belt.
    /// @return beltRoot The root, or `_EMPTY_ROOT` if the belt has no leaves.
    function root(Belt storage self) internal view returns (bytes32 beltRoot) {
        uint256 count = self._leafCount;
        if (count == 0) return _EMPTY_ROOT;

        Shape memory shape = _shape(count);
        uint256 validCount = _sharedMountainCount(shape, count + _TYPICAL_APPEND_SIZE);

        (bytes32 beltNode, bool hasBeltNode, bytes32 rangeNode) = _resume(self, shape, validCount);
        uint256 position = shape.mountainCount - validCount;
        while (position != 0) {
            --position;
            bytes32 peak = self._mountains[_slot(shape, position)].peak;
            if (position == shape.mountainCount - 1) {
                rangeNode = peak;
            } else if ((shape.rangeStartBits >> position) & 1 == 1) {
                beltNode = hasBeltNode ? _hash(beltNode, rangeNode) : rangeNode;
                hasBeltNode = true;
                rangeNode = peak;
            } else {
                rangeNode = _hash(rangeNode, peak);
            }
        }
        beltRoot = hasBeltNode ? _hash(beltNode, rangeNode) : rangeNode;
    }

    /// @notice Returns the number of leaves in the belt.
    /// @param self The belt.
    /// @return count The number of leaves, which is also the number of the newest leaf.
    function leafCount(Belt storage self) internal view returns (uint256 count) {
        count = self._leafCount;
    }

    /// @notice Computes the root after an append and stores the range and belt nodes that the next append reads.
    /// @param self The belt.
    /// @param shape The shape of the new leaf count.
    /// @param validCount The number of leading mountains whose range and belt nodes are valid in storage.
    /// @param keptCount The number of leading mountains whose range and belt nodes the next append reads.
    /// @param createdPeaks The peaks that the append created, by slot.
    /// @param createdSlotBits A bit for each slot in `createdPeaks` that holds a peak.
    /// @return beltRoot The root after the append.
    function _bag(
        Belt storage self,
        Shape memory shape,
        uint256 validCount,
        uint256 keptCount,
        bytes32[] memory createdPeaks,
        uint256 createdSlotBits
    ) private returns (bytes32 beltRoot) {
        (bytes32 beltNode, bool hasBeltNode, bytes32 rangeNode) = _resume(self, shape, validCount);
        uint256 mountainCount = shape.mountainCount;
        // The mountains before `keptCount` sit at the positions from `keptFrom` up.
        uint256 keptFrom = mountainCount - keptCount;
        uint256 position = mountainCount - validCount;
        uint256 previousSlot = validCount == 0 ? 0 : _slot(shape, position);
        while (position != 0) {
            --position;
            uint256 slot = _slot(shape, position);
            bytes32 peak = (createdSlotBits >> slot) & 1 == 1 ? createdPeaks[slot] : self._mountains[slot].peak;
            if (position == mountainCount - 1) {
                rangeNode = peak;
            } else if ((shape.rangeStartBits >> position) & 1 == 1) {
                beltNode = hasBeltNode ? _hash(beltNode, rangeNode) : rangeNode;
                hasBeltNode = true;
                if (position >= keptFrom) self._mountains[previousSlot].beltNode = beltNode;
                rangeNode = peak;
            } else {
                rangeNode = _hash(rangeNode, peak);
                if (position >= keptFrom) self._mountains[slot].rangeNode = rangeNode;
            }
            previousSlot = slot;
        }
        beltRoot = hasBeltNode ? _hash(beltNode, rangeNode) : rangeNode;
    }

    /// @notice Adds the leaves as mountains of height 0 and does the merge of each append step.
    /// @param self The belt.
    /// @param shape The shape of the new leaf count.
    /// @param oldCount The leaf count before the append.
    /// @param leaves The leaves.
    /// @return createdPeaks The peaks that the append created and did not merge again, by slot.
    /// @return createdSlotBits A bit for each slot in `createdPeaks` that holds a peak.
    function _merge(Belt storage self, Shape memory shape, uint256 oldCount, bytes32[] memory leaves)
        private
        view
        returns (bytes32[] memory createdPeaks, uint256 createdSlotBits)
    {
        // No mountain is higher than the mountain count of the new leaf count.
        createdPeaks = new bytes32[](2 * shape.mountainCount + 2);

        uint256 leafNumber = oldCount;
        uint256 leafTotal = leaves.length;
        for (uint256 i = 0; i < leafTotal; ++i) {
            ++leafNumber;
            uint256 leafSlot = leafNumber & 1;
            createdPeaks[leafSlot] = leaves[i];
            createdSlotBits |= 1 << leafSlot;

            // Step n merges the two mountains of height ν2(n + 1), unless n + 1 is a power of two.
            uint256 next = leafNumber + 1;
            if (next & leafNumber != 0) {
                uint256 height = LibBit.ffs(next);
                uint256 rightSlot = 2 * height;
                bytes32 left = (createdSlotBits >> (rightSlot + 1)) & 1 == 1
                    ? createdPeaks[rightSlot + 1]
                    : self._mountains[rightSlot + 1].peak;
                bytes32 right =
                    (createdSlotBits >> rightSlot) & 1 == 1 ? createdPeaks[rightSlot] : self._mountains[rightSlot].peak;
                createdSlotBits &= ~(uint256(3) << rightSlot);

                uint256 mergedSlot = rightSlot + 2 + ((next >> (height + 1)) & 1);
                createdPeaks[mergedSlot] = _hash(left, right);
                createdSlotBits |= 1 << mergedSlot;
            }
        }
    }

    /// @notice Reads the bagging state after the last valid mountain from storage.
    /// @param self The belt.
    /// @param shape The shape of the current leaf count.
    /// @param validCount The number of leading mountains whose range and belt nodes are valid in storage.
    /// @return beltNode The belt node of the ranges before the range of the last valid mountain.
    /// @return hasBeltNode Whether a range comes before the range of the last valid mountain.
    /// @return rangeNode The range node of the last valid mountain.
    function _resume(Belt storage self, Shape memory shape, uint256 validCount)
        private
        view
        returns (bytes32 beltNode, bool hasBeltNode, bytes32 rangeNode)
    {
        if (validCount == 0) return (beltNode, hasBeltNode, rangeNode);

        uint256 position = shape.mountainCount - validCount;
        // The range of the last valid mountain starts at the nearest start bit at or above its position.
        uint256 startPosition = position + LibBit.ffs(shape.rangeStartBits >> position);

        Mountain storage last = self._mountains[_slot(shape, position)];
        rangeNode = startPosition == position ? last.peak : last.rangeNode;
        if (startPosition + 1 < shape.mountainCount) {
            beltNode = self._mountains[_slot(shape, startPosition + 1)].beltNode;
            hasBeltNode = true;
        }
    }

    /// @notice Hashes two nodes into their parent.
    /// @param left The left node.
    /// @param right The right node.
    /// @return parent The Poseidon2 hash of the node domain and both nodes.
    function _hash(bytes32 left, bytes32 right) private pure returns (bytes32 parent) {
        parent = bytes32(Poseidon2T4.hash3(_NODE_DOMAIN, uint256(left), uint256(right)));
    }

    /// @notice Computes the mountains of a leaf count.
    /// @param count The leaf count, which must not be zero.
    /// @return shape The shape.
    function _shape(uint256 count) private pure returns (Shape memory shape) {
        unchecked {
            // `count` is at most `_MAX_LEAF_COUNT`, so no operation overflows.
            uint256 bits = count + 1;
            uint256 mountainCount = LibBit.fls(bits);
            uint256 mask = (uint256(1) << mountainCount) - 1;
            shape.mountainCount = mountainCount;
            shape.heightBits = bits;
            // A range starts after a one bit that a zero bit follows on either side.
            shape.rangeStartBits = (bits >> 1) & (~bits | ~(bits >> 2)) & mask;
            // Two mountains have equal height where a one bit follows a zero bit.
            shape.rightOfPairBits = ~(bits >> 1) & bits & mask;
        }
    }

    /// @notice Computes the number of leading mountains whose peaks and range starts are the same for two leaf counts.
    /// @param from The shape of the smaller leaf count.
    /// @param toCount The larger leaf count.
    /// @return count The number of leading mountains.
    function _sharedMountainCount(Shape memory from, uint256 toCount) private pure returns (uint256 count) {
        unchecked {
            uint256 fromMountains = from.mountainCount;
            uint256 toBits = toCount + 1;
            uint256 changedBits = from.heightBits ^ toBits;
            if (changedBits >> fromMountains == 0) {
                // Same mountain count: the mountains above the highest changed bit keep their height and range start.
                count = fromMountains - 1 - LibBit.fls(changedBits);
            } else if (toBits >> (fromMountains + 2) == 0) {
                // One more mountain: a one bit before the new top bit and a zero bit after it keep the height.
                uint256 fromZeros = ~from.heightBits & ((uint256(1) << fromMountains) - 1);
                uint256 leadingOnes = fromZeros == 0 ? fromMountains : fromMountains - 1 - LibBit.fls(fromZeros);
                uint256 toOnes = toBits & ((uint256(1) << (fromMountains + 1)) - 1);
                uint256 leadingZeros = toOnes == 0 ? fromMountains + 1 : fromMountains - LibBit.fls(toOnes);
                count = _min(leadingOnes, leadingZeros);
            }
        }
    }

    /// @notice Computes the slot of the mountain at a bit position.
    /// @param shape The shape.
    /// @param position The bit position, below the mountain count.
    /// @return slot The slot.
    function _slot(Shape memory shape, uint256 position) private pure returns (uint256 slot) {
        unchecked {
            uint256 height = position + ((shape.heightBits >> position) & 1);
            slot = 2 * height + 1 - ((shape.rightOfPairBits >> position) & 1);
        }
    }

    /// @notice Returns the smaller of two numbers.
    /// @param a The first number.
    /// @param b The second number.
    /// @return smaller The smaller number.
    function _min(uint256 a, uint256 b) private pure returns (uint256 smaller) {
        smaller = a < b ? a : b;
    }
}
