#!/usr/bin/env bash
# Fails unless the staged changes against $1 are ones a release commit may
# make: the version bumps, the changelog, the docs page, the npm README copy
# and the default webdriver versions.
set -euo pipefail

base=$1
bad=0
fail() {
  echo "::error::$1"
  bad=1
}

# Fails if the file's diff changes any line besides those matching $2.
only_lines() {
  if git diff --cached -U0 "$base" -- "$1" | grep -E '^[-+]' | grep -vE '^(\+\+\+|---) ' | grep -vE "^[-+]$2\$"; then
    fail "$1 has changes besides $3"
  fi
}

while IFS= read -r path; do
  case "$path" in
    CHANGELOG.md | docs/index.html) ;;
    Cargo.toml | Cargo.lock)
      only_lines "$path" 'version = "[0-9.]+"' 'the version' ;;
    npm/package.json)
      only_lines "$path" '  "version": "[0-9.]+",' 'the version' ;;
    npm/README.md)
      if [ "$(git show :npm/README.md)" != "$(git show :README.md)" ]; then
        fail "npm/README.md differs from README.md"
      fi ;;
    src/test/webdriver/chromedriver.rs)
      only_lines "$path" 'const DEFAULT_CHROMEDRIVER_VERSION: &str = "[0-9.]+";' DEFAULT_CHROMEDRIVER_VERSION ;;
    src/test/webdriver/geckodriver.rs)
      only_lines "$path" 'const DEFAULT_GECKODRIVER_VERSION: &str = "v[0-9.]+";' DEFAULT_GECKODRIVER_VERSION ;;
    *) fail "Unexpected change to $path" ;;
  esac
done < <(git diff --cached --name-only --no-renames "$base")
exit $bad
