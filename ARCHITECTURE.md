# signal-repository-ledger Architecture

`signal-repository-ledger` is the ordinary Signal contract for repository event
submissions and read queries.

It is called by trusted local peers that can submit repository-receive
notifications or ask for repository ledger state. Privileged repository
registration, hook policy, and mirror configuration live in
`owner-signal-repository-ledger`.

## Owns

- `RepositoryReceiveHookNotification`, matching the current Gitolite
  `post-receive` hook fallback spool record.
- `RepositoryPushObservation`, the direct hook-to-ledger submission record. It
  wraps the push notification plus per-commit observations and per-file changes.
- Repository/ref identity newtypes.
- `RepositoryLedgerDaemonConfiguration`, the typed startup record for the
  daemon's ordinary socket, owner socket, store, and spool directory.
- Read query payloads for recent repository events and registered repositories.
- Read query payloads for agent-facing discovery:
  `RepositoryRecentRepositoriesQuery`, `RepositoryChangedFileQuery`, and
  `RepositoryCommitMessageQuery`.
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
(RepositoryPushObservation
  (RepositoryReceiveHookNotification
    "repository-ledger"
    "gitolite-admin"
    "20260519T140736Z"
    true
    [(RefUpdate "old-commit" "new-commit" "refs/heads/main")])
  [(RepositoryCommitObservation
      "new-commit"
      "refs/heads/main"
      "2026-05-19T14:07:36+00:00"
      "add repository query surface\n\nLonger commit message body."
      [(RepositoryFileChange "M" "src/lib.rs" None)
       (RepositoryFileChange "R100" "src/new.rs" (Some "src/old.rs"))])])
```

Agent discovery queries:

```nota
(RepositoryRecentRepositoriesQuery (Some "20260519T000000Z") 20)

(RepositoryChangedFileQuery
  (Some "repository-ledger")
  (Some "20260519T000000Z")
  (Some "20260519T235959Z")
  (Some "src")
  50)

(RepositoryCommitMessageQuery
  None
  None
  None
  (Some "query surface")
  50)
```

These are the canonical record shapes. The current generated channel request
CLI surface still accepts present optional fields as bare values; that is a
`signal_channel!` syntax gap, not the contract's desired record grammar.
