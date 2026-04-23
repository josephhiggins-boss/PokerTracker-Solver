# AGPL FAQ

This FAQ is a plain-language summary of the most common questions
about how AGPL-3.0 applies to this project. It is not legal advice.
For anything binding, consult a lawyer.

The authoritative document is [LICENSE](../LICENSE). If anything here
conflicts with the actual license text, the license text wins.

## Can I use this commercially?

Yes. AGPL-3.0 permits commercial use. You can run this service for
paying customers, charge for hosting, offer it as part of a paid
product, or build a business around it.

The obligation AGPL imposes is not "you can't make money from it" -
it is "you must make the source available to the users you are
serving over the network, including any modifications you have made."

## Do I have to open-source MY app that calls this API?

No, not on account of this project.

The PokerTracker mobile app (and any other client you build) is a
separate work that communicates with this server over a network
protocol. AGPL-3.0 section 13 governs the server - people interacting
with the *server* over the network are entitled to the *server's*
source. It does not reach out across the wire and infect clients.

You do need to open-source the server itself - including your
modifications to it - if you run it for network users. You do not
need to open-source the client application that calls it.

If your "client" is actually linking against this code or
`postflop-solver` directly (in-process, as a library), that is a
different situation and the copyleft would apply. HTTP clients are
fine.

## What if I modify the server?

Two obligations kick in.

1. **Keep the same license.** Your modified version is still AGPL-3.0.
   You cannot strip the license off your fork or relicense it.
2. **Offer source to network users.** Anyone who interacts with your
   running modified version over a network must be able to obtain
   the complete corresponding source code of the exact version you
   are running - including your modifications.

The typical way to satisfy the second point is to publish your fork
publicly (GitHub, GitLab, a source tarball on your own server) and
expose a link to it from the running service - for example, at a
`/source` endpoint or in a footer.

You should also record your modifications somewhere visible.
`CHANGELOG.md` is a reasonable place.

## Can I relicense this?

No, not unilaterally.

Relicensing an AGPL project requires permission from every copyright
holder. That includes:

- All contributors to this project.
- The upstream authors of any AGPL components this project links
  against - notably `postflop-solver`, by b-inary.

Without their consent, this project cannot be re-released under a
permissive license, a proprietary license, or any license that is
not AGPL-3.0 (or later, at the project's option).

If you are building something that needs a permissive license for
this functionality, your options are:

- Negotiate a separate commercial license with every relevant
  copyright holder.
- Replace the AGPL components with code you own or with
  permissively-licensed equivalents.

## Does running this on my laptop for personal use trigger anything?

No. AGPL-3.0 only triggers source-availability obligations when you
are distributing the software or running it as a network service
*for other users*. Using the software yourself - even a modified
version - does not require you to publish anything.

## Where can I ask more questions?

Open an issue using the license-question template in
`.github/ISSUE_TEMPLATE/license-question.md`. Describe your situation
specifically; vague questions get vague answers.
