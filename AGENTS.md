# Guidelines for Codex agents

This repository holds a small Rust crate. Make sure the code remains
formatted, and all tests pass before sending a pull request.

This crate is intended to evolve into a full HTTP framework. 
Upcoming milestones will security mechanisms, database connections with an ORM, and a template system.
The aim is to provide a Rust alternative to typical PHP web stacks.

* Always run `cargo fmt` and `cargo test` before committing changes.
* Document new modules or functions with Rust doc comments. Add examples where
  appropriate.
* Prefer clear commit messages summarizing the intent of the change. Write lists of
  changes in the commit body.
* Strive for clean code and keep the repository architecture organized.
* Follow Clean Architecture principles and practise TDD whenever possible.
* All identifiers, comments, documentation, branch names, commit messages, and
  file names **must be written in English**.
* Whenever the project evolves, update both `ROADMAP.md`, `CHANGELOG.md` and `README.md` to
  reflect the current state and features.
