# Release Checklist

Releases are automated, with maintainers reviewing each step:

1. Run the [Release PR](https://github.com/wasm-bindgen/wasm-pack/actions/workflows/release-pr.yml)
  workflow on `master`, choosing a minor or patch bump. It bumps the version in
  `Cargo.toml`, `Cargo.lock`, `npm/package.json` and `docs/index.html`, copies
  `README.md` to `npm/README.md`, and updates the default chromedriver and
  geckodriver versions. Claude then finalizes the changelog, choosing the
  release's emoji and adding entries for merged PRs that lack them, and the
  workflow opens a `Release #.#.#` PR from `wasm-bindgen-bot` whose
  description is the release's changelog section. Claude's notes for
  reviewers are posted as a comment.
1. Review the PR, pushing fixes to it as needed. If more PRs land on `master`
  first, run the workflow again, which regenerates the branch from `master`
  and discards changes pushed to it.
1. Squash-merge the PR. Once the tests pass on the merge commit, the
  [Release tag](https://github.com/wasm-bindgen/wasm-pack/actions/workflows/release-tag.yml)
  workflow tags it `v#.#.#`. If they fail, re-running the failed jobs tags it
  once they pass.
1. The tag runs the Release workflow, which builds the binaries into a draft
  GitHub release with the changelog section as its notes, checks it has every
  asset, then waits for approval of the `publish` environment. The site's
  installer already points at the new version from the merge, so approve
  promptly.
1. Approval publishes to crates.io, then publishes the GitHub release and
  marks it as the latest, then publishes the npm package, which downloads its
  binary from the now public release. Releases are immutable once published.

To release by hand, make the same changes in a PR, merge it, and push the
`v#.#.#` tag once the tests pass, if the Release tag workflow didn't.
