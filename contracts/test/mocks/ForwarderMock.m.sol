// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {Address} from "@openzeppelin-contracts-5.7.0/utils/Address.sol";
import {IForwarder} from "anoma-forwarder-bases-3.1.1/src/interfaces/IForwarder.sol";

/// @notice Calls its caller with the input and returns the output, so a test can read or reenter the caller.
contract ForwarderMock is IForwarder {
    function forwardCall(bytes32, bytes calldata input) external override returns (bytes memory output) {
        output = Address.functionCall({target: msg.sender, data: input});
    }
}
