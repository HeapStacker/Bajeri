# kComprehensive Technical Comparison of Licenses

| **License** | scope | copyleft | include | mention changes | patent grant | if you sue | don't mention for marketing |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **Apache-2.0**                     | Project | -       | +                      | +                      | + | lose patent grant, keep copyright          | + |
| **Apache-2.0 WITH LLVM-exception** | Project | -       | not in embedded binary | not in embedded binary | + | lose patent grant, keep copyright          | + |
| **BSD-2-Clause**                   | Project | -       | +                      | -                      | - | keep copyright                             | - |
| **BSD-3-Clause**                   | Project | -       | +                      | -                      | - | keep copyright                             | + |
| **0BSD**                           | Project | -       | -                      | -                      | - | keep copyright                             | - |
| **CC0-1.0**                        | Project | -       | -                      | -                      | - | copyright give-up remains                  | - |
| **Unlicense**                      | Project | -       | -                      | -                      | - | copyright give-up remains                  | - |
| **MIT**                            | Project | -       | +                      | -                      | - | keep copyright                             | - |
| **Zlib**                           | Project | -       | source only            | +                      | - | keep copyright                             | - |
| **BSL-1.0**                        | Project | -       | source only            | -                      | - | keep copyright                             | - |
| **ISC**                            | Project | -       | +                      | -                      | - | keep copyright                             | - |
| **MPL-2.0**                        | File    | file    | +                      | -                      | + | lose copyright and patent if you sue first | - |
| **LGPL-2.1-only**                  | Library | library | +                      | +                      | - | keep copyright                             | - |
| **LGPL-3.0-only**                  | Library | library | +                      | +                      | + | lose patent grant, keep copyright          | - |
| **GPL-2.0-only**                   | Work    | work    | +                      | +                      | - | keep copyright                             | - |
| **GPL-3.0-only**                   | Work    | work    | +                      | +                      | + | lose patent grant, keep copyright          | - |
| **AGPL-3.0-only**                  | Work    | network | +                      | +                      | + | lose patent grant, keep copyright          | - |
| **Unicode-3.0**                    | Data    | -       | copy or docs           | -                      | - | keep copyright                             | + |
| **Unicode-DFS-2016**               | Data    | -       | copy or docs           | -                      | - | keep copyright                             | + |

- **scope** — what the license pulls in when you share the code
  - `Project` — stays on that code. Other files may use another license.
  - `File` — a changed file stays under it. A neighboring file does not have to.
  - `Library` — stays on the library. A program that only links it does not take it on.
  - `Work` — pulls in the whole combined program you share. A separate program beside it does not.
  - `Data` — Unicode data and the software with it, not the whole project.
- **copyleft** — whether the recipient must get the source under the same license. There is no `+`: the word says how far the duty reaches.
  - `-` — no. You may keep your changes closed.
  - `file` — only files that already carry this license.
  - `library` — the library stays, and the recipient must be able to replace it. Linking alone does not pull in the whole program. Copied code does.
  - `work` — the whole combination stays under the license, and the recipient gets the source.
  - `network` — the same as `work`, also for users of your modified version over a network.
- **include** — whether you must keep the copyright notice and the license text
  - `+` — you must.
  - `-` — you do not have to.
  - `source only` — required in a source copy, not in a binary alone.
  - `not in embedded binary` — not for the part compiled into object code. Still required for source.
  - `copy or docs` — in the copy, or in the docs that come with it.
- **mention changes** — whether you must mark on the file that you changed it. This is not a duty to publish the changes.
  - `+` — you must.
  - `-` — you do not have to.
  - `not in embedded binary` — not on the part compiled into object code. Still required on source.
- **patent grant** — whether a contributor explicitly gives the patents on their contribution. It protects you from that contributor, not from an outside company.
  - `+` — yes.
  - `-` — no.
- **if you sue** — what happens to your rights if you claim this code infringes your patent
  - `keep copyright` — your permission to use the code stays.
  - `lose patent grant, keep copyright` — you lose the patent permission. Permission to use the code stays. This also applies when you raise the patent as a reply in someone else's lawsuit.
  - `lose copyright and patent if you sue first` — you lose both only if you started the lawsuit. A reply to someone else's lawsuit does not trigger it, nor does asking a court to say you do not infringe.
  - `copyright give-up remains` — the author gave up the copyright. A lawsuit does not bring it back. There is no patent permission.
- **don't mention for marketing** — whether the license forbids using the author's name in promotion
  - `+` — it forbids that.
  - `-` — it says nothing about that.
