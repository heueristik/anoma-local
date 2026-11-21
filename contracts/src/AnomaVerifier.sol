// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {ReentrancyGuardTransient} from "@openzeppelin-contracts-5.7.0/utils/ReentrancyGuardTransient.sol";
import {IForwarder} from "anoma-forwarder-bases-3.1.1/src/interfaces/IForwarder.sol";

import {IAnomaVerifier} from "./interfaces/IAnomaVerifier.sol";
import {IGroth16Verifier} from "./interfaces/IGroth16Verifier.sol";
import {ActionDigest} from "./libs/ActionDigest.sol";
import {MerkleMountainBelt} from "./libs/MerkleMountainBelt.sol";

/// @title AnomaVerifier
/// @author Heueristik, 2026
/// @notice The immutable contract that verifies resource machine transactions with one Groth16 proof and executes
/// them.
/// @dev The final circuit checks the balance of each transaction, so the contract checks no delta.
/// @custom:security-contact security@anoma.foundation
contract AnomaVerifier is IAnomaVerifier, ReentrancyGuardTransient {
    using MerkleMountainBelt for MerkleMountainBelt.Belt;

    /// @inheritdoc IAnomaVerifier
    address public immutable override GROTH16_VERIFIER;

    /// @notice The commitments of the created resources, in the order of their transactions and actions.
    MerkleMountainBelt.Belt internal _commitmentTree;

    mapping(bytes32 root => bool isHistorical) internal _historicalRoots;

    mapping(bytes32 nullifier => bool isContained) internal _nullifiers;

    /// @notice Thrown if the constructor gets the zero address as the Groth16 verifier.
    error ZeroGroth16VerifierNotAllowed();

    /// @notice Thrown if a transaction has no actions.
    error EmptyTransactionNotAllowed();

    /// @notice Thrown if a consumed resource uses a root that is not a historical commitment tree root.
    /// @param root The root.
    error NonExistingRoot(bytes32 root);

    /// @notice Thrown if the nullifier set already contains the nullifier of a consumed resource.
    /// @param nullifier The nullifier.
    error PreExistingNullifier(bytes32 nullifier);

    /// @notice Thrown if the output of a forwarder call is not the expected output of its external payload blob.
    /// @param expected The expected output.
    /// @param actual The output of the call.
    error ForwarderCallOutputMismatch(bytes expected, bytes actual);

    /// @notice Sets the Groth16 verifier and adds the root of the empty commitment tree to the historical roots.
    /// @param groth16Verifier The verifier of the final circuit, which proves the compliance units, the resource logics
    /// and the balance of a transaction.
    constructor(address groth16Verifier) {
        if (groth16Verifier == address(0)) revert ZeroGroth16VerifierNotAllowed();
        GROTH16_VERIFIER = groth16Verifier;
        _addCommitmentTreeRoot(_commitmentTree.root());
    }

    /// @inheritdoc IAnomaVerifier
    function execute(Transaction calldata transaction) external override nonReentrant {
        Action[] calldata actions = transaction.actions;
        uint256 actionCount = actions.length;
        if (actionCount == 0) revert EmptyTransactionNotAllowed();

        bytes32 actionDigest = bytes32(0);
        bytes32[] memory commitments = new bytes32[](actionCount);
        for (uint256 i = 0; i < actionCount; ++i) {
            Action calldata action = actions[i];
            _processAction(action);
            // The compliance circuit makes the commitments unique, since it derives them from unique nullifiers.
            commitments[i] = action.created.tag;
            actionDigest = ActionDigest.extend({digest: actionDigest, action: action});
        }

        IGroth16Verifier(GROTH16_VERIFIER).verifyProof({proof: transaction.proof, input: [uint256(actionDigest)]});

        _addCommitmentTreeRoot(_commitmentTree.append(commitments));

        _executeForwarderCalls(actions);

        // NOTE: The `nonReentrant` modifier keeps the events of other transactions out of this transaction.
        // forge-lint: disable-next-item(reentrancy-events)
        // slither-disable-next-line reentrancy-events
        emit TransactionExecuted({actionDigest: actionDigest});
    }

    /// @inheritdoc IAnomaVerifier
    function commitmentCount() external view override returns (uint256 count) {
        count = _commitmentTree.leafCount();
    }

    /// @inheritdoc IAnomaVerifier
    function isCommitmentTreeRootHistorical(bytes32 root) external view override returns (bool isHistorical) {
        isHistorical = _historicalRoots[root];
    }

    /// @inheritdoc IAnomaVerifier
    function isNullifierContained(bytes32 nullifier) external view override returns (bool isContained) {
        isContained = _nullifiers[nullifier];
    }

    /// @inheritdoc IAnomaVerifier
    function latestCommitmentTreeRoot() external view override returns (bytes32 root) {
        root = _commitmentTree.root();
    }

    /// @notice Checks the root of an action, adds its nullifier, and emits its events.
    /// @param action The action.
    function _processAction(Action calldata action) internal {
        bytes32 root = action.commitmentTreeRoot;
        // NOTE: One invalid action must revert the whole transaction.
        // forge-lint: disable-next-item(require-revert-in-loop)
        if (!_historicalRoots[root]) revert NonExistingRoot(root);

        bytes32 nullifier = action.consumed.tag;
        // forge-lint: disable-next-item(require-revert-in-loop)
        if (_nullifiers[nullifier]) revert PreExistingNullifier(nullifier);
        _nullifiers[nullifier] = true;

        _emitAppDataBlobs(action.consumed);
        _emitAppDataBlobs(action.created);
        emit ActionExecuted({
            nullifier: nullifier,
            commitment: action.created.tag,
            consumedLogicRef: action.consumed.logicRef,
            createdLogicRef: action.created.logicRef
        });
    }

    /// @notice Makes the forwarder calls of all resources, in the order of the actions, the consumed resource first.
    /// @param actions The actions of the transaction.
    function _executeForwarderCalls(Action[] calldata actions) internal {
        uint256 actionCount = actions.length;
        for (uint256 i = 0; i < actionCount; ++i) {
            _executeForwarderCalls(actions[i].consumed);
            _executeForwarderCalls(actions[i].created);
        }
    }

    /// @notice Makes the forwarder calls of one resource, in the order of its external payload.
    /// @param resource The resource that carries the calls.
    function _executeForwarderCalls(ResourceInstance calldata resource) internal {
        ExpirableBlob[] calldata calls = resource.appData.externalPayload;
        uint256 callCount = calls.length;
        for (uint256 i = 0; i < callCount; ++i) {
            _executeForwarderCall({carrierLogicRef: resource.logicRef, callBlob: calls[i].blob});
        }
    }

    /// @notice Calls an untrusted forwarder and checks its output.
    /// @param carrierLogicRef The logic reference of the resource that carries the call, which the forwarder checks.
    /// @param callBlob The ABI-encoded forwarder, input and expected output.
    function _executeForwarderCall(bytes32 carrierLogicRef, bytes calldata callBlob) internal {
        (address untrustedForwarder, bytes memory input, bytes memory expectedOutput) =
            abi.decode(callBlob, (address, bytes, bytes));

        // forge-lint: disable-next-item(calls-loop)
        bytes memory actualOutput =
            IForwarder(untrustedForwarder).forwardCall({logicRef: carrierLogicRef, input: input});

        // forge-lint: disable-next-item(require-revert-in-loop)
        if (keccak256(actualOutput) != keccak256(expectedOutput)) {
            revert ForwarderCallOutputMismatch({expected: expectedOutput, actual: actualOutput});
        }

        // NOTE: The `nonReentrant` modifier on `execute` keeps the events of other transactions out of this one.
        // forge-lint: disable-next-item(reentrancy-events)
        // slither-disable-next-line reentrancy-events
        emit ForwarderCallExecuted({untrustedForwarder: untrustedForwarder, input: input, output: actualOutput});
    }

    /// @notice Emits the blobs of a resource whose deletion criterion is `Never`.
    /// @param resource The resource.
    function _emitAppDataBlobs(ResourceInstance calldata resource) internal {
        bytes32 tag = resource.tag;
        ExpirableBlob[] calldata payload = resource.appData.resourcePayload;
        uint256 blobCount = payload.length;
        for (uint256 i = 0; i < blobCount; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                emit ResourcePayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }

        payload = resource.appData.discoveryPayload;
        blobCount = payload.length;
        for (uint256 i = 0; i < blobCount; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                emit DiscoveryPayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }

        payload = resource.appData.externalPayload;
        blobCount = payload.length;
        for (uint256 i = 0; i < blobCount; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                emit ExternalPayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }

        payload = resource.appData.applicationPayload;
        blobCount = payload.length;
        for (uint256 i = 0; i < blobCount; ++i) {
            if (payload[i].deletionCriterion == DeletionCriterion.Never) {
                emit ApplicationPayload({tag: tag, index: i, blob: payload[i].blob});
            }
        }
    }

    /// @notice Adds a root to the set of historical commitment tree roots.
    /// @param root The root.
    function _addCommitmentTreeRoot(bytes32 root) internal {
        _historicalRoots[root] = true;
        emit CommitmentTreeRootAdded({root: root});
    }
}
