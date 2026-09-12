# signal-repository-ledger

Ordinary Signal contract for the repository ledger component.

The contract carries repository receive-hook notifications, direct push
observations, and the bounded read queries over the ledger. Runtime storage
lives in `repository-ledger`; privileged registration, spool and mirror policy
live in `meta-signal-repository-ledger`.

`ethos/signal.ethos` is the schema authority. `build.rs` regenerates it with
Ethos Zero and asserts the committed projection in `src/generated/signal.rs`
matches. `examples/canonical.datom` is the contract's canonical text, emitted
by the codec that reads it back.
