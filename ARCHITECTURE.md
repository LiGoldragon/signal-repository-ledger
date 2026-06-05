# signal-repository-ledger Architecture

`signal-repository-ledger` is the ordinary Signal contract for repository event
submissions and read queries.

It is called by trusted local peers that can submit repository-receive
notifications, submit direct push observations, or ask for repository
ledger state. Privileged repository registration, hook policy, and
mirror configuration live in `meta-signal-repository-ledger`.

## Migration history — three-layer model (2026-05-19/2026-05-20)

This crate is the pilot contract for the migration from public
`signal-core` Sema verbs to the three-layer model affirmed
2026-05-20 (per
`primary/reports/designer/246-v4-bundled-fix-deep-design-with-examples.md`
and `primary/reports/designer/248-three-layer-changes-for-operators.md`).

The public request surface (Layer 1) is now:

- `Receive(ReceiveHookNotification)` — a Gitolite hook fallback spool
  notification reached the ordinary daemon.
- `Observe(PushObservation)` — the caller observed a repository push
  and submits the typed commits and file changes.
- `Query(Query)` — the caller asks for ledger state. The payload is a
  closed `Query` sum with read targets:
  `Events`, `RecentRepositories`, `ChangedFiles`, `CommitMessages`,
  and `Catalog`.

There is no public `Assert` / `Match` tag in this contract. The daemon
owns its typed Component Commands (Layer 2 — e.g.
`LedgerCommand::RecordEvent`, `LedgerCommand::ReadRecentRepositories`)
and projects them to payloadless Sema class labels (Layer 3) for
cross-component observation via `ToSemaOperation`. See
`~/primary/skills/component-triad.md` §"Verbs come in three layers".

Repository-ledger is **not** a persona component; the mandatory
`Tap`/`Untap` observable block does not apply here. (Persona
components reserve the standardized observability surface.) If a
domain-specific observation channel lands later, it uses domain-named
Subscribe/Retract pairs.

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
  daemon's ordinary socket, meta socket, store, and spool directory.
- Closed `Query` payload for repository event and repository catalog
  reads.
- Closed `QueryResult` payload for repository event and repository
  catalog replies.
- Read payloads for agent-facing discovery:
  `RecentRepositories`, `ChangedFiles`, and `CommitMessages`.
- Ordinary operation/reply variants declared with `signal_channel!`.

## Does Not Own

- The daemon, socket listeners, actors, or sema-engine storage.
- Meta-signal policy/configuration mutation.
- Gitolite server installation.
- GitHub mirroring.

## Constraints

- Public operation roots are contract-local verbs. They do not expose
  `Assert`, `Mutate`, `Retract`, `Match`, `Subscribe`, or `Validate`.
- `Receive` introduces a hook-notification fact when accepted by the
  daemon. (Daemon-side projection: Sema `Assert`.)
- `Observe` introduces one push event plus zero or more commit/file
  observations when accepted by the daemon. (Daemon-side projection:
  Sema `Assert`.)
- `Query` reads ledger state; the daemon's `CommandExecutor` knows
  the read plan. (Daemon-side projection: Sema `Match`.)
- Query replies are grouped under `QueryResult`; individual `*Listing`
  records are payload records, not public reply siblings.
- Typed Component Commands and Sema-class projection belong to the
  daemon, not this contract.
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
    [repository-ledger]
    [gitolite-admin]
    [20260519T140736Z]
    true
    [(RefUpdate [old-commit] [new-commit] [refs/heads/main])])
  [(CommitObservation
      [new-commit]
      [refs/heads/main]
      [2026-05-19T14:07:36+00:00]
      [add repository query surface\n\nLonger commit message body.]
      [(FileChange [M] [src/lib.rs] None)
       (FileChange [R100] [src/new.rs] [src/old.rs])])]))
```

Agent discovery queries:

```nota
(Query (RecentRepositories (Some [20260519T000000Z]) 20))

(Query (ChangedFiles
  (Some [repository-ledger])
  (Some [20260519T000000Z])
  (Some [20260519T235959Z])
  (Some [src])
  50))

(Query (CommitMessages
  None
  None
  None
  (Some [query surface])
  50))
```

Query result reply:

```nota
(QueryResult (RecentRepositories
  [(RecentRepository [repository-ledger] [20260519T140736Z] 42 8)]))
```

## Pending schema-engine upgrade

**Status:** scheduled for migration to schema-language-based contract per `reports/designer/326-v13-spirit-complete-schema-vision.md` + `reports/designer/324-migration-mvp-spirit-handover-re-specification.md`.

**Target:** this contract's hand-written `signal_channel!` invocation converts to a single `repository-ledger/repository-ledger.schema` file (shared with the repository-ledger daemon's repository). The brilliant macro library (`primary-ezqx.1`) reads the schema + emits this crate's wire types + ShortHeader projection + dispatcher binding + VersionProjection impls.

**Sequence:** Spirit is the MVP pilot landing first via `primary-ezqx.1`; this contract's schema cutover follows after pilot succeeds. Repository-ledger has a paired `meta-signal-repository-ledger`; both legs of the policy-vs-working split appear in the shared `repository-ledger.schema` file per the schema-language's separation discipline.

**Per-component concerns:** Query-heavy contract (the canonical examples above show `(Query …)` requests + `(QueryResult …)` replies); the schema-language must encode optional positional fields (`(Some …)` / `None`) per the canonical examples without losing the typed-variant discipline. No special blockers — straightforward cutover after pilot.

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` — uniform header form + schema-language design
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` — migration MVP + handover state
- `reports/designer/322-spirit-mvp-positional-schema-worked-example.md` — Spirit MVP worked example
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` — header/body/feature separation + lowering rules
