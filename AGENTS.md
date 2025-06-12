# Guidelines for Codex agents

This repository holds a small Rust crate. Make sure the code remains
formatted and all tests pass before sending a pull request.

This crate is intended to evolve into a full HTTP framework. The initial focus
is on HTTP abstractions, but upcoming milestones will introduce controllers,
security mechanisms, database connections with an ORM, and a template system.
The aim is to provide a Rust alternative to typical PHP web stacks.

* Always run `cargo fmt` and `cargo test` before committing changes.
* Document new modules or functions with Rust doc comments.
* Prefer clear commit messages summarising the intent of the change.
* Strive for clean code and keep the repository architecture organised.
* Follow Clean Architecture principles and practise TDD whenever possible.
