# Source Offer (AGPL-3.0 Section 13)

This file is the formal "offer to provide source" for this deployment.
It exists because AGPL-3.0 section 13 requires more than owning a license
file — it requires that anyone interacting with a *running, network-facing*
copy of this software actually be able to get the source of the exact
version they are talking to.

## What section 13 actually says (plain language)

> If you run a modified version of this Program on a server and let other
> users communicate with it there, your server must also allow them to
> download the source code corresponding to the modified version.

Key details, precisely:

- It applies to **running the software as a network service**, not just
  distributing binaries. Merely having a public LICENSE file is not
  sufficient by itself — the *offer* has to be made to the people using
  the service.
- It applies whether or not you modified the code. Even an unmodified
  deployment must make the corresponding source available to network
  users (in practice this is trivially satisfied by linking to the
  unmodified upstream repo, but the link must actually exist and work).
- If you *did* modify it, the source you offer must be **the modified
  source**, not the original upstream project.
- "Users interacting with it remotely through a computer network" means
  the humans on the other end of the deployment — for
  `solver.jo3vo.io`, that is PokerTracker's paying subscribers, whose
  app makes solver requests on their behalf. It does not mean only
  developers who read raw HTTP responses.
- The offer must be at "no charge," through "some standard or customary
  means of facilitating copying of software" — a public git host
  (GitHub, GitLab, etc.) satisfies this; a private repo, a repo that
  returns 404, or a promise with no working link does not.

## Is the current setup in breach?

**Yes, right now.** Specifically:

1. `solver.jo3vo.io` is a live, network-facing deployment of
   AGPL-3.0-licensed software (this project wraps `postflop-solver`,
   itself AGPL-3.0).
2. Until this repository is pushed to GitHub and made public, the
   README's source link (`https://github.com/josephhiggins-boss/PokerTracker-Solver`)
   returns 404. An offer that points to a dead link is not an offer.
3. No part of the running service previously surfaced a source link at
   all — `GET /health` returned only `{"status":"ok"}`. This session
   added `GET /source` (see `solver-api/src/handlers.rs`) so the
   service itself now advertises where to get its source, but that
   endpoint is only true once step 1 is done.
4. Nothing in the PokerTracker mobile app or its legal page currently
   points end users at the solver's source. Section 13 obligates you to
   make source available to those users, not just to make it
   theoretically fetchable by someone who already knows to call
   `/source`.

Net: the *code and docs* are now in place to comply once published, but
**compliance is not yet true** — the actual source is not publicly
reachable. This is a live legal exposure for as long as
`solver.jo3vo.io` keeps running unmodified.

## Exact steps to fix it (in order)

1. **Verify the pushed source matches the deployed binary exactly.**
   Before pushing, confirm `solver-api/` on disk (including this
   session's `/source` endpoint addition) is what's actually compiled
   and running at `solver.jo3vo.io`. If the running binary is older,
   either redeploy first or note the discrepancy — don't publish source
   that doesn't match what's live, and don't claim compliance until they
   match.
2. **Push this repository to GitHub and set it to Public.**
   ```
   cd C:\Projects\PokerTracker-Solver
   git push -u origin main
   ```
   Then in GitHub repo settings: Danger Zone → Change visibility → Public.
   (Note: `solver-api/` currently shows as a nested git repo with
   uncommitted changes — `git status` shows "modified content" for it.
   Resolve that — either commit solver-api's own history and add it as a
   proper submodule with a `.gitmodules` entry, or fold its tracked files
   into the parent repo directly — before pushing, so what's public
   actually includes the source tree, not a dangling gitlink.)
3. **Tag the release that matches the live deployment**, e.g. `git tag
   v0.1.0-solver.jo3vo.io && git push --tags`, so the offer points at a
   fixed, verifiable commit rather than a moving `main` branch that may
   drift from what's actually running.
4. **Set `SOLVER_SOURCE_URL`** in the solver-api's environment on the
   host running `solver.jo3vo.io` to the tagged URL, e.g.:
   ```
   SOLVER_SOURCE_URL=https://github.com/josephhiggins-boss/PokerTracker-Solver/tree/v0.1.0-solver.jo3vo.io
   ```
   (If unset, `/source` falls back to the `main` branch URL, which is
   weaker evidence of "this exact version" once the code moves on.)
5. **Rebuild and redeploy** solver-api so `GET /source` is live at
   `solver.jo3vo.io/source` and returns the tagged URL.
6. **Surface the offer to actual end users**, not just API responders.
   The people entitled to the source are PokerTracker's subscribers, who
   never call `/source` directly. Add one line + link to
   `PokerTracker/landing/legal.html` (Privacy/Terms page, already
   linked from the app) naming the solver component and linking to the
   published repo. A short addition, e.g. under "Sharing" or a new
   "Open-source components" note, is enough — it does not need its own
   page.
7. **Re-run `COMPLIANCE.md`'s checklist** and check off each item once
   true, not once "written."

Steps 2–6 require Joseph's GitHub account and access to the deployment
host — they cannot be completed from this session. Everything up to
that point (endpoint code, docs, this notice) is done.

This is not legal advice. For anything binding, consult a lawyer.
