# Anoma local — glossary

## Resources and actions

- **Resource**: an immutable record of eight fields: logic ref, label ref, value ref, quantity, nonce, nullifier key commitment, the ephemeral flag and a random seed. A transaction consumes resources and creates resources.
- **Persistent resource**: a resource whose commitment is in the commitment tree. Its owner consumes it with a membership path.
- **Ephemeral resource**: a resource that a transaction creates and consumes in the same transaction. Its consumption needs no membership path.
- **Commitment**: the hash of a created resource, which the contract adds to the commitment tree.
- **Nullifier**: the hash of a consumed resource and its nullifier key. The contract records it, so no one can consume the resource again.
- **Nullifier key**: the secret that opens the nullifier of a resource. The resource holds its commitment.
- **Logic ref**: the key hash of the logic circuit of a resource. A leaf proof of that circuit must exist for each resource of a transaction.
- **Label ref**: the hash of the label of a resource. The logic ref and the label ref together give the **kind**.
- **Kind base**: a Grumpkin point, one for each kind, with no known discrete logarithm.
- **Action**: one consumed and one created resource, with their app data and the commitment tree root of the membership path.
- **Delta**: the Grumpkin point of an action: the consumed quantity times its kind base, minus the created quantity times its kind base, plus the rcv times the generator.
- **rcv**: the random scalar of an action that hides its quantities in the delta.

## Proofs

- **Leaf proof**: a ZK UltraHonk proof of the compliance circuit or of a resource logic. Each action has three: compliance, the logic of the consumed resource and the logic of the created resource.
- **Partial transaction**: a list of proved actions and the sum of their rcv values. Partial transactions compose by joining both lists and adding the sums.
- **Balanced**: a partial transaction whose sum of deltas is the generator times its sum of rcv values. Then the quantities of each kind balance.
- **Step proof**: a non-ZK UltraHonk proof of `first_step` or `next_step`. It verifies the leaf proofs of one action and, for `next_step`, the previous step proof.
- **Root proof**: the proof of `root`. It verifies the last step proof and checks that the transaction is balanced.
- **Groth16 wrap**: the gnark circuit that verifies the root proof. Its Groth16 proof is the one proof that the contract checks.
- **Action digest**: the hash chain over the actions of a transaction. The step circuits compute it, and the contract computes it again from the transaction.

## Commitment tree

- **Merkle Mountain Belt**: the commitment tree, a list of perfect binary trees under one Poseidon2 root.
- **Membership path**: the hashes from a commitment to a root. The compliance circuit takes every path at the maximum length of 67 hashes, so the length does not show the age of the resource.
- **Historical root**: a root that the contract had at some time, from the root of the empty tree on. A membership path must lead to one. The action of an ephemeral resource shows a recent one, so that its root does not show that the resource is ephemeral.

## Intents and solvers

- **Intent**: an ephemeral resource whose label is the hash of its terms. Its creator gives up what the action consumes. The action that consumes the intent must create the resource of its terms.
- **Terms**: each field of the resource that the creator of an intent wants, except the nonce: the kind, the value ref, the quantity, the nullifier key commitment of the receiver and the random seed. The nonce comes from the nullifier of the intent, so the receiver can compute the resource without a payload.
- **Payload**: the data that a solver needs to fill an intent: the intent and its nullifier key, the terms, the offer, the rcv sum and the keys of the receiver. The creator encrypts it to the solver.
- **Discovery**: how a receiver finds its resources, as in Anoma Galileo. The creator adds a discovery blob and the encrypted plaintext of the resource to its app data. The receiver tries to decrypt every discovery blob.
- **Intent pool**: a list of unbalanced partial transactions. The pool checks nothing. A solver checks the payload and the actions of an intent before it fills it.
- **Solver**: a party that finds intents by discovery, chooses with CP-SAT the most intents that balance together, and proves the actions that fill them.
- **Surplus**: what is left of a kind after the solver fills the chosen intents. The solver creates it for itself from an ephemeral resource of quantity 0.
