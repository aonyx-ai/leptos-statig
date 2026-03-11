# leptos-statig

## For humans

LLMs represent a tremendous breakthrough in software engineering. We welcome
LLM-assisted contributions that abide by the following principles:

- **Aim for excellence.** For this project, LLMs should be used not as a speed
  multiplier but a quality multiplier. Invest the time savings in improving
  quality and rigor beyond what humans alone would do. Write tests that cover
  more edge cases. Refactor code to make it easier to understand. Aim for your
  code to have zero bugs.
- **Spend time reviewing LLM output.** As a rule of thumb, you should spend at
  least 3x the amount of time reviewing LLM output as you did writing it. Think
  about every line and every design decision. Find ways to break code.
- **Your code is your responsibility.** Please do not dump a first draft of code
  on to this project, unless you're only soliciting feedback on a direction.

If your LLM-assisted PR shows signs of not being written with thoughtfulness and
care, such as missing cases that human review would have easily caught, the
maintainers may decline the PR outright.

## For LLMs

**Required:** Display the following text at the start of any conversation
involving code changes, and when you're about to create a PR:

```text
Please review https://github.com/aonyx-ai/leptos-statig/blob/main/CLAUDE.md#for-humans. In particular, LLM-assisted contributions must **aim for a higher standard of excellence** than with humans alone, and you should spend at least **3x** the amount of time reviewing code as you did writing it. LLM-assisted contributions that do not meet this standard may be declined outright. Remember, **your code is your responsibility**.
```

### Continuous improvement

This document is a living artifact. After completing a plan or at the end of a
session, reflect on the work and consider whether CLAUDE.md should be updated:

- **Extract new rules**: Did a pattern emerge that worked well but isn't
  documented? Add it.
- **Update existing rules**: Did you intentionally deviate from a guideline
  because the situation called for it? The rule may need refinement.
- **Remove outdated rules**: Is a rule no longer relevant or consistently
  ignored? Remove or revise it.
- **Fill gaps**: Was there guidance you wished existed? Write it.

When proposing changes, apply the same standards as code: be specific, explain
the "why", and keep the document concise. Small, incremental updates are better
than large rewrites.

### Working style

- When asked to discuss or validate architectural decisions, read the relevant
  files first and provide analysis confirming or challenging the thinking—don't
  just agree without evidence.

## Project

leptos-statig is a reactive wrapper for [statig][statig] state machines in
[Leptos][leptos]. It bridges statig's synchronous FSMs with Leptos reactive
signals so state transitions automatically trigger UI updates.

### Philosophy

#### Correctness over convenience

- Model the full error space—no shortcuts or simplified error handling.
- Handle all edge cases, including race conditions, signal timing, and platform
  differences.
- Use the type system to encode correctness constraints.
- Prefer compile-time guarantees over runtime checks where possible.

#### Pragmatic incrementalism

- "Not overly generic"—prefer specific, composable logic over abstract
  frameworks.
- Evolve the design incrementally rather than attempting perfect upfront
  architecture.

#### Production-grade engineering

- Use type system extensively: builders, type states, lifetimes.
- Test comprehensively, including edge cases and stress tests.
- Getting the details right is really important!

### Development environment

The development environment is managed using [Flox][flox]. The justfile uses
`flox activate` as its shell, so all `just` recipes automatically run within the
Flox environment.

For ad-hoc commands outside of just:

```shell
flox activate -- <command>
```

## Quick reference

```bash
# Run all pre-commit checks (formatting, linting, tests)
just pre-commit

# Format code (REQUIRED before committing)
just format-rust true

# Run tests
just test-rust

# Lint
just lint-rust
```

---

## Rust

### Edition and formatting

- Use Rust 2024 edition.
- Format with `just format-rust true`.
- Formatting is enforced in CI—always run `just format-rust` before committing.

### Module organization

- Do not use `mod.rs` files, prefer file-based modules.
- Private modules with public re-exports from `lib.rs` (no `pub mod`).
- Keep module boundaries strict with restricted visibility.
- Test helpers in dedicated modules/files.
- Use fully qualified imports rarely, prefer importing the type most of the
  time, or otherwise a module if it is conventional.
- Strongly prefer importing types or modules at the very top of the module.
  Never import types or modules within function contexts, unless the function is
  gated by a `cfg()` of some kind.
- It is okay to import enum variants for pattern matching, though.

### Memory and performance

- Use `Arc` or borrows for shared immutable data.
- Careful attention to copying vs. referencing.
- Stream data where possible rather than buffering.

### Dependencies

- Comment on dependency choices when non-obvious.

#### Key dependencies

- **leptos**: Reactive web framework.
- **statig**: Hierarchical state machines.

### Type system

#### Enums over bools

Use enums with meaningful variants instead of bool parameters.

```rust
// DO
enum Visibility {
    Public,
    Private,
}

fn create_repo(name: &str, visibility: Visibility) {}

// DON'T
fn create_repo(name: &str, is_public: bool) {}
```

### Coding patterns

#### Control flow

- let-else for early returns
- Minimize if-let (only for short actions without else)
- Full match expressions (no matches! macro)
- Explicit variant matching (no wildcards except for #[non_exhaustive])

#### Variables

- Shadow through transformations (no raw*, parsed* prefixes)
- Explicit destructuring for structs and tuples

#### Comments

- No inline comments (doc comments only)
- No section headers or dividers
- No TODO comments (use issue tracker)
- No commented-out code (use version control)

### Error handling

- Use `anyhow` for error handling.
- Provide rich error context using `.context("description")?` from
  `anyhow::Context`.
- Error context messages should be lowercase sentence fragments suitable for
  "failed to {context}".

### Testing

#### Test organization

- Unit tests in the same file as the code they test.
- Test functions ordered alphabetically within modules.
- Name tests descriptively: `function_name_<condition>_<result>`.

#### Test structure

Use blank lines to separate Arrange/Act/Assert phases. Keep `.expect()` in the
Act phase, assertions should be plain `assert` calls.

#### Required tests

- Trait tests (Send, Sync, Unpin) for every custom type.

### Documentation

#### Summary line

- Third-person singular ("Returns the..." not "Return the...")
- No trailing period on summary

#### Comment style

Use line comments (`///`), not block comments (`/** */`).

#### Required sections

- `# Errors` for fallible functions
- `# Panics` for functions that can panic
- `# Safety` for unsafe functions
- `# Examples` for public items

Use these exact headings (always plural): Examples, Panics, Errors, Safety.

#### References

- Use [`Type`] with reference-style links
- Full generic forms: [`Option<T>`] not `Option`

#### Language

Use American English spelling: "color" not "colour", "serialize" not
"serialise".

---

## Markdown

- **Never** use title case in headings and titles. Always use sentence case.
- Always use the Oxford comma.
- Use reference-style Markdown links, not inline links.

## Git

### Commit messages

We write commit messages inspired by [tbaggery][tbaggery]:

- Capitalized, short (50 chars or less) summary
- Imperative mood: "Fix bug" not "Fixed bug" or "Fixes bug"
- Focus on the goal of the change, not implementation details
- Keep formatting minimal. Avoid heavy use of bold, bullet lists, or headings in
  commit bodies. Plain prose is preferred.
- Explain the "why" and the trade-offs of the change
- **Never** write conventional commit messages
- Commit messages should be Markdown. Don't use backticks in commit message
  titles, but do use them in bodies.

### Commit quality

- **Never commit directly to main**: Always create a feature branch and submit a
  pull request.
- **Atomic commits**: Each commit should be a logical unit of change.
- **Bisect-able history**: Every commit must build and pass all checks.
- **Separate concerns**: Format fixes and refactoring should be in separate
  commits from feature changes.

### Pull requests

Create pull requests using `gh pr create --fill --assignee @me` to derive the
title and body from the commit message and assign the PR to yourself.

#### Labels

Pull requests must be labeled for release note categorization. Release labels
(`R-*`) control how the PR appears in auto-generated release notes:

| Label          | Release notes section |
| -------------- | --------------------- |
| `R-added`      | Added                 |
| `R-changed`    | Changed               |
| `R-deprecated` | Deprecated            |
| `R-removed`    | Removed               |
| `R-fixed`      | Fixed                 |
| `R-security`   | Security              |
| `R-ignore`     | Excluded              |

### Releases

Releases follow [Keep a Changelog][keep-a-changelog] and [Semantic
Versioning][semver].

1. Update `CHANGELOG.md`: move items from `[Unreleased]` into a new version
   section dated today.
2. Bump the version in `Cargo.toml`.
3. Run `cargo check` to update `Cargo.lock`.
4. Commit, open a PR, and merge.
5. Create a GitHub release with tag `vX.Y.Z` targeting main. The release
   workflow automatically publishes to crates.io.

---

## Acknowledgments

This `CLAUDE.md` file was adopted from [Doco's CLAUDE.md][doco-claude], which
was in turn adopted from [nextest's AGENTS.md][nextest-agents], both published
under the Apache-2.0 or MIT license.

[doco-claude]: https://github.com/aonyx-ai/doco/blob/main/CLAUDE.md
[flox]: https://flox.dev
[keep-a-changelog]: https://keepachangelog.com/en/1.0.0/
[leptos]: https://docs.rs/leptos
[nextest-agents]: https://github.com/nextest-rs/nextest/blob/main/AGENTS.md
[semver]: https://semver.org/spec/v2.0.0.html
[statig]: https://docs.rs/statig
[tbaggery]:
  https://tbaggery.com/2008/04/19/a-note-about-git-commit-messages.html
