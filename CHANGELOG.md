# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial AGPL compliance scaffolding: LICENSE (AGPL-3.0), NOTICE,
  README with operator obligations, COMPLIANCE.md checklist, AGPL
  FAQ, and an issue template for license questions.
- `GET /source` endpoint (`solver-api/src/handlers.rs`): returns the
  AGPL-3.0 source-offer as JSON (`license`, `source_url`, `notice`).
  `source_url` reads `SOLVER_SOURCE_URL` from the environment, falling
  back to the project's GitHub URL.
- `SOURCE-OFFER.md`: precise statement of what AGPL-3.0 section 13
  requires for this deployment, whether the current setup is in
  breach (yes, until the repo is public), and the exact steps to fix
  it.

### Known gap (blocking legal compliance)

- The repository is not yet pushed to GitHub / made public. Until it
  is, the source-offer URL in README, NOTICE, and `/source` all point
  to a URL that 404s, and the deployment at `solver.jo3vo.io` is in
  breach of AGPL-3.0 section 13. See SOURCE-OFFER.md.
