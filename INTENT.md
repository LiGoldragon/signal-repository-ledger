# INTENT — signal-repository-ledger

*The ordinary wire vocabulary contract for the repository-ledger component.
Defines the typed request/reply channel that trusted local peers use to submit
repository receive-hook events and push observations and to query ledger state.
Companion to `ARCHITECTURE.md` and `Cargo.toml`. Maintenance: `primary/skills/repo-intent.md`.*

## Repo-scope only

This file carries only the intent that is FOR this `signal-repository-ledger`
contract. Workspace-shape intent stays in the primary workspace `primary/INTENT.md`.
Component daemon intent stays in `repository-ledger/INTENT.md`. Owner-only policy
stays in `owner-signal-repository-ledger`.

## Why this repo exists

`signal-repository-ledger` is the **ordinary peer-callable wire contract** for the
repository-ledger component. It is called by trusted local peers that submit
repository-receive notifications, submit direct push observations, or ask for
repository-ledger state. The first slice covers Gitolite receive-hook
notifications and basic read queries. Privileged repository registration, hook
policy, and mirror configuration live in `owner-signal-repository-ledger`; the
runtime store and lowering logic live in `repository-ledger`. This crate was the
**pilot contract** for the migration from public `signal-core` Sema verbs to the
three-layer model (2026-05-19/2026-05-20).

## The channel shape

The Ledger channel carries:

- **Operations:** `Receive(ReceiveHookNotification)` — a Gitolite hook fallback
  spool notification reached the daemon; `Observe(PushObservation)` — the caller
  observed a repository push and submits typed commits and file changes;
  `Query(Query)` — read ledger state, with a closed `Query` sum naming the read
  targets `Events`, `RecentRepositories`, `ChangedFiles`, `CommitMessages`, and
  `Catalog`.
- **Replies:** `EventRecorded` — a receive/observe was accepted and assigned a
  ledger sequence; `QueryResult` — a query returned one of the closed result
  payloads; `RequestUnimplemented` — the daemon accepted the contract shape but
  the runtime lacks that operation or query path (skeleton honesty).

The wire vocabulary is contract-local: there is no public `Assert` / `Match` tag.
The daemon lowers these public operations into component-local commands; Sema
classification happens at observation time, not on the wire.

## Channels are closed, boundaries are named

- Wire enums are closed. No `Unknown` escape hatch; unimplemented paths reply
  `RequestUnimplemented`.
- Request payloads do not mint the ledger sequence or commit timestamps — the
  daemon assigns the sequence on acceptance.
- `repository-ledger` mints those values at the daemon; request records carry the
  observed receive/push facts and the query target only.
- No stringly-typed dispatch. Query targets, result payloads, and reasons are
  typed closed enums.

## Wire vocabulary discipline

Per `primary/skills/contract-repo.md` §"Public contracts use contract-local
operation verbs":

- Operation roots are domain verbs in verb form: `Receive`, `Observe`, `Query` —
  not Sema class words.
- Reply success variants name the outcome: receive/observe → `EventRecorded`;
  query → `QueryResult`.
- Payload record names are domain nouns the operation carries
  (`ReceiveHookNotification`, `PushObservation`, `Query`). Per
  `primary/skills/contract-repo.md` §"Contracts name a component's wire surface",
  names do not repeat the repository-ledger namespace already supplied by the
  crate — a changed-file query is `ChangedFileQuery`, not
  `RepositoryChangedFileQuery`.
- Repository-ledger is **not** a persona component, so the mandatory `Tap`/`Untap`
  observable surface does NOT apply here. A later domain-specific observation
  channel would use domain-named subscribe/retract pairs.

## Constraints

- This crate carries only typed wire vocabulary, NOTA codecs, and round-trip
  witnesses.
- No runtime code: no actors, no tokio, no socket binding, no redb, no ledger
  reducers.
- Contract types derive NOTA in this crate. Consumers do not carry shadow types.
- Every operation and reply variant round-trips through both rkyv frames and NOTA
  text.
- The ledger sequence and event identity are minted by the daemon, never accepted
  from a caller payload.
- Privileged repository registration, hook policy, and mirror configuration stay
  in `owner-signal-repository-ledger`, not here.
- The contract is scheduled for cutover to a schema-language source (per
  `reports/designer/326-v13` + `/324`); the wire surface it defines is the input
  to that cutover.

## Three-layer model

Layer 1 (this crate): contract operations on the wire (`Receive`, `Observe`,
`Query`).
Layer 2 (daemon): component-local `LedgerCommand` enum (e.g. `RecordEvent`,
`ReadRecentRepositories`) that the daemon executes.
Layer 3 (observation): payloadless Sema class labels via `ToSemaOperation` for
cross-component introspection.

The contract names the public action at the boundary; the daemon decides what
internal work and Sema class label each action maps to. Sema classification
never appears on the wire.

## Code map

```text
src/lib.rs                                  — ReceiveHookNotification/PushObservation/Query records, NOTA codecs, signal_channel! invocation
schema/signal-repository-ledger.concept.schema — concept-schema source for the contract
tests/round_trip.rs                         — rkyv frame and NOTA round-trip witnesses per operation
```

## Non-ownership

This crate does not own:

- `repository-ledger` daemon runtime, actors, or component lifecycle;
- the ledger redb store, event tables, or repository indices;
- socket binding, transport, or version handshake policy;
- ledger reducers, hook spool processing, or registration logic;
- owner-only repository registration, hook policy, or mirror configuration (that
  is `owner-signal-repository-ledger`);
- NOTA projection policy or surface (CLI formatting, audit wrapping).

## See also

- `ARCHITECTURE.md` — detailed channel shape, the three-layer migration history,
  the pending schema-engine cutover, and closed-enum discipline.
- `../repository-ledger/INTENT.md` — daemon-side intent (schema-driven planes,
  actors, state).
- `../owner-signal-repository-ledger/INTENT.md` — owner-only repository-ledger
  policy contract.
- `primary/skills/contract-repo.md` — contract repo discipline and naming rules.
- `primary/skills/component-triad.md` — repo triad structure and wire layers.
