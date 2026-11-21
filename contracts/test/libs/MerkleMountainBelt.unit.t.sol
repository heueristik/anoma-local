// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";
import {Poseidon2T4} from "privacy-boost-protocol-62a8140/Poseidon2T4.sol";

import {MerkleMountainBelt} from "../../src/libs/MerkleMountainBelt.sol";
import {BeltReference} from "./BeltReference.sol";

contract MerkleMountainBeltTest is Test {
    using MerkleMountainBelt for MerkleMountainBelt.Belt;
    using BeltReference for BeltReference.Mountains;

    MerkleMountainBelt.Belt internal _belt;

    function test_append_of_no_leaves_leaves_the_root_unchanged() public {
        BeltReference.Mountains memory mountains;
        _appendNext(mountains, 5);
        bytes32 rootBefore = _belt.root();

        assertEq(_belt.append(new bytes32[](0)), rootBefore);
        assertEq(_belt.leafCount(), 5);
    }

    function test_append_of_one_leaf_gives_the_leaf_as_root() public {
        BeltReference.Mountains memory mountains;
        assertEq(_appendNext(mountains, 1), _leaf(1));
    }

    function test_append_of_two_leaves_gives_the_node_hash_of_the_circuits() public {
        bytes32[] memory leaves = new bytes32[](2);
        leaves[0] = bytes32(uint256(1));
        leaves[1] = bytes32(uint256(2));

        // The value of `node_hash(1, 2)` in the Noir library of the circuits.
        assertEq(_belt.append(leaves), 0x2437d9514d36909ae8e6c80bb839bd22a8ad60d9b31d2ce92392151ec5d2aba9);
    }

    function test_append_of_eleven_leaves_gives_the_root_of_the_rust_library() public {
        bytes32[] memory leaves = new bytes32[](11);
        for (uint256 i = 0; i < leaves.length; ++i) {
            leaves[i] = bytes32(i + 1);
        }

        // The root of `Belt` in the Rust crate `anoma-arm` for the leaves 1 to 11.
        assertEq(_belt.append(leaves), 0x1c7e6bc56e76a52583ecdd2af6e324a07dc0e4e88806498136a5d800ccafd0a7);
    }

    function test_append_of_seven_leaves_gives_one_range() public {
        // Three mountains of heights 2, 1 and 0, and the leaf count plus one, 0b1000, starts no second range.
        bytes32 first = _hash(_hash(_leaf(1), _leaf(2)), _hash(_leaf(3), _leaf(4)));
        bytes32 expected = _hash(_hash(first, _hash(_leaf(5), _leaf(6))), _leaf(7));

        BeltReference.Mountains memory mountains;
        assertEq(_appendNext(mountains, 7), expected);
    }

    function test_append_of_eleven_leaves_gives_two_ranges() public {
        // Three mountains of heights 3, 1 and 0, and the leaf count plus one, 0b1100, starts a range at the second one.
        bytes32 left = _hash(_hash(_leaf(1), _leaf(2)), _hash(_leaf(3), _leaf(4)));
        bytes32 right = _hash(_hash(_leaf(5), _leaf(6)), _hash(_leaf(7), _leaf(8)));
        bytes32 expected = _hash(_hash(left, right), _hash(_hash(_leaf(9), _leaf(10)), _leaf(11)));

        BeltReference.Mountains memory mountains;
        assertEq(_appendNext(mountains, 11), expected);
        assertEq(mountains.root(), expected);
    }

    function test_append_matches_the_reference_for_each_leaf_count_up_to_600() public {
        BeltReference.Mountains memory mountains;
        for (uint256 count = 1; count <= 600; ++count) {
            bytes32 appendRoot = _appendNext(mountains, 1);
            assertEq(appendRoot, mountains.root(), string.concat("count ", vm.toString(count)));
            assertEq(_belt.root(), appendRoot);
        }
    }

    function test_append_matches_the_reference_across_a_power_of_two_far_from_the_start() public {
        uint256 power = 1 << 11;
        BeltReference.Mountains memory mountains;
        while (_belt.leafCount() < power - 6) {
            _appendNext(mountains, _min(64, power - 6 - _belt.leafCount()));
        }
        for (uint256 size = 1; size <= 4; ++size) {
            bytes32 appendRoot = _appendNext(mountains, size);
            assertEq(appendRoot, mountains.root(), string.concat("count ", vm.toString(_belt.leafCount())));
            assertEq(_belt.root(), appendRoot);
        }
    }

    /// forge-config: default.fuzz.runs = 64
    function testFuzz_append_matches_the_reference_for_any_append_sizes(uint256 seed) public {
        BeltReference.Mountains memory mountains;
        while (_belt.leafCount() < 150) {
            seed = uint256(keccak256(abi.encode(seed)));
            // Sizes from 1 to 3 times the typical size, so that some appends read nodes that the belt did not store.
            uint256 size = 1 + seed % (3 * MerkleMountainBelt._TYPICAL_APPEND_SIZE);
            bytes32 appendRoot = _appendNext(mountains, size);
            assertEq(appendRoot, mountains.root(), string.concat("count ", vm.toString(_belt.leafCount())));
            assertEq(_belt.root(), appendRoot);
        }
    }

    function test_append_reverts_on_a_leaf_that_is_not_in_the_field() public {
        bytes32[] memory leaves = new bytes32[](1);
        leaves[0] = bytes32(Poseidon2T4.PRIME - 1);
        _belt.append(leaves);

        leaves[0] = bytes32(Poseidon2T4.PRIME);
        vm.expectRevert(abi.encodeWithSelector(MerkleMountainBelt.LeafNotInField.selector, leaves[0]));
        _belt.append(leaves);
    }

    function test_append_reverts_above_the_maximum_leaf_count() public {
        // The leaf count is the first field of the belt, at the slot of `_belt`.
        uint256 slot;
        // solhint-disable-next-line no-inline-assembly
        assembly {
            slot := _belt.slot
        }
        vm.store(address(this), bytes32(slot), bytes32(MerkleMountainBelt._MAX_LEAF_COUNT));

        vm.expectRevert(
            abi.encodeWithSelector(
                MerkleMountainBelt.LeafCountTooLarge.selector, MerkleMountainBelt._MAX_LEAF_COUNT + 1
            )
        );
        _belt.append(new bytes32[](1));
    }

    function test_root_of_the_empty_belt_is_zero() public view {
        assertEq(_belt.root(), bytes32(0));
    }

    /// @dev Appends the next leaves to the belt and pushes them to the mountains.
    function _appendNext(BeltReference.Mountains memory mountains, uint256 size) internal returns (bytes32 appendRoot) {
        uint256 first = _belt.leafCount() + 1;
        bytes32[] memory batch = new bytes32[](size);
        for (uint256 i = 0; i < size; ++i) {
            batch[i] = _leaf(first + i);
            mountains.push(batch[i]);
        }
        appendRoot = _belt.append(batch);
    }

    function _leaf(uint256 number) internal pure returns (bytes32 leaf) {
        leaf = bytes32(uint256(keccak256(abi.encode("leaf", number))) % Poseidon2T4.PRIME);
    }

    function _hash(bytes32 left, bytes32 right) internal pure returns (bytes32 parent) {
        parent = BeltReference.hash(left, right);
    }

    function _min(uint256 a, uint256 b) internal pure returns (uint256 smaller) {
        smaller = a < b ? a : b;
    }
}
