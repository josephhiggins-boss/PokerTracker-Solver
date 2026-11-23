# PokerTracker Solver API

## What this is

An HTTP API that wraps [postflop-solver](https://github.com/b-inary/postflop-solver)
to provide poker hand solutions for the PokerTracker mobile app's
hand-review feature. The server accepts a hand description, runs the
solver, and returns the solution over HTTP.

## License

This project is licensed under the **GNU Affero General Public License,
version 3 (AGPL-3.0)**. The full text is in [LICENSE](LICENSE). Third-party
attributions are in [NOTICE](NOTICE).

## AGPL implications for operators

The AGPL is a copyleft license with a network-use clause (section 13).
If you run this server and let users interact with it over a network,
you are required to make the complete corresponding source code,
including any modifications you have made, available to those users.

In practical terms, if you deploy this service:

- Publish your source (including patches) somewhere publicly reachable.
- Tell your users where to get it. A link in the API response or in
  user-facing documentation is the standard approach.
- If you modify the server, your modified version must carry the same
  AGPL-3.0 license and offer source to its users.

See [COMPLIANCE.md](COMPLIANCE.md) for an operator checklist and
[docs/agpl-faq.md](docs/agpl-faq.md) for common questions.

## Source availability

The canonical source for this project is:

> https://github.com/josephhiggins-boss/PokerTracker-Solver

**Status: this repository is not yet public.** `git remote -v` already
points here, but until this repo is pushed and set to public visibility
on GitHub, the URL above 404s and the deployment at `solver.jo3vo.io`
is in breach of AGPL-3.0 section 13 (see [SOURCE-OFFER.md](SOURCE-OFFER.md)
for exactly what that means and the steps to fix it). This is not a
"nice to have" — it blocks legal operation of the deployed service.

The running service also exposes `GET /source`, which returns this URL
(or `SOLVER_SOURCE_URL` if set) as a machine-readable source offer, per
AGPL-3.0 section 13's "network server" method of providing access to
Corresponding Source.

If you are running a modified build, you must publish your sources at
a location of your own, point `SOLVER_SOURCE_URL` at it, and make that
location discoverable to your users.

## Build

Requires a recent stable Rust toolchain.

```
cd solver-api
cargo build --release
```

## Run

```
./solver-api/target/release/solver-api
```

Configuration, ports, and endpoint documentation live alongside the
service in `solver-api/`.

## Relationship to the PokerTracker mobile app

The PokerTracker mobile app is a separate work that communicates with
this service over HTTP. The mobile app does not link against
`postflop-solver` or this server's code; it only speaks the network
protocol. Under AGPL-3.0 section 13, communicating with an AGPL
service over a network protocol does not make the client a derivative
work of the service. The mobile app is therefore licensed separately
by its author and is not bound by the AGPL.

The AGPL obligations in this repository apply to operators of this
server, not to consumers of its HTTP API.

## Contributing

Contributions are welcome. By submitting a contribution you agree to
license your contribution under AGPL-3.0 on the same terms as the rest
of this project. No separate contributor license agreement is
required. If you need relicensing (for example, to a dual license),
that requires explicit agreement from all contributors, and is not
something we can do unilaterally.
