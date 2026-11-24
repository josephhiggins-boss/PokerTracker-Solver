# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-24

First public release. The repository is public at
https://github.com/josephhiggins-boss/PokerTracker-Solver as of this date.

### Added

- HTTP service wrapping postflop-solver: `POST /solve`, `GET /solve/{id}`,
  `GET /health` and `GET /source`, with quality presets `fast`, `balanced` and `deep`.
- `GET /source`: the AGPL-3.0 section 13 source offer as JSON (`license`,
  `source_url`, `notice`). `source_url` reads `SOLVER_SOURCE_URL` and falls back
  to the project's GitHub URL.
- AGPL compliance documents: LICENSE (AGPL-3.0), NOTICE, SOURCE-OFFER.md,
  COMPLIANCE.md, docs/agpl-faq.md and a license-question issue template.
- A public README covering build, configuration, the full HTTP API with examples,
  observed performance, memory notes and responsible-use guidance.

### Changed

- Documentation rewritten for a public repository; earlier drafts described the
  repository as not yet public.
- The runtime log `solver.out` is no longer tracked and is ignored.
