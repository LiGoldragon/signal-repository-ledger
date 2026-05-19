# signal-repository-ledger Architecture

`signal-repository-ledger` is the ordinary Signal contract for repository event
submissions and read queries.

It is called by trusted local peers that can submit repository-receive
notifications or ask for repository ledger state. Privileged repository
registration, hook policy, and mirror configuration live in
`owner-signal-repository-ledger`.

## Owns

- `RepositoryReceiveHookNotification`, matching the current Gitolite
  `post-receive` hook spool record.
- Repository/ref identity newtypes.
- `RepositoryLedgerDaemonConfiguration`, the typed startup record for the
  daemon's ordinary socket, owner socket, store, and spool directory.
- Read query payloads for recent repository events and registered repositories.
- Ordinary request/reply variants declared with `signal_channel!`.

## Does Not Own

- The daemon, socket listeners, actors, or sema-engine storage.
- Owner-only policy/configuration mutation.
- Gitolite server installation.
- GitHub mirroring.

## Constraints

- Every request variant declares one of the six `signal-core` verbs.
- Hook notifications are `Assert`: they introduce a new event fact.
- Read queries are `Match`.
- Contract records stay domain-specific and runtime-free.
- Daemon configuration is a typed contract record, not CLI flags or environment
  variables on the production launch path.
