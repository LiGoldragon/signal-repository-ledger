# signal-repository-ledger Architecture

`signal-repository-ledger` is the ordinary Signal contract for the
`repository-ledger` component: the peer-callable wire for repository event
submissions and bounded reads over the ledger.

Trusted local peers submit repository receive-hook notifications and direct
push observations, or ask for ledger state. Privileged repository
registration, spool policy and mirror configuration live in
`meta-signal-repository-ledger`.

## Authority

`ethos/signal.ethos` is the schema authority. Ethos Zero generates
`src/generated/signal.rs` from it, the generated file is committed, and
`build.rs` asserts the committed projection against a fresh generation on
every build. No contract type is hand-written.

`examples/canonical.datom` is the contract's canonical text. Every line was
emitted by the codec that reads it back, and `tests/generated_contract.rs`
actualizes each line and asserts it re-renders to itself.

The portable rkyv frame, its three kinds and its four-byte big-endian length
prefix come from `signal`. One request is one frame carrying the rkyv archive
of `Query`; one reply is one frame of `Response`. There is no envelope, no
route and no contract seat.

## Request surface

- `Receive(ReceiveHookNotification)` — a Gitolite hook fallback spool
  notification reached the ordinary daemon.
- `Observe(PushObservation)` — the caller observed a repository push and
  submits the typed commits and file changes.
- `Query(LedgerQuery)` — the caller asks for ledger state. `LedgerQuery` is a
  closed sum over the read targets `Events`, `RecentRepositories`,
  `ChangedFiles`, `CommitMessages` and `Catalog`.

The query family stays nested rather than flattened into the request enum so
that the component's three operations keep the arity they have always had, and
so `RequestUnimplemented` can name the query kind that was not built.

## Reply surface

- `EventRecorded(EventRecorded)` — a receive or observe operation was accepted
  and assigned a ledger sequence.
- `QueryResult(QueryResult)` — a query returned one of the closed result
  payloads: `Events`, `RecentRepositories`, `ChangedFiles`, `Commits` or
  `Catalog`.
- `RequestUnimplemented(RequestUnimplemented)` — the daemon accepted the
  contract shape but the runtime lacks that operation or query path.

## Naming discipline

`QueryKind` is a tag enum whose heads — `Events`, `RecentRepositories`,
`ChangedFiles`, `CommitMessages`, `Catalog` — are wire-facing names. Ethos Zero
gives a bare variant the declared type of the same name as its payload, so **no
declared type in this contract may spell one of those five heads**. The request
payloads are therefore named `EventsQuery`, `RecentRepositoriesQuery`,
`ChangedFilesQuery` and `CommitMessagesQuery`, and the listings
`EventListing`, `RecentRepositoriesListing`, `ChangedFileListing`,
`CommitListing` and `CatalogListing`. The same rule holds for `OperationKind`
(`Receive`, `Observe`, `Query`), `Class` and `UnimplementedReason`.
`tests/generated_contract.rs` carries the behavioural witness that these stay
bare on the wire.

`LedgerQuery::Catalog` carries nothing. The hand-written contract it replaces
needed an empty `Catalog` struct only because its channel macro required every
operation to name a type; ethos says "carries nothing" with a bare head.

## Owns

- `ReceiveHookNotification`, matching the Gitolite `post-receive` hook fallback
  spool record.
- `PushObservation`, the direct hook-to-ledger submission record: the push
  notification plus per-commit observations and per-file changes.
- Repository and ref identity aliases.
- `DaemonConfiguration`, the typed startup record for the daemon's ordinary
  socket, meta socket, store and spool directory.
- The closed `LedgerQuery` and `QueryResult` payloads, and the read payloads
  for agent-facing discovery.

## Does Not Own

- The daemon, socket listeners, actors, or sema-engine storage.
- Meta-signal policy and configuration mutation.
- Gitolite server installation, or GitHub mirroring.

## Constraints

- Public operation roots are contract-local verbs. They do not expose
  `Assert`, `Mutate`, `Retract`, `Match`, `Subscribe` or `Validate`.
- Query replies are grouped under `QueryResult`; individual `*Listing` records
  are payload records, not public reply siblings.
- Typed component commands and any storage projection belong to the daemon,
  not to this contract.
- Contract records stay domain-specific and runtime-free.
- Daemon configuration is a typed contract record, not CLI flags or
  environment variables on the production launch path.
