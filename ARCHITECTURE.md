# signal-repository-ledger Architecture

`signal-repository-ledger` is the ordinary Signal contract for repository event
submissions and read queries.

It is called by trusted local peers that can submit repository-receive
notifications or ask for repository ledger state. Privileged repository
registration, hook policy, and mirror configuration live in
`owner-signal-repository-ledger`.

## MUST IMPLEMENT — signal architecture migration

This contract is migrating to contract-local verbs per
`primary/reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`
and `primary/reports/designer/239-signal-architecture-migration-plan.md`.

This crate is the named pilot for the migration. Drop the five
`Match *Query` variants (`EventQuery`, `RecentRepositoriesQuery`,
`ChangedFileQuery`, `CommitMessageQuery`, `CatalogQuery`); lift `Query`
to a contract-local operation root whose payload is a closed `Query`
enum naming the read targets (`Events`, `RecentRepositories`,
`ChangedFiles`, `CommitMessages`, `Catalog`). Replace the two
`Assert` variants (`ReceiveHookNotification`, `PushObservation`) with
contract-local verbs on the assertion side — `Receive` for the
hook-spool notification and `Observe` (or another verb-form word
chosen during implementation) for the direct push observation.
Replace the `Match`/`Assert` SignalVerb declarations in
`signal_channel!` with `operation <Verb>(<Payload>)` shapes; move the
daemon's verb-to-Sema lowering (`Query` → `Match` over indexes,
`Receive`/`Observe` → `Assert` event rows) into the runtime executor.

References: `primary/reports/designer/238-signal-architecture-redirection-contract-local-verbs.md`,
`primary/reports/designer/239-signal-architecture-migration-plan.md`.

**Note to remover:** when the refactor lands, remove this section and
add a `## Migration history — contract-local verbs (2026-05-XX)`
paragraph noting the shape change.

## Owns

- `ReceiveHookNotification`, matching the current Gitolite
  `post-receive` hook fallback spool record.
- `PushObservation`, the direct hook-to-ledger submission record. It
  wraps the push notification plus per-commit observations and per-file changes.
- Repository/ref identity newtypes.
- `DaemonConfiguration`, the typed startup record for the
  daemon's ordinary socket, owner socket, store, and spool directory.
- Read query payloads for recent repository events and registered repositories.
- Read query payloads for agent-facing discovery:
  `RecentRepositoriesQuery`, `ChangedFileQuery`, and
  `CommitMessageQuery`.
- Ordinary request/reply variants declared with `signal_channel!`.

## Does Not Own

- The daemon, socket listeners, actors, or sema-engine storage.
- Owner-only policy/configuration mutation.
- Gitolite server installation.
- GitHub mirroring.

## Constraints

- Every request variant declares one of the six `signal-core` verbs.
- Hook notifications are `Assert`: they introduce a new event fact.
- Push observations are `Assert`: they introduce one push event plus zero or
  more commit/file observations.
- Read queries are `Match`.
- Query text matching is ordinary substring matching over typed fields. The
  first implementation is case-insensitive so agents can search commit messages
  without knowing exact capitalization.
- Contract records stay domain-specific and runtime-free.
- Daemon configuration is a typed contract record, not CLI flags or environment
  variables on the production launch path.

## Pseudo-NOTA Schema

Direct hook submission:

```nota
(PushObservation
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
       (FileChange "R100" "src/new.rs" (Some "src/old.rs"))])])
```

Agent discovery queries:

```nota
(RecentRepositoriesQuery (Some "20260519T000000Z") 20)

(ChangedFileQuery
  (Some "repository-ledger")
  (Some "20260519T000000Z")
  (Some "20260519T235959Z")
  (Some "src")
  50)

(CommitMessageQuery
  None
  None
  None
  (Some "query surface")
  50)
```

These are the canonical record shapes. The current generated channel request
CLI surface still accepts present optional fields as bare values; that is a
`signal_channel!` syntax gap, not the contract's desired record grammar.
