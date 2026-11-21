// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {Poseidon2T4} from "privacy-boost-protocol-62a8140/Poseidon2T4.sol";

/// @dev Computes the root of a Merkle Mountain Belt step by step as the paper defines it, with no stored nodes and no
/// bit tricks. A test pushes each leaf, so that a root costs only the hashes of its bagging.
library BeltReference {
    /// @dev The mountains after `leafCount` steps, from the oldest to the newest.
    struct Mountains {
        uint256 leafCount;
        uint256 count;
        // Room for the mountains of any leaf count below 2^64, plus the leaf that a step adds before it merges.
        bytes32[66] peaks;
        uint256[66] heights;
    }

    bytes32 internal constant EMPTY_ROOT = bytes32(0);

    uint256 internal constant NODE_DOMAIN = 9;

    error MountainCountMismatch(uint256 mountainCount, uint256 bitCount);

    /// @dev Appends the leaf as a mountain of height 0; step n then merges the rightmost two mountains of height
    /// ν2(n + 1), unless n + 1 is a power of two.
    function push(Mountains memory self, bytes32 leaf) internal pure {
        uint256 leafNumber = ++self.leafCount;
        self.peaks[self.count] = leaf;
        self.heights[self.count] = 0;
        ++self.count;

        uint256 next = leafNumber + 1;
        if (next & leafNumber == 0) return;
        uint256 height = 0;
        while ((next >> height) & 1 == 0) {
            ++height;
        }
        uint256 right = self.count - 1;
        while (!(self.heights[right] == height && self.heights[right - 1] == height)) {
            --right;
        }
        self.peaks[right - 1] = hash(self.peaks[right - 1], self.peaks[right]);
        self.heights[right - 1] = height + 1;
        for (uint256 i = right; i + 1 < self.count; ++i) {
            self.peaks[i] = self.peaks[i + 1];
            self.heights[i] = self.heights[i + 1];
        }
        --self.count;
    }

    function root(Mountains memory self) internal pure returns (bytes32 beltRoot) {
        if (self.leafCount == 0) return EMPTY_ROOT;

        // The bits of the leaf count plus one below its top bit, from the top: one bit for each mountain.
        uint256 bitsOfNext = self.leafCount + 1;
        uint256 bitCount = 0;
        while (bitsOfNext >> (bitCount + 1) != 0) {
            ++bitCount;
        }
        if (bitCount != self.count) revert MountainCountMismatch(self.count, bitCount);

        bytes32 beltNode;
        bool hasBeltNode;
        bytes32 rangeNode;
        for (uint256 i = 0; i < self.count; ++i) {
            if (i == 0) {
                rangeNode = self.peaks[0];
            } else if (startsRange(bitsOfNext, bitCount, i)) {
                beltNode = hasBeltNode ? hash(beltNode, rangeNode) : rangeNode;
                hasBeltNode = true;
                rangeNode = self.peaks[i];
            } else {
                rangeNode = hash(rangeNode, self.peaks[i]);
            }
        }
        beltRoot = hasBeltNode ? hash(beltNode, rangeNode) : rangeNode;
    }

    /// @dev Mountain i starts a range if the bit before it is one, and its own bit or the bit two before it is zero.
    function startsRange(uint256 bitsOfNext, uint256 bitCount, uint256 i) internal pure returns (bool isStart) {
        uint256 bit = (bitsOfNext >> (bitCount - 1 - i)) & 1;
        uint256 previous = (bitsOfNext >> (bitCount - i)) & 1;
        bool hasTwoBefore = i >= 2;
        uint256 twoBefore = hasTwoBefore ? (bitsOfNext >> (bitCount + 1 - i)) & 1 : 1;
        isStart = previous == 1 && (bit == 0 || twoBefore == 0);
    }

    function hash(bytes32 left, bytes32 right) internal pure returns (bytes32 parent) {
        parent = bytes32(Poseidon2T4.hash3(NODE_DOMAIN, uint256(left), uint256(right)));
    }
}
