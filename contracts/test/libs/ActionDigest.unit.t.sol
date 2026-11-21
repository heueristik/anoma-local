// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {ActionDigest} from "../../src/libs/ActionDigest.sol";

contract ActionDigestTest is Test {
    function test_bind_matches_the_circuits() public pure {
        // The value of `bind` in the Noir library of the circuits for the digest 3, the chain ID 1 and the address 2.
        assertEq(
            ActionDigest.bind({digest: bytes32(uint256(3)), chainId: 1, verifier: address(2)}),
            0x1701f34c80e1ef7f8125e5d4d2d7e19b509e25d26e462d5308b5abb95b64783e
        );
    }
}
