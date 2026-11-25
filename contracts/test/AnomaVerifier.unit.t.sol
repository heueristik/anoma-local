// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.30;

import {ReentrancyGuardTransient} from "@openzeppelin-contracts-5.7.0/utils/ReentrancyGuardTransient.sol";
import {IForwarder} from "anoma-forwarder-bases-3.1.1/src/interfaces/IForwarder.sol";
import {Test, Vm} from "forge-std-1.17.0/src/Test.sol";
import {Poseidon2T4} from "privacy-boost-protocol-62a8140/Poseidon2T4.sol";

import {AnomaVerifier} from "../src/AnomaVerifier.sol";
import {IAnomaVerifier} from "../src/interfaces/IAnomaVerifier.sol";
import {IGroth16Verifier} from "../src/interfaces/IGroth16Verifier.sol";
import {MerkleMountainBelt} from "../src/libs/MerkleMountainBelt.sol";
import {ForwarderMock} from "./mocks/ForwarderMock.m.sol";
import {Groth16VerifierMock} from "./mocks/Groth16VerifierMock.m.sol";

contract AnomaVerifierTest is Test {
    using MerkleMountainBelt for MerkleMountainBelt.Belt;

    bytes32 internal constant _CONSUMED_LOGIC_REF = keccak256("consumed logic");
    bytes32 internal constant _CREATED_LOGIC_REF = keccak256("created logic");

    AnomaVerifier internal _verifier;
    Groth16VerifierMock internal _groth16;
    ForwarderMock internal _forwarder;
    bytes32 internal _emptyRoot;

    MerkleMountainBelt.Belt internal _expectedTree;

    function setUp() public {
        _groth16 = new Groth16VerifierMock();
        _forwarder = new ForwarderMock();
        _verifier = new AnomaVerifier(address(_groth16));
        _emptyRoot = _verifier.latestCommitmentTreeRoot();
    }

    function test_constructor_reverts_on_the_zero_verifier() public {
        vm.expectRevert(AnomaVerifier.ZeroGroth16VerifierNotAllowed.selector);
        new AnomaVerifier(address(0));
    }

    function test_execute_reverts_on_the_empty_transaction() public {
        IAnomaVerifier.Transaction memory transaction;
        vm.expectRevert(AnomaVerifier.EmptyTransactionNotAllowed.selector);
        _verifier.execute(transaction);
    }

    function test_execute_passes_the_action_digest_to_the_verifier() public {
        assertLt(2 ** 253, Poseidon2T4.PRIME, "253 bits fit into the BN254 scalar field");
        IAnomaVerifier.Action memory action = _action(_tag("nullifier"), _tag("commitment"));
        IAnomaVerifier.Resource memory created = action.resources[1];
        created.appData.resourcePayload = _blobs(IAnomaVerifier.DeletionCriterion.Never, "ab");
        created.appData.applicationPayload = _blobs(IAnomaVerifier.DeletionCriterion.Immediately, "c");

        // The four payloads each start with a 4-byte count; a blob adds its criterion, its 4-byte length and its bytes.
        bytes memory consumedPayloads = abi.encodePacked(uint32(0), uint32(0), uint32(0), uint32(0));
        bytes memory createdPayloads = abi.encodePacked(
            abi.encodePacked(uint32(1), uint8(1), uint32(2), "ab"),
            uint32(0),
            uint32(0),
            abi.encodePacked(uint32(1), uint8(0), uint32(1), "c")
        );
        uint256 digest = uint256(
            sha256(
                abi.encode(
                    bytes32(0),
                    true,
                    action.resources[0].tag,
                    _CONSUMED_LOGIC_REF,
                    _emptyRoot,
                    uint256(sha256(consumedPayloads)) % 2 ** 253,
                    false,
                    created.tag,
                    _CREATED_LOGIC_REF,
                    bytes32(0),
                    uint256(sha256(createdPayloads)) % 2 ** 253
                )
            )
        ) % 2 ** 253;

        IAnomaVerifier.Transaction memory transaction = _prove(_list(action));
        vm.expectCall(
            address(_groth16),
            abi.encodeCall(IGroth16Verifier.verifyProof, (transaction.proof, [uint256(_bind(bytes32(digest)))]))
        );
        _verifier.execute(transaction);
    }

    function test_execute_chains_the_action_digest_in_the_order_of_the_actions() public {
        IAnomaVerifier.Action memory first = _action(_tag("first nullifier"), _tag("first commitment"));
        IAnomaVerifier.Action memory second = _action(_tag("second nullifier"), _tag("second commitment"));
        bytes32 digest = _extend(_extend(bytes32(0), first), second);

        IAnomaVerifier.Transaction memory transaction = _prove(_list(first, second));
        vm.expectCall(
            address(_groth16),
            abi.encodeCall(IGroth16Verifier.verifyProof, (transaction.proof, [uint256(_bind(digest))]))
        );
        _verifier.execute(transaction);
    }

    function test_execute_reverts_on_a_proof_for_another_chain() public {
        IAnomaVerifier.Transaction memory transaction = _prove(_list(_action(_tag("n"), _tag("c"))));

        vm.chainId(block.chainid + 1);
        vm.expectRevert(Groth16VerifierMock.ProofInvalid.selector);
        _verifier.execute(transaction);
    }

    function test_execute_reverts_on_a_proof_for_another_verifier() public {
        IAnomaVerifier.Transaction memory transaction = _prove(_list(_action(_tag("n"), _tag("c"))));
        AnomaVerifier other = new AnomaVerifier(address(_groth16));
        assertEq(other.latestCommitmentTreeRoot(), _emptyRoot, "the premise is that both verifiers have the same root");

        vm.expectRevert(Groth16VerifierMock.ProofInvalid.selector);
        other.execute(transaction);
    }

    function test_execute_reverts_on_a_proof_for_other_actions() public {
        IAnomaVerifier.Transaction memory transaction = _prove(_list(_action(_tag("n"), _tag("a"))));
        transaction.actions = _list(_action(_tag("n"), _tag("b")));

        vm.expectRevert(Groth16VerifierMock.ProofInvalid.selector);
        _verifier.execute(transaction);
    }

    function test_execute_emits_the_action_digest() public {
        IAnomaVerifier.Action[] memory actions = _list(_action(_tag("nullifier"), _tag("commitment")));

        vm.expectEmit(address(_verifier));
        emit IAnomaVerifier.TransactionExecuted(_extend(bytes32(0), actions[0]));
        _verifier.execute(_prove(actions));
    }

    function test_execute_adds_the_nullifiers() public {
        bytes32 nullifier = _tag("nullifier");
        _verifier.execute(_prove(_list(_action(nullifier, _tag("commitment")))));

        assertTrue(_verifier.isNullifierContained(nullifier));
    }

    function test_execute_appends_the_commitments_in_the_order_of_the_actions_and_their_resources() public {
        bytes32[] memory commitments = new bytes32[](3);
        commitments[0] = _tag("first");
        commitments[1] = _tag("second");
        commitments[2] = _tag("third");
        IAnomaVerifier.Action memory first =
            _action(_created(commitments[0]), _consumed(_tag("first nullifier")), _created(commitments[1]));

        _verifier.execute(_prove(_list(first, _action(_tag("second nullifier"), commitments[2]))));

        bytes32 expectedRoot = _expectedTree.append(commitments);
        assertEq(_verifier.latestCommitmentTreeRoot(), expectedRoot);
        assertTrue(_verifier.isCommitmentTreeRootHistorical(expectedRoot));
        assertEq(_verifier.commitmentCount(), commitments.length);
    }

    function test_execute_accepts_a_root_that_an_earlier_transaction_added() public {
        _verifier.execute(_prove(_list(_action(_tag("first nullifier"), _tag("commitment")))));
        IAnomaVerifier.Action memory action = _action(_tag("second nullifier"), _tag("other"));
        action.resources[0].commitmentTreeRoot = _verifier.latestCommitmentTreeRoot();

        _verifier.execute(_prove(_list(action)));
        assertTrue(_verifier.isNullifierContained(action.resources[0].tag));
    }

    function test_execute_reverts_on_a_root_that_is_not_historical() public {
        IAnomaVerifier.Action memory action = _action(_tag("nullifier"), _tag("commitment"));
        bytes32 root = _tag("not a root");
        action.resources[0].commitmentTreeRoot = root;

        IAnomaVerifier.Transaction memory transaction = _prove(_list(action));
        vm.expectRevert(abi.encodeWithSelector(AnomaVerifier.NonExistingRoot.selector, root));
        _verifier.execute(transaction);
    }

    function test_execute_reverts_on_a_nullifier_that_an_earlier_transaction_added() public {
        bytes32 nullifier = _tag("nullifier");
        _verifier.execute(_prove(_list(_action(nullifier, _tag("commitment")))));

        IAnomaVerifier.Transaction memory transaction = _prove(_list(_action(nullifier, _tag("other"))));
        vm.expectRevert(abi.encodeWithSelector(AnomaVerifier.PreExistingNullifier.selector, nullifier));
        _verifier.execute(transaction);
    }

    function test_execute_reverts_on_a_nullifier_that_occurs_twice_in_the_transaction() public {
        bytes32 nullifier = _tag("nullifier");
        IAnomaVerifier.Transaction memory transaction =
            _prove(_list(_action(nullifier, _tag("first")), _action(nullifier, _tag("second"))));

        vm.expectRevert(abi.encodeWithSelector(AnomaVerifier.PreExistingNullifier.selector, nullifier));
        _verifier.execute(transaction);
    }

    function test_execute_reverts_on_a_nullifier_that_occurs_twice_in_one_action() public {
        bytes32 nullifier = _tag("nullifier");
        IAnomaVerifier.Transaction memory transaction =
            _prove(_list(_action(_consumed(nullifier), _consumed(nullifier), _created(_tag("commitment")))));

        vm.expectRevert(abi.encodeWithSelector(AnomaVerifier.PreExistingNullifier.selector, nullifier));
        _verifier.execute(transaction);
    }

    function test_execute_emits_the_tags_and_logic_refs_of_each_action_in_the_order_of_its_resources() public {
        IAnomaVerifier.Action memory action = _action(
            _consumed(_tag("first nullifier")), _created(_tag("commitment")), _consumed(_tag("second nullifier"))
        );
        action.resources[2].logicRef = _CREATED_LOGIC_REF;
        bytes32[] memory nullifiers = new bytes32[](2);
        nullifiers[0] = action.resources[0].tag;
        nullifiers[1] = action.resources[2].tag;
        bytes32[] memory consumedLogicRefs = new bytes32[](2);
        consumedLogicRefs[0] = _CONSUMED_LOGIC_REF;
        consumedLogicRefs[1] = _CREATED_LOGIC_REF;

        vm.expectEmit(address(_verifier));
        emit IAnomaVerifier.ActionExecuted({
            nullifiers: nullifiers,
            consumedLogicRefs: consumedLogicRefs,
            commitments: _words(action.resources[1].tag),
            createdLogicRefs: _words(_CREATED_LOGIC_REF)
        });
        _verifier.execute(_prove(_list(action)));
    }

    function test_execute_calls_the_forwarder_with_the_logic_ref_of_the_carrier_resource() public {
        bytes memory input = abi.encodeCall(IAnomaVerifier.commitmentCount, ());
        bytes memory output = abi.encode(uint256(1));
        IAnomaVerifier.Action memory action = _action(_tag("nullifier"), _tag("commitment"));
        action.resources[1].appData.externalPayload = _call(abi.encode(address(_forwarder), input, output));

        vm.expectCall(address(_forwarder), abi.encodeCall(IForwarder.forwardCall, (_CREATED_LOGIC_REF, input)));
        vm.expectEmit(address(_verifier));
        emit IAnomaVerifier.ForwarderCallExecuted(address(_forwarder), input, output);
        _verifier.execute(_prove(_list(action)));
    }

    function test_execute_makes_the_forwarder_calls_after_the_state_writes() public {
        IAnomaVerifier.Action memory action = _action(_tag("nullifier"), _tag("commitment"));
        bytes memory input = abi.encodeCall(IAnomaVerifier.isNullifierContained, (action.resources[0].tag));
        action.resources[0].appData.externalPayload = _call(abi.encode(address(_forwarder), input, abi.encode(true)));

        _verifier.execute(_prove(_list(action)));
    }

    function test_execute_makes_the_forwarder_calls_in_the_order_of_the_actions_and_their_resources() public {
        bytes[] memory inputs = new bytes[](3);
        IAnomaVerifier.Action memory first =
            _action(_created(_tag("first commitment")), _consumed(_tag("nullifier")), _created(_tag("second")));
        IAnomaVerifier.Action memory second = _action(_tag("second nullifier"), _tag("third commitment"));
        IAnomaVerifier.Resource[3] memory carriers = [first.resources[0], first.resources[1], second.resources[0]];
        for (uint256 i = 0; i < inputs.length; ++i) {
            inputs[i] = abi.encodeCall(IAnomaVerifier.isNullifierContained, (_tag(vm.toString(i))));
            carriers[i].appData.externalPayload = _call(abi.encode(address(_forwarder), inputs[i], abi.encode(false)));
        }

        vm.recordLogs();
        _verifier.execute(_prove(_list(first, second)));

        Vm.Log[] memory logs = vm.getRecordedLogs();
        uint256 callCount = 0;
        for (uint256 i = 0; i < logs.length; ++i) {
            if (logs[i].topics[0] != IAnomaVerifier.ForwarderCallExecuted.selector) continue;
            (bytes memory input,) = abi.decode(logs[i].data, (bytes, bytes));
            assertEq(input, inputs[callCount]);
            ++callCount;
        }
        assertEq(callCount, inputs.length);
    }

    function test_execute_reverts_on_a_forwarder_output_mismatch() public {
        bytes memory expected = abi.encode(true);
        bytes memory actual = abi.encode(false);
        bytes memory input = abi.encodeCall(IAnomaVerifier.isNullifierContained, (_tag("absent")));
        IAnomaVerifier.Action memory action = _action(_tag("nullifier"), _tag("commitment"));
        action.resources[1].appData.externalPayload = _call(abi.encode(address(_forwarder), input, expected));
        IAnomaVerifier.Transaction memory transaction = _prove(_list(action));

        vm.expectRevert(abi.encodeWithSelector(AnomaVerifier.ForwarderCallOutputMismatch.selector, expected, actual));
        _verifier.execute(transaction);
    }

    function test_execute_reverts_on_a_reentrant_call_from_a_forwarder() public {
        IAnomaVerifier.Transaction memory inner;
        bytes memory input = abi.encodeCall(IAnomaVerifier.execute, (inner));
        IAnomaVerifier.Action memory action = _action(_tag("nullifier"), _tag("commitment"));
        action.resources[1].appData.externalPayload = _call(abi.encode(address(_forwarder), input, ""));
        IAnomaVerifier.Transaction memory transaction = _prove(_list(action));

        vm.expectRevert(ReentrancyGuardTransient.ReentrancyGuardReentrantCall.selector);
        _verifier.execute(transaction);
    }

    function test_execute_emits_only_the_blobs_that_are_never_deleted() public {
        IAnomaVerifier.Action memory action = _action(_tag("nullifier"), _tag("commitment"));
        IAnomaVerifier.ExpirableBlob[] memory payload = new IAnomaVerifier.ExpirableBlob[](2);
        payload[0] = _blob(IAnomaVerifier.DeletionCriterion.Immediately, "deleted");
        payload[1] = _blob(IAnomaVerifier.DeletionCriterion.Never, "kept");
        action.resources[1].appData.resourcePayload = payload;

        vm.recordLogs();
        _verifier.execute(_prove(_list(action)));

        Vm.Log[] memory logs = vm.getRecordedLogs();
        uint256 payloadLogCount = 0;
        for (uint256 i = 0; i < logs.length; ++i) {
            if (logs[i].topics[0] != IAnomaVerifier.ResourcePayload.selector) continue;
            ++payloadLogCount;
            assertEq(logs[i].topics[1], action.resources[1].tag);
            assertEq(logs[i].data, abi.encode(uint256(1), bytes("kept")));
        }
        assertEq(payloadLogCount, 1);
    }

    function test_constructor_adds_the_empty_root_to_the_historical_roots() public view {
        assertEq(_emptyRoot, _expectedTree.root(), "the latest root is the root of the empty belt");
        assertTrue(_verifier.isCommitmentTreeRootHistorical(_emptyRoot));
        assertEq(_verifier.commitmentCount(), 0);
    }

    /// @dev An action that consumes the resource of `nullifier` and creates the resource of `commitment`.
    function _action(bytes32 nullifier, bytes32 commitment)
        internal
        view
        returns (IAnomaVerifier.Action memory action)
    {
        action.resources = new IAnomaVerifier.Resource[](2);
        action.resources[0] = _consumed(nullifier);
        action.resources[1] = _created(commitment);
    }

    /// @dev A consumed resource that shows the root of the empty commitment tree.
    function _consumed(bytes32 nullifier) internal view returns (IAnomaVerifier.Resource memory resource) {
        resource.isConsumed = true;
        resource.tag = nullifier;
        resource.logicRef = _CONSUMED_LOGIC_REF;
        resource.commitmentTreeRoot = _emptyRoot;
    }

    /// @dev Returns a transaction whose proof the mock verifier accepts for its action digest, bound to `_verifier`.
    function _prove(IAnomaVerifier.Action[] memory actions)
        internal
        view
        returns (IAnomaVerifier.Transaction memory transaction)
    {
        bytes32 digest = bytes32(0);
        for (uint256 i = 0; i < actions.length; ++i) {
            digest = _extend(digest, actions[i]);
        }
        transaction = IAnomaVerifier.Transaction({actions: actions, proof: abi.encode(_bind(digest))});
    }

    /// @dev The action digest bound to the chain and to `_verifier`, written apart from the contract's library.
    function _bind(bytes32 digest) internal view returns (bytes32 bound) {
        bound = _low253Bits(sha256(abi.encode(block.chainid, address(_verifier), digest)));
    }

    function _action(
        IAnomaVerifier.Resource memory first,
        IAnomaVerifier.Resource memory second,
        IAnomaVerifier.Resource memory third
    ) internal pure returns (IAnomaVerifier.Action memory action) {
        action.resources = new IAnomaVerifier.Resource[](3);
        action.resources[0] = first;
        action.resources[1] = second;
        action.resources[2] = third;
    }

    function _created(bytes32 commitment) internal pure returns (IAnomaVerifier.Resource memory resource) {
        resource.tag = commitment;
        resource.logicRef = _CREATED_LOGIC_REF;
    }

    /// @dev The action digest, written apart from the contract's library.
    function _extend(bytes32 digest, IAnomaVerifier.Action memory action) internal pure returns (bytes32 extended) {
        bytes memory packed = abi.encode(digest);
        for (uint256 i = 0; i < action.resources.length; ++i) {
            IAnomaVerifier.Resource memory resource = action.resources[i];
            packed = abi.encodePacked(
                packed,
                abi.encode(
                    resource.isConsumed,
                    resource.tag,
                    resource.logicRef,
                    resource.commitmentTreeRoot,
                    _appDataHash(resource.appData)
                )
            );
        }
        extended = _low253Bits(sha256(packed));
    }

    function _appDataHash(IAnomaVerifier.AppData memory appData) internal pure returns (bytes32 hash) {
        hash = _low253Bits(
            sha256(
                abi.encodePacked(
                    _pack(appData.resourcePayload),
                    _pack(appData.discoveryPayload),
                    _pack(appData.externalPayload),
                    _pack(appData.applicationPayload)
                )
            )
        );
    }

    function _pack(IAnomaVerifier.ExpirableBlob[] memory blobs) internal pure returns (bytes memory packed) {
        packed = abi.encodePacked(uint32(blobs.length));
        for (uint256 i = 0; i < blobs.length; ++i) {
            packed = abi.encodePacked(
                packed, uint8(blobs[i].deletionCriterion), uint32(blobs[i].blob.length), blobs[i].blob
            );
        }
    }

    function _low253Bits(bytes32 hash) internal pure returns (bytes32 low) {
        low = bytes32(uint256(hash) % 2 ** 253);
    }

    function _list(IAnomaVerifier.Action memory first) internal pure returns (IAnomaVerifier.Action[] memory actions) {
        actions = new IAnomaVerifier.Action[](1);
        actions[0] = first;
    }

    function _list(IAnomaVerifier.Action memory first, IAnomaVerifier.Action memory second)
        internal
        pure
        returns (IAnomaVerifier.Action[] memory actions)
    {
        actions = new IAnomaVerifier.Action[](2);
        actions[0] = first;
        actions[1] = second;
    }

    function _words(bytes32 word) internal pure returns (bytes32[] memory words) {
        words = new bytes32[](1);
        words[0] = word;
    }

    /// @dev A tag is a field element, because the circuits compute it.
    function _tag(string memory seed) internal pure returns (bytes32 tag) {
        tag = bytes32(uint256(keccak256(bytes(seed))) % Poseidon2T4.PRIME);
    }

    function _blob(IAnomaVerifier.DeletionCriterion deletionCriterion, bytes memory data)
        internal
        pure
        returns (IAnomaVerifier.ExpirableBlob memory blob)
    {
        blob = IAnomaVerifier.ExpirableBlob({deletionCriterion: deletionCriterion, blob: data});
    }

    function _blobs(IAnomaVerifier.DeletionCriterion deletionCriterion, bytes memory data)
        internal
        pure
        returns (IAnomaVerifier.ExpirableBlob[] memory blobs)
    {
        blobs = new IAnomaVerifier.ExpirableBlob[](1);
        blobs[0] = _blob(deletionCriterion, data);
    }

    function _call(bytes memory callBlob) internal pure returns (IAnomaVerifier.ExpirableBlob[] memory blobs) {
        blobs = _blobs(IAnomaVerifier.DeletionCriterion.Immediately, callBlob);
    }
}
