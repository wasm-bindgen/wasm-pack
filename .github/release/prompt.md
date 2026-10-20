You are preparing the wasm-pack {{version}} release, the release after
{{prev}}. Your working directory is the repository at `master` with the
versions already bumped. You can read any file, but you can only edit
`CHANGELOG.md`. Maintainers review everything you change in the release PR.

The `## 🤍 Unreleased` section of `CHANGELOG.md` collects the entries since
{{prev}}. Turn it into the {{version}} section:

- Insert `## <emoji> {{version}}` so that the current entries move under it,
  leaving `## 🤍 Unreleased` empty above it, as in previous releases. Choose
  an emoji that continues the progression of the previous releases' emoji;
  patch releases can reuse or play on the previous one.
- Group entries under `- ### ✨ Features`, `- ### 🤕 Fixes`,
  `- ### 🛠️ Maintenance` and `- ### 📖 Documentation`, in that order, dropping
  empty groups.
- Fix clear mistakes only: typos, broken links, duplicates, missing link
  definitions, and entries under the wrong group. Don't otherwise reword
  entries.
- Compare the entries against the merged PRs below. Add an entry for each
  user-facing PR that has none, in the style of the surrounding ones: a bold
  title with the author and PR, a short description when the PR's purpose
  isn't clear from its title, and the link definitions. Leave out dependency
  bumps, CI changes and other internal changes unless they matter to users,
  and list any PR you were unsure about in `notes`.

In `notes`, write short Markdown for the maintainers reviewing the release
PR: PRs you left out or were unsure about, and anything else that needs a
human decision. Leave out what you changed, since the diff shows that.
