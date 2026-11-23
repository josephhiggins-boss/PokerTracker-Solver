# AGPL Compliance Checklist

This checklist is for operators running this service in production.
AGPL-3.0 section 13 requires that network users of a modified version
be offered the corresponding source code. The items below are the
minimum an operator should confirm before exposing this service to
end users.

## Operator checklist

- [ ] **Source code publicly available at:**
  `https://github.com/josephhiggins-boss/PokerTracker-Solver`

  **Not yet true as of this writing — the URL 404s.** `git remote -v`
  already points here; the repo has not been pushed / made public. See
  [SOURCE-OFFER.md](SOURCE-OFFER.md) for the exact steps. This is the
  one blocking item — everything else in this checklist is scaffolding
  around it.

  Why: AGPL-3.0 section 13 requires operators to offer the complete
  corresponding source of the version running on the server to any
  user who interacts with it over a network.

- [x] **`/source` HTTP endpoint returns a link to the source archive.**
  Added `GET /source` in `solver-api/src/handlers.rs` (routed in
  `lib.rs`). Returns `{license, source_url, notice}`, where
  `source_url` reads `SOLVER_SOURCE_URL` from the environment and
  falls back to the GitHub URL above. Set `SOLVER_SOURCE_URL` on the
  deployment host once a tagged release is public (see
  SOURCE-OFFER.md step 4) so the endpoint points at the exact running
  version rather than a moving branch.

  Why: A discoverable, machine-readable way for clients to locate the
  source is the cleanest way to satisfy the "offer source" obligation.
  Putting it in the API surface makes it verifiable and hard to
  overlook.

- [ ] **The offer reaches actual end users, not just API callers.**
  PokerTracker subscribers never call `/source` directly — their app
  does, invisibly. Add a line + link to `PokerTracker/landing/legal.html`
  naming the solver component and its source location. (TODO — see
  SOURCE-OFFER.md step 6.)

  Why: Section 13's "users" are the humans interacting with the service
  remotely, not just the HTTP client. A machine-readable endpoint alone
  doesn't put the offer in front of the people it's for.

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
