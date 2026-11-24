// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";
import {Poseidon2T4} from "privacy-boost-protocol-62a8140/Poseidon2T4.sol";

import {MerkleMountainBelt} from "../../src/libs/MerkleMountainBelt.sol";
import {BeltReference} from "./BeltReference.sol";

/// @dev Compares the belt with the `Belt` of the Rust crate `anoma-arm`, which makes the paths that the compliance
/// circuit checks against the root of the contract.
contract MerkleMountainBeltDifferentialTest is Test {
    using MerkleMountainBelt for MerkleMountainBelt.Belt;

    /// @dev The binary of `crates/belt-ffi`, which `just contracts-test` builds; forge runs it in the `contracts` folder.
    string internal constant _RUST_BELT = "../target/release/anoma-belt-ffi";

    /// @dev The most commitments that a fuzz run appends, as the Rust belt computes each root from all leaves.
    uint256 internal constant _MAX_FUZZ_COMMITMENTS = 128;

    MerkleMountainBelt.Belt internal _belt;

    /// forge-config: default.fuzz.runs = 256
    function testFuzz_append_matches_the_rust_library_for_random_commitments(bytes32[] memory commitments, uint256 seed)
        public
    {
        bytes32[] memory leaves = _toLeaves(commitments);
        (uint256[] memory sizes, bytes32[] memory roots) = _appendInRandomBatches(leaves, seed);
        _assertRustRoots(leaves, sizes, roots);
    }

    /// forge-config: default.fuzz.runs = 256
    function testFuzz_path_of_the_rust_library_leads_from_a_random_commitment_to_the_root(
        bytes32[] memory commitments,
        uint256 seed,
        uint256 index
    ) public {
        bytes32[] memory leaves = _toLeaves(commitments);
        vm.assume(leaves.length != 0);
        _appendInRandomBatches(leaves, seed);
        index = _bound(index, 0, leaves.length - 1);

        (bytes32[] memory siblings, bool[] memory isLeft) =
            abi.decode(_rust("path", abi.encode(leaves, index)), (bytes32[], bool[]));
        assertEq(siblings.length, isLeft.length, "the path has one side for each sibling");
        // The compliance circuit computes the root from the leaf and the path in this way.
        bytes32 node = leaves[index];
        for (uint256 i = 0; i < siblings.length; ++i) {
            node = isLeft[i] ? BeltReference.hash(siblings[i], node) : BeltReference.hash(node, siblings[i]);
        }
        assertEq(node, _belt.root(), string.concat("leaf ", vm.toString(index)));
    }

    function testFuzz_append_of_two_leaves_gives_the_node_hash_of_the_rust_library(uint256 left, uint256 right) public {
        bytes32[] memory leaves = new bytes32[](2);
        leaves[0] = bytes32(_bound(left, 0, Poseidon2T4.PRIME - 1));
        leaves[1] = bytes32(_bound(right, 0, Poseidon2T4.PRIME - 1));
        uint256[] memory sizes = new uint256[](1);
        sizes[0] = leaves.length;
        bytes32[] memory roots = new bytes32[](1);
        roots[0] = _belt.append(leaves);
        _assertRustRoots(leaves, sizes, roots);
    }

    function test_append_matches_the_rust_library_for_each_leaf_count_up_to_300() public {
        bytes32[] memory leaves = new bytes32[](300);
        uint256[] memory sizes = new uint256[](leaves.length);
        bytes32[] memory roots = new bytes32[](leaves.length);
        bytes32[] memory batch = new bytes32[](1);
        for (uint256 i = 0; i < leaves.length; ++i) {
            leaves[i] = _leaf(i + 1);
            batch[0] = leaves[i];
            sizes[i] = 1;
            roots[i] = _belt.append(batch);
        }
        _assertRustRoots(leaves, sizes, roots);
    }

    function test_append_matches_the_rust_library_across_a_power_of_two_far_from_the_start() public {
        uint256 power = 1 << 11;
        bytes32[] memory leaves = new bytes32[](power + 4);
        for (uint256 i = 0; i < leaves.length; ++i) {
            leaves[i] = _leaf(i + 1);
        }
        // Appends of 64 up to 6 leaves before the power, then appends of 1 to 4 leaves, which end 4 leaves after it.
        uint256 bulkCount = (power - 6 + 63) / 64;
        uint256[] memory sizes = new uint256[](bulkCount + 4);
        for (uint256 i = 0; i < bulkCount; ++i) {
            sizes[i] = i + 1 < bulkCount ? 64 : power - 6 - 64 * (bulkCount - 1);
        }
        for (uint256 size = 1; size <= 4; ++size) {
            sizes[bulkCount + size - 1] = size;
        }
        _assertRustRoots(leaves, sizes, _appendInBatches(leaves, sizes));
    }

    function test_append_matches_the_rust_library_for_the_smallest_and_largest_field_elements() public {
        bytes32[] memory leaves = new bytes32[](40);
        for (uint256 i = 0; i < leaves.length; ++i) {
            leaves[i] = bytes32(i % 2 == 0 ? i / 2 : Poseidon2T4.PRIME - 1 - i / 2);
        }
        (uint256[] memory sizes, bytes32[] memory roots) = _appendInRandomBatches(leaves, 0);
        _assertRustRoots(leaves, sizes, roots);
    }

    /// @dev Appends the leaves in batches of 1 to 3 times the typical size, so that some appends read nodes that the
    /// belt did not store.
    function _appendInRandomBatches(bytes32[] memory leaves, uint256 seed)
        internal
        returns (uint256[] memory sizes, bytes32[] memory roots)
    {
        uint256[] memory allSizes = new uint256[](leaves.length);
        uint256 batchCount = 0;
        uint256 next = 0;
        while (next < leaves.length) {
            seed = uint256(keccak256(abi.encode(seed)));
            uint256 size = 1 + seed % (3 * MerkleMountainBelt._TYPICAL_APPEND_SIZE);
            if (size > leaves.length - next) size = leaves.length - next;
            allSizes[batchCount] = size;
            ++batchCount;
            next += size;
        }
        sizes = new uint256[](batchCount);
        for (uint256 i = 0; i < batchCount; ++i) {
            sizes[i] = allSizes[i];
        }
        roots = _appendInBatches(leaves, sizes);
    }

    /// @dev Appends the leaves in batches of the sizes, and returns the root after each append.
    function _appendInBatches(bytes32[] memory leaves, uint256[] memory sizes)
        internal
        returns (bytes32[] memory roots)
    {
        roots = new bytes32[](sizes.length);
        uint256 next = 0;
        for (uint256 i = 0; i < sizes.length; ++i) {
            bytes32[] memory batch = new bytes32[](sizes[i]);
            for (uint256 j = 0; j < sizes[i]; ++j) {
                batch[j] = leaves[next + j];
            }
            roots[i] = _belt.append(batch);
            next += sizes[i];
        }
    }

    /// @dev Checks that the Rust belt gives the roots after the same appends of the same leaves.
    function _assertRustRoots(bytes32[] memory leaves, uint256[] memory sizes, bytes32[] memory roots) internal {
        bytes32[] memory rustRoots = abi.decode(_rust("roots", abi.encode(leaves, sizes)), (bytes32[]));
        assertEq(rustRoots.length, roots.length, "the Rust belt gives one root for each append");
        uint256 leafCount = 0;
        for (uint256 i = 0; i < roots.length; ++i) {
            leafCount += sizes[i];
            assertEq(rustRoots[i], roots[i], string.concat("leaf count ", vm.toString(leafCount)));
        }
    }

    /// @dev Runs a command of the Rust belt on the ABI-encoded input and returns its ABI-encoded output.
    function _rust(string memory command, bytes memory input) internal returns (bytes memory output) {
        string[] memory arguments = new string[](3);
        arguments[0] = _RUST_BELT;
        arguments[1] = command;
        arguments[2] = vm.toString(input);
        output = vm.ffi(arguments);
    }

    /// @dev Takes at most `_MAX_FUZZ_COMMITMENTS` commitments, each mapped into the BN254 scalar field.
    function _toLeaves(bytes32[] memory commitments) internal pure returns (bytes32[] memory leaves) {
        uint256 count = commitments.length < _MAX_FUZZ_COMMITMENTS ? commitments.length : _MAX_FUZZ_COMMITMENTS;
        leaves = new bytes32[](count);
        for (uint256 i = 0; i < count; ++i) {
            leaves[i] = bytes32(_bound(uint256(commitments[i]), 0, Poseidon2T4.PRIME - 1));
        }
    }

    function _leaf(uint256 number) internal pure returns (bytes32 leaf) {
        leaf = bytes32(uint256(keccak256(abi.encode("leaf", number))) % Poseidon2T4.PRIME);
    }
}
