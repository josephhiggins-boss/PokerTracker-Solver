# Source Offer (AGPL-3.0 Section 13)

This file is the formal offer of source for running copies of this service.

## The offer

The complete corresponding source code of this service is published at no charge at:

> https://github.com/josephhiggins-boss/PokerTracker-Solver

The repository has been public since 2026-11-24. Release tags in that repository
identify exact versions. A running instance also advertises the location through
`GET /source`, which returns the URL in the `SOLVER_SOURCE_URL` environment variable
(defaulting to the repository above).

## What section 13 requires (plain language)

> If you run a modified version of this Program on a server and let other
> users communicate with it there, your server must also allow them to
> download the source code corresponding to the modified version.

- It applies to running the software as a network service, not only to
  distributing binaries.
- It applies to the people who interact with the service remotely, which for a
  service behind an app means the app's users, not only developers calling the API.
- The source offered must match the version that is actually running. If you
  run a modified build, offer your modified source.
- The offer must be free of charge and use a standard means of copying software,
  such as a public git host.

## If you operate your own copy

1. Run an unmodified release: the default `source_url` is enough, ideally pointed at
   the matching release tag.
2. Run a modified build: publish your sources (including patches) in a public
   repository, keep the AGPL-3.0 license, and set `SOLVER_SOURCE_URL` to that
   repository or to a tag of it.
3. Tell your users where the source is. A link in your app, website or legal page
   is the usual way. A machine-readable endpoint alone does not reach the people the
   offer is for.

See [COMPLIANCE.md](COMPLIANCE.md) for a checklist and [docs/agpl-faq.md](docs/agpl-faq.md)
for common questions.

This is not legal advice. For anything binding, consult a lawyer.
