## Documentation and Tests
- Use `///` and `//!` for documentation.
- Do not add comments explaining internal implementation details.

### Scope & Targets
- Inspect new and edited files (via git) inside `src/` and `tests/`.

### Documentation Requirements
- Document every new, modified, or undocumented public item thoroughly across the codebase:
  - **Modules & Crate Root:** Responsibility, crate purpose, and first useful call.
  - **Functions:** Purpose, parameters, return values, error/panic conditions, and executable doc-tests (`cargo test --doc`).
  - **Structs & Enums:** Roles, field usage, variants, construction, and invariants.
  - **Traits & Macros:** Guarantees, relying behaviors, syntax, expansion, and examples.
- Skip private items.Inspect new and edited files (via git) inside `src/` and `tests/`.
- Retain accurate existing documentation; update only incorrect text without overwriting unaffected paragraphs.

### Testing Requirements
- Provide comprehensive test coverage for all public API items and changes.
- Add **unit tests** for component behaviors.
- Add **integration tests** under `tests/` to verify public API calls from outside the library.
- **Do not alter existing assertions** to match changed code. If a change causes an existing test to fail, leave the assertion intact and note the failure in the summary.
- Do not add benchmarks, Miri harnesses, or mutation-testing configurations.
