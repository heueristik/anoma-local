// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {Test} from "forge-std-1.17.0/src/Test.sol";

import {IAnomaVerifier} from "../../src/interfaces/IAnomaVerifier.sol";
import {ActionDigest} from "../../src/libs/ActionDigest.sol";

/// @dev Passes a memory action to `ActionDigest.extend`, which takes calldata, through an external call.
library ActionDigestCalldata {
    function extend(bytes32 digest, IAnomaVerifier.Action calldata action) external pure returns (bytes32 extended) {
        extended = ActionDigest.extend({digest: digest, action: action});
    }
}

contract ActionDigestTest is Test {
    function test_extend_matches_the_circuits() public pure {
        IAnomaVerifier.Action memory action;
        action.resources = new IAnomaVerifier.Resource[](2);
        action.resources[0] = IAnomaVerifier.Resource({
            isConsumed: true,
            tag: bytes32(uint256(1)),
            logicRef: bytes32(uint256(2)),
            commitmentTreeRoot: bytes32(uint256(3)),
            appData: _emptyAppData()
        });
        action.resources[1] = IAnomaVerifier.Resource({
            isConsumed: false,
            tag: bytes32(uint256(4)),
            logicRef: bytes32(uint256(5)),
            commitmentTreeRoot: bytes32(0),
            appData: _emptyAppData()
        });

        // The value of `extend` in the Noir library of the circuits for this action from the digest zero.
        assertEq(
            ActionDigestCalldata.extend({digest: bytes32(0), action: action}),
            0x0b60bead17f79a743fd35dc5f1d9948be77dceede95f75c6a7de00414a41138d
        );
    }

    function test_bind_matches_the_circuits() public pure {
        // The value of `bind` in the Noir library of the circuits for the digest 3, the chain ID 1 and the address 2.
        assertEq(
            ActionDigest.bind({digest: bytes32(uint256(3)), chainId: 1, verifier: address(2)}),
            0x1701f34c80e1ef7f8125e5d4d2d7e19b509e25d26e462d5308b5abb95b64783e
        );
    }

    function _emptyAppData() internal pure returns (IAnomaVerifier.AppData memory appData) {
        appData = IAnomaVerifier.AppData({
            resourcePayload: new IAnomaVerifier.ExpirableBlob[](0),
            discoveryPayload: new IAnomaVerifier.ExpirableBlob[](0),
            externalPayload: new IAnomaVerifier.ExpirableBlob[](0),
            applicationPayload: new IAnomaVerifier.ExpirableBlob[](0)
        });
    }
}
