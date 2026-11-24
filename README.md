# PokerTracker Solver

A small HTTP service that wraps [postflop-solver](https://github.com/b-inary/postflop-solver)
and exposes heads-up postflop solving as a JSON API. It was written for the
hand-review feature of the PokerTracker app, but it is a plain HTTP service and
can be used by anything that can send a POST request.

Given a board, two ranges, a pot and an effective stack, it runs a CFR solve and
returns each player's equity plus the first player's strategy at the root of the
tree: how often each action is taken and what it is worth.

## Responsible use

**This is a post-session study tool.** Running a solver, or any other form of
real-time assistance, while you play online poker breaks the terms of every
major poker site and can get an account closed. Never run this software while a
poker client is open on the same machine.

The PokerTracker backend that launches this service enforces that rule: it starts
the solver only when a solve is requested, stops it when idle, refuses to start it
while a known poker client process is running, and kills it immediately if one
starts. If you run the service yourself, the same responsibility is yours. This
is not legal advice; read your poker site's terms.

## License

Licensed under the **GNU Affero General Public License, version 3** (AGPL-3.0).
The full text is in [LICENSE](LICENSE); third-party attributions are in
[NOTICE](NOTICE).

This service builds on **[postflop-solver](https://github.com/b-inary/postflop-solver)
by b-inary**, also AGPL-3.0. All the solving is done by that crate; this repository
adds only the HTTP layer, presets and result summarising. Upstream development of
postflop-solver is suspended (it is no longer actively maintained), so this
project pins a specific upstream commit (see `solver-api/Cargo.toml`) and does not
expect upstream fixes.

Because the AGPL has a network clause (section 13), anyone who uses a running copy
of this service over a network is entitled to its complete source, including your
changes if you modified it. See [SOURCE-OFFER.md](SOURCE-OFFER.md),
[COMPLIANCE.md](COMPLIANCE.md) and [docs/agpl-faq.md](docs/agpl-faq.md).

Canonical source: <https://github.com/josephhiggins-boss/PokerTracker-Solver>

## Build and run

Requires a recent stable Rust toolchain. On Windows with the GNU toolchain you
also need a full MinGW (for example `scoop install mingw`) so that linking works.
The `postflop-solver` dependency is fetched from GitHub on first build.

```
cd solver-api
cargo build --release
cargo test            # includes one end-to-end solve; takes a few minutes
./target/release/solver-api
```

The service listens on `127.0.0.1:7878` and logs to stdout.

## Configuration

| Variable | Default | Meaning |
|---|---|---|
| `SOLVER_BIND` | `127.0.0.1:7878` | Address and port to listen on. |
| `SOLVER_SOURCE_URL` | the GitHub URL above | Returned by `GET /source`. Set it to your own repository if you run a modified build. |
| `RUST_LOG` | `info,solver_api=debug` | Log filter (`tracing` env-filter syntax). |

There is **no authentication and no rate limiting**. Keep it bound to
`127.0.0.1` and put your own authenticating proxy in front of it if other
machines need access.

## HTTP API

All bodies are JSON.

### `GET /health`

```
$ curl http://127.0.0.1:7878/health
{"status":"ok"}
```

### `GET /source`

The AGPL section 13 source offer.

```
$ curl http://127.0.0.1:7878/source
{"license":"AGPL-3.0-or-later","source_url":"https://github.com/josephhiggins-boss/PokerTracker-Solver","notice":"This service wraps postflop-solver ..."}
```

### `POST /solve`

Starts a solve and returns immediately with `202 Accepted` and a job id.

```
$ curl -X POST http://127.0.0.1:7878/solve -H 'content-type: application/json' -d '{
    "board": "Ah7d2c",
    "oop_range": "TT+,AKs",
    "ip_range": "QQ+,AKs",
    "starting_pot": 100,
    "effective_stack": 100,
    "quality": "fast"
  }'
{"job_id":"8676da98-37f9-4728-ba5a-d84ea50d67ff"}
```

| Field | Type | Notes |
|---|---|---|
| `board` | string | 6, 8 or 10 characters: flop (`AsTd9c`), flop+turn, or flop+turn+river. Whitespace is ignored. |
| `oop_range`, `ip_range` | string | postflop-solver range notation, e.g. `22+,A2s+,KTo+,QJs`. OOP acts first. |
| `starting_pot` | integer | Chips in the pot at the start of the street being solved. |
| `effective_stack` | integer | Smaller remaining stack, same units as the pot. |
| `quality` | `"fast"` \| `"balanced"` \| `"deep"` | Optional. Overrides iteration cap, exploitability target and bet sizes (table below). |
| `max_iterations` | integer | Default 1000. Ignored when `quality` is set. |
| `target_exploitability_pct` | number | Default 0.5 (percent of the pot). Ignored when `quality` is set. |
| `bet_sizes` | `{flop,turn,river}` | Optional pot fractions per street, e.g. `{"flop":[0.33,0.75]}`. Ignored when `quality` is set. |
| `raise_sizes` | string array | Default `["2.5x"]`. |
| `include_allin_bet` | boolean | Default true: add all-in as a bet size. |
| `add_allin_threshold`, `force_allin_threshold` | number | Tree-pruning thresholds (defaults 1.5 and 0.15). |

Preset details:

| `quality` | Iteration cap | Stop at exploitability | Bet sizes (pot fractions) |
|---|---|---|---|
| `fast` | 200 | 2% of pot | flop 0.5, turn 0.75, river 1.0 |
| `balanced` | 500 | 1% | flop 0.33/0.75, turn 0.75, river 0.75/1.5 |
| `deep` | 1500 | 0.4% | flop 0.33/0.75/1.5, turn 0.5/1.0, river 0.5/1.0/1.5 |

Rake is not modelled. An unparseable body is rejected by the framework with a
plain 4xx response; invalid ranges or boards are reported later as a job error.

### `GET /solve/{job_id}`

Poll until `status` is `done` or `error`. Unknown ids return `404`
`{"error":"job not found"}`.

While running (real output from the binary):

```
{"status":"running","progress":0.24,"iterations":120,"exploitability":1.9465061,"result":null,"error":null}
```

When done (real output for the request above, formatted for reading):

```
{
  "status": "done",
  "progress": 1.0,
  "iterations": 30,
  "exploitability": 1.7222595,
  "error": null,
  "result": {
    "board": "Ah7d2c",
    "starting_pot": 100,
    "effective_stack": 100,
    "oop_equity": 0.2880725,
    "ip_equity": 0.7119275,
    "actions": [
      {"action": "check", "amount": null, "frequency": 0.9438222, "ev": 29.110287},
      {"action": "bet", "amount": 50, "frequency": 0.055699255, "ev": 22.675789},
      {"action": "allin", "amount": 100, "frequency": 0.0004785237, "ev": 9.226245}
    ],
    "total_iterations": 30,
    "final_exploitability": 1.7222595,
    "solve_ms": 321
  }
}
```

`actions` are the options of the player who acts first at the root, with the
frequency averaged over that player's whole range and the average EV in chips.
Action names are lower case: `check`, `bet`, `raise`, `call`, `fold`, `allin`.
`ev` can be `null` if the engine returned no per-action value.

Errors from the engine come back as a job with `"status":"error"` and a message in `error` (for example a range that does not parse). A request missing a required field is rejected at submission with HTTP 422, e.g. `Failed to deserialize the JSON body into the target type: missing field \`oop_range\``.

## Performance and memory

Solve time depends on the board, the ranges, the stack depth and the CPU. As
an **observed** data point, a `fast` solve took roughly **60 to 150 seconds** on a
6-core laptop (Core i7-8750H, 2018) depending on other load. A later run
on the same laptop, otherwise idle, took about 27 seconds for a `fast` solve of a
tight-range spot (`TT+,AQs+,AKo` against `22+,A2s+,K9s+,QTs+,JTs,ATo+`, flop
`Ks7d2c`, 100 pot, 900 stack). `balanced` and `deep` are progressively larger
trees and take longer; with two wide 20-30% ranges, `balanced` was still running
after 4.5 minutes and the process held about 5.4 GB. Those are single
observations, not benchmarks or promises.

The solver allocates uncompressed 32-bit strategy tables, so memory grows quickly
with range width, stack depth and the number of bet sizes. Wide ranges with the
`deep` preset can use many gigabytes. There is no queue or concurrency limit:
several simultaneous solves share the CPU and multiply memory use, so submit one
at a time.

Finished jobs are kept in memory until the process exits (there is no eviction).
That is fine for a service that is started on demand and stopped when idle, as
the PokerTracker backend does; if you run it for long periods, restart it
periodically.

## Contributing

Contributions are welcome. By submitting a contribution you agree to license it
under AGPL-3.0 on the same terms as the rest of the project. No separate
contributor license agreement is required.
