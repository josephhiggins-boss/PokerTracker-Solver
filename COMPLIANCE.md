# AGPL Compliance Checklist

This checklist is for operators running this service in production.
AGPL-3.0 section 13 requires that network users of a modified version
be offered the corresponding source code. The items below are the
minimum an operator should confirm before exposing this service to
end users.

## Operator checklist

- [ ] **Source code publicly available at:** `___________________`

  Why: AGPL-3.0 section 13 requires operators to offer the complete
  corresponding source of the version running on the server to any
  user who interacts with it over a network.

- [ ] **`/source` HTTP endpoint (or equivalent) returns a link to the
      source archive.** (TODO for the solver-api team.)

  Why: A discoverable, machine-readable way for clients to locate the
  source is the cleanest way to satisfy the "offer source" obligation.
  Putting it in the API surface makes it verifiable and hard to
  overlook.

- [ ] **NOTICE contents surfaced in API responses (e.g. in `/health`
      or the root endpoint).** (TODO for the solver-api team.)

  Why: The NOTICE file names the upstream AGPL component
  (`postflop-solver`) and explains the network-use clause. Surfacing
  it alongside the API keeps attribution visible to anyone auditing
  the deployment.

- [ ] **Modifications are documented in [CHANGELOG.md](CHANGELOG.md).**

  Why: AGPL-3.0 requires that modified files carry prominent notices
  stating that they were changed and when. A maintained changelog is
  the simplest way to satisfy this at the project level.

- [ ] **Third-party dependency audit run for the current release.**

  Known AGPL-3.0 dependency: `postflop-solver`
  (https://github.com/b-inary/postflop-solver). All other transitive
  dependencies are enumerated in `solver-api/Cargo.lock`; re-check
  their licenses before each release in case of upstream changes.

  Why: A new transitive dependency with an incompatible license can
  silently break compliance. Cargo.lock is the source of truth; treat
  it as such.

## When to re-run this checklist

- Before every tagged release.
- After adding or upgrading any Rust crate dependency.
- After deploying a fork or modified build.
- When moving the service to a new host or domain (the source link
  needs to remain reachable to users of the new deployment).

## Out of scope here

- Licensing of the PokerTracker mobile client. The client speaks HTTP
  to this service and is not a derivative work of it; see the README
  and the AGPL FAQ for details.
- Licensing of any proprietary features layered on top of a fork.
  AGPL-3.0 applies to the server as a whole, including your
  additions.
