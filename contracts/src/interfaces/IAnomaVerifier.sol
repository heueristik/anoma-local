// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

/// @title IAnomaVerifier
/// @author Heueristik, 2026
/// @notice The interface of the contract that verifies resource machine transactions with one Groth16 proof and
/// executes them.
/// @custom:security-contact security@anoma.foundation
interface IAnomaVerifier {
    /// @notice The deletion criteria of a blob.
    enum DeletionCriterion {
        Immediately,
        Never
    }

    /// @notice A transaction: its actions and the proof of its compliance units, its resource logics and its balance.
    /// @param actions The actions, in the order in which the final circuit chains their action digest.
    /// @param proof The Groth16 proof of the final circuit, whose public input is the action digest of `actions`.
    struct Transaction {
        Action[] actions;
        bytes proof;
    }

    /// @notice An action: one compliance unit, which consumes one resource and creates one resource.
    /// @param consumed The consumed resource, whose tag is its nullifier.
    /// @param created The created resource, whose tag is its commitment.
    /// @param commitmentTreeRoot The historical commitment tree root that the membership proof of the consumed resource
    /// uses.
    struct Action {
        ResourceInstance consumed;
        ResourceInstance created;
        bytes32 commitmentTreeRoot;
    }

    /// @notice The public data of one resource.
    /// @param tag The nullifier of a consumed resource or the commitment of a created resource.
    /// @param logicRef The hash of the verifying key of the resource logic.
    /// @param appData The application data of the resource.
    struct ResourceInstance {
        bytes32 tag;
        bytes32 logicRef;
        AppData appData;
    }

    /// @notice The application data of a resource, as four lists of blobs.
    /// @param resourcePayload Blobs with the plaintext data of resources.
    /// @param discoveryPayload Blobs that let the receiver find the resource.
    /// @param externalPayload Blobs that each encode one forwarder call: the forwarder, the input and the expected
    /// output.
    /// @param applicationPayload Blobs for the application.
    struct AppData {
        ExpirableBlob[] resourcePayload;
        ExpirableBlob[] discoveryPayload;
        ExpirableBlob[] externalPayload;
        ExpirableBlob[] applicationPayload;
    }

    /// @notice A blob and its deletion criterion.
    /// @param deletionCriterion Whether the verifier emits the blob in an event (`Never`) or not (`Immediately`).
    /// @param blob The blob.
    struct ExpirableBlob {
        DeletionCriterion deletionCriterion;
        bytes blob;
    }

    /// @notice Emitted when a transaction is executed.
    /// @param actionDigest The action digest of the transaction, which is the public input of its proof.
    event TransactionExecuted(bytes32 indexed actionDigest);

    /// @notice Emitted when an action is executed.
    /// @param nullifier The nullifier of the consumed resource.
    /// @param commitment The commitment of the created resource.
    /// @param consumedLogicRef The logic reference of the consumed resource.
    /// @param createdLogicRef The logic reference of the created resource.
    event ActionExecuted(
        bytes32 indexed nullifier, bytes32 indexed commitment, bytes32 consumedLogicRef, bytes32 createdLogicRef
    );

    /// @notice Emitted when a forwarder call is executed.
    /// @param untrustedForwarder The forwarder contract that made the call.
    /// @param input The input of the call.
    /// @param output The output of the call, which is equal to the expected output.
    event ForwarderCallExecuted(address indexed untrustedForwarder, bytes input, bytes output);

    /// @notice Emitted when a root is added to the set of historical commitment tree roots.
    /// @param root The root.
    event CommitmentTreeRootAdded(bytes32 root);

    /// @notice Emitted for a resource payload blob whose deletion criterion is `Never`.
    /// @param tag The tag of the resource that the blob belongs to.
    /// @param index The index of the blob in the payload.
    /// @param blob The blob.
    event ResourcePayload(bytes32 indexed tag, uint256 index, bytes blob);

    /// @notice Emitted for a discovery payload blob whose deletion criterion is `Never`.
    /// @param tag The tag of the resource that the blob belongs to.
    /// @param index The index of the blob in the payload.
    /// @param blob The blob.
    event DiscoveryPayload(bytes32 indexed tag, uint256 index, bytes blob);

    /// @notice Emitted for an external payload blob whose deletion criterion is `Never`.
    /// @param tag The tag of the resource that the blob belongs to.
    /// @param index The index of the blob in the payload.
    /// @param blob The blob.
    event ExternalPayload(bytes32 indexed tag, uint256 index, bytes blob);

    /// @notice Emitted for an application payload blob whose deletion criterion is `Never`.
    /// @param tag The tag of the resource that the blob belongs to.
    /// @param index The index of the blob in the payload.
    /// @param blob The blob.
    event ApplicationPayload(bytes32 indexed tag, uint256 index, bytes blob);

    /// @notice Adds the nullifiers, verifies the proof against the action digest, appends the commitments to the
    /// commitment tree, and then makes the forwarder calls.
    /// @param transaction The transaction.
    function execute(Transaction calldata transaction) external;

    /// @notice Returns the number of commitments in the commitment tree.
    /// @return count The number of commitments.
    function commitmentCount() external view returns (uint256 count);

    /// @notice Returns whether a consumed resource can use a root in its membership proof.
    /// @param root The root to look up.
    /// @return isHistorical Whether the commitment tree had the root after the deployment or after a transaction.
    function isCommitmentTreeRootHistorical(bytes32 root) external view returns (bool isHistorical);

    /// @notice Returns whether a transaction consumed the resource of a nullifier.
    /// @param nullifier The nullifier to look up.
    /// @return isContained Whether the nullifier set contains the nullifier.
    function isNullifierContained(bytes32 nullifier) external view returns (bool isContained);

    /// @notice Returns the root of the commitment tree.
    /// @return root The root after the last transaction that created resources.
    function latestCommitmentTreeRoot() external view returns (bytes32 root);

    // solhint-disable func-name-mixedcase

    /// @notice The Groth16 verifier of the final circuit.
    /// @return verifier The verifier contract.
    function GROTH16_VERIFIER() external view returns (address verifier);

    // solhint-enable func-name-mixedcase
}
