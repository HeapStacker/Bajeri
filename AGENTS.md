# Agent rules

1. REUSE WHAT ALREADY EXISTS — Search the project first. Reuse existing things, don't reinvent the wheel.
2. ONLY REFACTOR WHEN I ASK FOR IT — KEEP REFACTORING SMALL AND CLEAN. Don't increase code, abstractions, or complexity without a concrete reason.
3. DO ONLY WHAT WAS REQUESTED — Make the smallest change that solves the task. No unrelated changes.
4. NO COMMENTS — Don't add comments unless requested.
5. PLAN LARGE CHANGES — Give a short plan before large refactors or multi-file changes.
6. ASK WHEN UNCLEAR — Ask before making assumptions about ambiguous requests.
7. IGNORE HIDDEN, BUILD AND CONFIGURATION FOLDERS — Never search, read, or edit files in `.vscode/`, `.cache/`, `.cmake/`, `build/`, `info/`, `.zed/`, `.cursor/`... unless it's specified.
8. TESTING EXCEPTION — You may access `build/` to run executables or check logs ONLY when explicitly asked to test or debug.
9. USE CONSISTENT CODE STYLE EVERYWHERE - If there is a simpler, more concise way to write something do it that way but remember to primarily write simple memorable code if you can

# AI Agent Instructions

## Custom Commands

Execution Rule:

- Execute custom commands ONLY when the user's prompt consists solely of the command itself (e.g. `do commit`, `do push`, or `do publish`).
- Do NOT trigger these flows if the command is merely referenced within a longer request or discussion.



When I type `do commit`, execute in terminal sequentially:

1. `cargo fmt`
2. `git add .`
3. `git commit -m "<appropriate short message>"`



When I type`do push`, generate tests and documentation by following the instructions in the file `info/generate_tests_docs.md`, and then execute the following commands:

1. `cargo fmt`
2. `just check` (must pass successfully)
3. `git add .`
4. `git commit -m "<appropriate short message>"`
5. `git push`



When I type `do publish`, generate tests and documentation by following the instructions in the file `info/generate_tests_docs.md`, and then execute the following commands:

1. Read the remote repository URL from `.git/config` and update the `repository` field under `[package]` in `Cargo.toml` with it.
2. Replace any `[[...]]` placeholders in `README.md` (including the brackets) with actual project details. If none are found, leave the file unchanged.
3. `cargo fmt`
4. `just check` (must pass successfully)
5. `git add .`
6. `git commit -m "<appropriate short message>"`
7. Read `version` from `Cargo.toml`.
8. `git push`
9. `git tag v<version>`
10. `git push origin v<version>`
