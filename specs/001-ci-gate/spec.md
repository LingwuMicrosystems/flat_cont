# Feature Specification: The gates run on every push to main and every pull request

**Feature Branch**: `001-ci-gate`

**Created**: 2026-11-13

**Status**: ready-for-agent

**Input**: User description: 给 flat_cont 项目加上 ci，先形成 spec

## Problem Statement

`flat_cont` defines the flat-continuation IR and nothing else: `FlatContGraph`, `FlatContinuation`, `ValueRef`, `EffectPath`, plus the type vocabulary in `common`. It ships three input-model tests and no runner, so nothing executes them on push. A change reaches `main` with failing or uncompiled tests unless the maintainer remembers to run them.

This repository has already paid for that once. ADR 0002 records that the rewrite in `e9661ba` dropped the types the source-IR converter still referenced, and that the crate root commented the pass out so the library would still build. The two commits that bound that interval are dated 2026-07-13 and 2026-11-13: the dead layer sat in the tree for four months, and a gate would have gone red on the first day of it.

The asymmetry that makes this urgent rather than tidy: this crate is consumed as a sibling path dependency. The `katabasis` gate checks out `LingwuMicrosystems/flat_cont` at whatever `main` points to when it runs, and records the SHA it received. A broken push here surfaces as a red run in a repository whose author did not cause it, and that run's log names a dependency instead of naming a defect.

The crate's portability promise is also unguarded. `#![no_std]` is why this crate exists instead of being a `std` data-structure crate, and nothing on the push path verifies that it still builds for a target with no operating system. This spec deliberately leaves that gate out and records the gap rather than papering over it.

## Solution

One GitHub Actions workflow runs on every push to `main`, on every pull request, and on manual dispatch. It has one job named `ci` on `ubuntu-latest` and runs three steps in order: the format check, clippy with warnings denied, and the test suite.

A committed toolchain file pins the compiler, formatter, and linter to one version, so the workflow never repeats a version number and a contributor's local run agrees with the runner's. The workflow holds read-only repository permissions, needs no secret, keeps no cache, and carries a fixed timeout.

The tree is already clean on all three gates — format and clippy pass today with zero findings — so the workflow's first run is green by construction. Unlike the two sibling repositories, no separate formatting commit is needed to earn that first green.

After the workflow lands, the `ci` check is made required on `main`. That is one repository setting applied by hand, recorded here as a dependency because no file can express it.

## User Stories

1. As a maintainer, I want every push to `main` and every pull request to run the test suite, so that a red change cannot land unnoticed.
2. As a maintainer, I want the format and lint gates to run alongside the tests, so that review attention is not spent on whitespace and default-group warnings.
3. As a maintainer, I want the first run of the gate to be green without a repair commit, so that enabling CI does not require editing sources in the same change.
4. As a contributor, I want the toolchain declared in the repository, so that my laptop and the runner use the same compiler without me reading the workflow.
5. As a contributor, I want to run locally exactly the commands CI runs, so that works-on-my-machine stays a narrow category.
6. As a contributor, I want those commands written in the repository's agent instructions, so that I do not have to reverse-engineer them from YAML.
7. As a reviewer, I want a failing check to name the command that failed, so that triage starts from the failure instead of from a re-run.
8. As a reviewer, I want a red check to block the merge, so that the gate is a mechanism rather than a habit.
9. As a reviewer, I want a rapid series of pushes to cancel its superseded runs, so that I never wait on a verdict for a commit nobody is looking at any more.
10. As a reviewer, I want every commit on `main` to keep its own completed verdict, so that the claim this commit was checked stays true in the history.
11. As a downstream implementer in `katabasis`, I want this repository's gate to go red before my own does, so that the failure is reported where its cause lives.
12. As a downstream implementer, I want the gate to run on the commit I just pushed, so that I can tell whether the head I build against is sound before opening my own repository.
13. As a future agent working here, I want the gate's exact commands and flags recorded in the spec, so that I can reproduce a red check without guessing at missing flags.
14. As a future agent, I want the spec to say which gates are deliberately absent, so that I do not mistake a decision for an oversight and quietly change scope.
15. As a security-conscious maintainer, I want the workflow to hold read-only repository permissions, so that a compromised action cannot write anywhere.
16. As an operator, I want a bounded job timeout, so that a hung cargo invocation fails the job instead of occupying a runner.
17. As an operator, I want the workflow to need no repository secret, so that there is nothing to expire, rotate, or leak.
18. As an operator, I want the job name frozen by the spec, so that a later rename cannot orphan the branch protection rule.
19. As a maintainer, I want adding CI to change no source file, no test, and no public item of the crate, so that the change is reviewable as configuration alone.
20. As a maintainer of the sibling repositories, I want the same action versions and the same toolchain pin as the other two, so that I keep one review habit rather than three.
21. As a reader of this repository, I want the spec to record why the cross-target build is absent and what it would cost to add, so that the gap reads as a decision rather than a blank.
22. As a maintainer, I want the no-cache decision backed by a measurement, so that a future complaint about slow CI can be answered with numbers instead of an opinion.

## Implementation Decisions

### One job, one frozen name

The workflow has a single job on `ubuntu-latest`, and that job is named `ci`. The name is frozen by this spec because a required status check is matched by name: renaming the job leaves the protection rule pointing at a check that never reports, which blocks every pull request indefinitely.

A single job rather than two is deliberate. Three steps in sequence are cheap here, and a single job means a single check name to configure and to read. The steps are ordered as format, lint, test, so a failure reports the cheapest gate first.

### The three gates

Each gate is one step with a name of its own, so a red run identifies the command rather than the job.

| Gate | Command |
| --- | --- |
| Format | `cargo fmt --all -- --check` |
| Lint | `cargo clippy --all-targets --locked -- -D warnings` |
| Test | `cargo test --locked` |

`--all` on the format gate is safe here because the crate is a single package with no path dependencies; the sibling that uses a package flag does so only to avoid reformatting other people's sources.

`--all-targets` on the lint gate is the difference between linting the library and linting everything a contributor can build, which includes the integration tests.

`-D warnings` is what makes the gate a gate. Today the tree produces no warnings, so raising this from advisory to fatal costs nothing now and costs a great deal later.

`--locked` on both the lint and test gates turns a dependency added without a committed lockfile into a red step with a one-line diagnosis, instead of a silent re-resolution to a version nobody tested.

No `rustfmt.toml` and no `clippy.toml` are added. Default rustfmt style and the default clippy lint groups are the policy; pedantic and nursery groups stay off.

### The toolchain declaration is the only place a version is written

A committed `rust-toolchain.toml` names channel `1.94.0` and the components `rustfmt` and `clippy`. The workflow does not repeat the version: a step runs `rustup show` and prints the compiler, formatter, and linter versions, so a missing component fails at a step named for the toolchain rather than halfway through the gates.

One complication is worth stating because it is the reason for the rule: rustfmt output changes between compiler releases. A workflow that pins its own version while the checkout pins none produces a gate that goes red for a change that is not wrong, and the usual repair is to stop trusting the gate.

The pinned version matches the two sibling repositories, so the three agree on what formatter output and what lint set is correct.

### What floats

Everything except the toolchain floats, and the choice is recorded rather than implied.

| Input | This version | Note |
| --- | --- | --- |
| Toolchain | Pinned to 1.94.0 in the repository | Changing it is a reviewed commit |
| Runner image | `ubuntu-latest` | Moves when GitHub moves it |
| Actions | Major version tags | Mutable by design; SHA pinning is out of scope |
| Cache | None | See the measurement in Further Notes |

Action versions are major tags, matching both sibling repositories. `actions/checkout` is the only third-party action the workflow uses, because it is the only one it needs.

### Triggers

The workflow triggers on a push to `main`, on any pull request, and on manual dispatch. A push to any other branch does not trigger it, because the pull-request trigger already covers those branches and an unrestricted push trigger would produce a duplicate pair of checks for one commit.

Manual dispatch earns its place twice: it is how the negative proofs below are produced against a scratch branch, and it is how a green run can be reproduced without manufacturing a commit.

### Cancellation

A superseded run is cancelled on pull requests only. The concurrency group is scoped per workflow and ref, with the run identity mixed in on non-pull-request events so that a queued run is never replaced by a newer one.

The asymmetry is the point. On a pull request, an obsolete verdict has no value, so cancelling saves minutes and noise. On `main`, a cancelled run leaves a commit with no completed verdict, and a required check that never completes is indistinguishable from one that failed. Every commit on `main` therefore keeps its own verdict.

### Timeout

The job carries a fixed twenty-minute timeout. It is not derived from a measurement — the workflow has to exist before it can be timed — and it is set far above the expected cost so that the timeout catches a hang rather than a slow machine. The cold measurement in Further Notes is the reason it can stay this generous.

### Permissions and credentials

The workflow declares `permissions: contents: read` and every checkout sets `persist-credentials: false`. There is no repository secret, and the workflow does not reference one: the crate has no private dependency to fetch, so a credential would be pure attack surface.

The job sets `CARGO_INCREMENTAL: 0`. Incremental artifacts have no value on a runner that is destroyed after the job, and disabling them keeps the build directory small enough that the no-cache decision is defensible.

### The baseline is already clean

Measured on 2026-11-13 at commit `618d979`: the format check reports no diffs, clippy reports zero warnings across all targets, and the suite reports three passing tests with zero unit tests and zero doctests.

No repair commit is needed, and none is included. This is the one structural difference from both sibling repositories, each of which had to land a formatting-only commit before its workflow could start green.

### Enforcement is a repository setting, not a file

Making `ci` a required check is applied by hand in the repository settings after the workflow first runs on `main`. It cannot be expressed in the tree, so it is recorded under Further Notes as a dependency rather than claimed as part of the change.

The consequence of a required check is that a cancelled run counts as not passed. Since cancellation here is scoped to pull requests, and a pull request that is superseded immediately gets a newer run, the only path to a stuck merge would be a permanently failing or never-starting job.

### Local parity

The three gate commands are added to the build-and-test section of the repository's agent instructions, next to the existing `cargo build` and `cargo test` lines, with a note that CI runs exactly these.

No Makefile and no pre-commit hook are added. The agent instructions are already the entry point that both humans and agents read, and a hook that only runs for contributors who install it produces a false sense of enforcement.

### What the change adds and does not touch

Besides this specification, it adds two files: the workflow and the toolchain declaration. It edits one document: the agent instructions. It does not touch a Rust source file, a test, the public surface of the crate, the dependency list, the lockfile, or the version number.

It also does not touch the sibling repositories. How they consume this crate is their own gate's business, and the `katabasis` spec already owns that side of the boundary.

### Decision record

| Point | Taken | Alternative |
| --- | --- | --- |
| Spec location | `specs/001-ci-gate/`, committed | Scratch directory, untracked |
| Gate set | Format, lint, test | Adding a doc build or a cross-target build |
| Runner | One `ubuntu-latest` job | macOS or Windows matrix |
| Triggers | Push to `main`, pull request, dispatch | Push on every branch, which duplicates on pull requests |
| Cancellation | Superseded runs cancelled on pull requests only | Cancelled on every ref |
| Toolchain | Pinned to 1.94.0 in the repository | Floating stable, or pinned in the workflow |
| Cache | None | Registry only, or a full build cache |
| Action versions | Major tags | Commit SHAs plus a dependency update bot |
| Enforcement | Required check from day one | Advisory check, required later |
| Job name | `ci` | A descriptive name |
| Local mirror | Three commands in the agent instructions | A Makefile target, or a git hook |

## Testing Decisions

### What makes a good gate test

A gate is tested the way any behaviour is tested: it must fail on a real defect, pass on a clean tree, and report which command failed. A gate that merely starts a build and exits green has tested nothing.

The gate's external behaviour is exactly three command boundaries plus the toolchain resolution step. Everything below those boundaries is the existing suite; this change adds no assertion to the crate.

### The seams

There is one seam, and it is the whole workflow: the three named run steps in order, entered from the runner. The toolchain step is a second, narrower seam — it is the only place where a wrong version becomes observable before the gates run.

No Rust seam is introduced or moved. The existing seam, the integration test binary, is already the crate's only public-surface test and this change does not touch it. The number of seams across the codebase stays at what it is today.

### The negative proofs

Two scratch branches prove the gate bites, each dispatched by hand and deleted afterwards. The first carries one deliberately mis-formatted line and must go red at the format step. The second carries a failing assertion and must go red at the test step. Neither should be able to reach the other's step.

A third check is a green run on the landing commit itself. If that run is red, the gate is measuring something other than the change.

### Prior art

The commands are already the ones a contributor runs by hand; this change only stops them from being optional. The closest existing test is the input-model integration test, which asserts observable round-trip behaviour rather than internals — the same standard applied to the gate's three command boundaries.

Both sibling repositories validated their own gates by breaking them on purpose with a scratch branch and a manual dispatch. That protocol is reused rather than invented, so the negative proof leaves the same kind of record: two red runs in the dispatch history, and no artifact.

### What is not tested

The crate has no behaviour-level tests, and this change does not add any. ADR 0002 records that gap; a green gate does not close it, and this spec does not claim it does. The gate raises the floor for the tests that exist; it does not raise the ceiling of what is covered.

## Out of Scope

- **The cross-target build gate.** `cargo build --locked --target thumbv7em-none-eabihf` is not part of this workflow, by explicit decision. The argument against adding it is that `#![no_std]` already turns any crate-local use of `std` into a host compile error, so the host build subsumes the check for this crate's own code. The argument for adding it is that the check covers more than crate-local code: a dependency that re-enables `std` through feature unification still compiles for the host and still fails for a bare-metal target, and this crate's two dependencies are both configurable that way. The omission is recorded, not resolved. Closing it costs one step and can be done at any time.
- A `cargo doc` warning gate. The crate has no doctests and no cross-crate documentation surface to protect yet.
- An MSRV declaration and an MSRV test job. Nothing declares a minimum compiler version beyond edition 2024's requirement.
- Coverage measurement, `cargo-deny` auditing, benchmarks, and release or publish automation.
- A multi-OS or multi-target matrix.
- A dependency cache, and any pinning of the runner image or the action revisions by SHA.
- A pre-commit or pre-push hook mirroring the gates locally.
- Configured branch protection as code. The required check is a repository setting applied by hand.
- Automatic changes to `katabasis`, `entel`, or `ich`, or a scheduled run that tests this crate against their heads.
- `plan.md`, `tasks.md`, and checklists. This feature is delivered as the spec alone; implementation planning follows approval.

## Further Notes

**Measured baseline, 2026-11-13, commit `618d979`.** Format: clean. Clippy across all targets with denied warnings: zero findings. Tests: three passing, all in the input-model integration test, plus zero unit tests and zero doctests. Docs with denied warnings: clean.

**Cold build cost.** The full dependency graph — `serde_core`, `serde`, `quote`, `syn`, `serde_derive`, `smallvec`, `flat_cont` — compiles from scratch in 2.85 seconds on the development machine for a bare-metal target, which is a stricter build than the host one the gate will run. That measurement is the entire justification for shipping no cache: the cache action would cost more to upload and restore than the rebuild it saves.

**Why no repair commit.** Both sibling repositories had to land a formatting-only commit first, because their trees were not format-clean when the gate was proposed. This tree is, which makes the first run green by construction and keeps the change reviewable as configuration alone.

**Failure ownership.** A red gate belongs to the author of the change. A red run on a commit that was green before, in a downstream repository, is a dependency break originating here — the downstream log will name a dependency rather than a command, which is why this repository's own gate has to exist.

**Operational dependencies.** GitHub Actions must be enabled for the organization, and the maintainer must have repository administration rights to set the required check. Neither is code; if the check never reports, these are the first two things to verify.

**The repository is public.** Its Actions minutes are not a billing concern, which is why the runner choice is settled by consistency with the sibling repositories rather than by cost.

**A cancelled run is not a green run.** When the check is required, a cancelled run counts as not passed. Cancellation is scoped to pull requests precisely because a pull request always gets a newer run on the next push, while a commit on `main` does not.

**Known gap, explicitly not closed here.** This crate has no behaviour-level tests for the IR itself. A green gate means the three input-model tests still pass, the tree formats, the tree lints, and the crate compiles — it does not mean the IR is correct. ADR 0002 records this gap and this spec inherits it.
