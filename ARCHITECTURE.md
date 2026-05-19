# signal-repository-ledger Architecture

`signal-repository-ledger` is the ordinary Signal contract for repository event
submissions and read queries.

It is called by trusted local peers that can submit repository-receive
notifications, submit direct push observations, or ask for repository
ledger state. Privileged repository registration, hook policy, and
mirror configuration live in `owner-signal-repository-ledger`.

## Migration history — contract-local operations (2026-05-19)

This crate is the pilot contract for the migration from public
`signal-core` Sema verbs to `signal-frame` contract-local operation
roots.

The public request surface is now:

- `Receive(ReceiveHookNotification)` — a Gitolite hook fallback spool
  notification reached the ordinary daemon.
- `Observe(PushObservation)` — the caller observed a repository push
  and submits the typed commits and file changes.
- `Query(Query)` — the caller asks for ledger state. The payload is a
  closed `Query` sum with read targets:
  `Events`, `RecentRepositories`, `ChangedFiles`, `CommitMessages`,
  and `Catalog`.

There is no public `Assert` / `Match` tag in this contract. The daemon
executor lowers these contract-local operations to Sema effects when
it mutates or reads its own durable tables.

The public reply surface mirrors that tree:

- `EventRecorded(EventRecorded)` — a receive/observe operation was
  accepted and assigned a ledger sequence.
- `QueryResult(QueryResult)` — a query returned one of the closed
  result payloads: `Events`, `RecentRepositories`, `ChangedFiles`,
  `Commits`, or `Catalog`.
- `RequestUnimplemented(RequestUnimplemented)` — the daemon accepted
  the contract shape but the runtime lacks that operation or query path.

## Owns

- `ReceiveHookNotification`, matching the current Gitolite
  `post-receive` hook fallback spool record.
- `PushObservation`, the direct hook-to-ledger submission record. It
  wraps the push notification plus per-commit observations and per-file changes.
- Repository/ref identity newtypes.
- `DaemonConfiguration`, the typed startup record for the
  daemon's ordinary socket, owner socket, store, and spool directory.
- Closed `Query` payload for repository event and repository catalog
  reads.
- Closed `QueryResult` payload for repository event and repository
  catalog replies.
- Read payloads for agent-facing discovery:
  `RecentRepositories`, `ChangedFiles`, and `CommitMessages`.
- Ordinary operation/reply variants declared with `signal_channel!`.

## Does Not Own

- The daemon, socket listeners, actors, or sema-engine storage.
- Owner-only policy/configuration mutation.
- Gitolite server installation.
- GitHub mirroring.

## Constraints

- Public operation roots are contract-local verbs. They do not expose
  `Assert`, `Mutate`, `Retract`, `Match`, `Subscribe`, or `Validate`.
- `Receive` introduces a hook-notification fact when accepted by the
  daemon.
- `Observe` introduces one push event plus zero or more commit/file
  observations when accepted by the daemon.
- `Query` reads ledger state; the daemon decides the Sema read plan.
- Query replies are grouped under `QueryResult`; individual `*Listing`
  records are payload records, not public reply siblings.
- Sema lowering belongs to the daemon executor, not this contract.
- Query text matching is ordinary substring matching over typed fields. The
  first implementation is case-insensitive so agents can search commit messages
  without knowing exact capitalization.
- Contract records stay domain-specific and runtime-free.
- Daemon configuration is a typed contract record, not CLI flags or environment
  variables on the production launch path.

## Pseudo-NOTA Schema

Direct push observation:

```nota
(Observe (PushObservation
  (ReceiveHookNotification
    "repository-ledger"
    "gitolite-admin"
    "20260519T140736Z"
    true
    [(RefUpdate "old-commit" "new-commit" "refs/heads/main")])
  [(CommitObservation
      "new-commit"
      "refs/heads/main"
      "2026-05-19T14:07:36+00:00"
      "add repository query surface\n\nLonger commit message body."
      [(FileChange "M" "src/lib.rs" None)
       (FileChange "R100" "src/new.rs" "src/old.rs")])]))
```

Agent discovery queries:

```nota
(Query (RecentRepositories (Some "20260519T000000Z") 20))

(Query (ChangedFiles
  (Some "repository-ledger")
  (Some "20260519T000000Z")
  (Some "20260519T235959Z")
  (Some "src")
  50))

(Query (CommitMessages
  None
  None
  None
  (Some "query surface")
  50))
```

Query result reply:

```nota
(QueryResult (RecentRepositories
  [(RecentRepository "repository-ledger" "20260519T140736Z" 42 8)]))
```
