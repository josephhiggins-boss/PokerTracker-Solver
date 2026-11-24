# AGPL Compliance Checklist

This checklist is for operators running this service. AGPL-3.0 section 13 requires
that network users of a running copy be offered its corresponding source code.
The items below are the minimum an operator should confirm before exposing the
service to other people.

## Operator checklist

- [x] **Source code publicly available** at
  `https://github.com/josephhiggins-boss/PokerTracker-Solver` (public since 2026-11-24).

  If you run a fork, publish your own repository and point `SOLVER_SOURCE_URL` at it.

- [x] **`/source` HTTP endpoint returns a link to the source.**
  `GET /source` returns `{license, source_url, notice}`. `source_url` reads
  `SOLVER_SOURCE_URL` and falls back to the repository above. Set it to the tag of
  the release you actually run so the offer names the exact version.

- [ ] **The offer reaches the people using your service.** Add a visible line and
  link to your app, website or terms page (operators of the hosted PokerTracker
  app do this on its legal page). This item is per deployment: tick it when it is
  true for yours.

- [x] **Modifications are documented in [CHANGELOG.md](CHANGELOG.md).** AGPL-3.0
  requires modified files to carry prominent notices that they were changed and when.
  If you fork, keep your own entries there.

- [ ] **Dependency license audit for the release you ship.** The known AGPL-3.0
  dependency is `postflop-solver` (https://github.com/b-inary/postflop-solver),
  pinned in `solver-api/Cargo.toml`. All other crates are listed in
  `solver-api/Cargo.lock`; re-check their licenses when you update it. Tick this per
  release.

## When to re-run this checklist

- Before every tagged release.
- After adding or upgrading any Rust crate.
- After deploying a fork or modified build.
- When moving the service to a new host or domain (the source link must stay reachable).

## Out of scope

- Licensing of clients. A client that only talks to this service over HTTP is a
  separate work; see the README and the AGPL FAQ.
- Licensing of proprietary features layered on top of a fork. AGPL-3.0 applies to
  the server as a whole, including your additions.
