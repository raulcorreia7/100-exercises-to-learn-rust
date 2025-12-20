#!/bin/sh
set -eu

print_help() {
  cat <<'EOF'
Usage: scripts/setup-git-identity.sh [options]

Sets repo-local git identity (user.name/user.email).

Options:
  -n NAME   Set user.name (or use GIT_USER_NAME)
  -e EMAIL  Set user.email (or use GIT_USER_EMAIL)
  -g        Populate name/email from gh CLI (falls back to noreply email)
  -s        Enable commit.gpgsign
  -h        Show this help
  --name NAME
  --email EMAIL
  --gh
  --sign
  --help

Env vars:
  GIT_USER_NAME, GIT_AUTHOR_NAME
  GIT_USER_EMAIL, GIT_AUTHOR_EMAIL
  GIT_COMMIT_SIGN=true
EOF
}

name="${GIT_USER_NAME:-${GIT_AUTHOR_NAME:-}}"
email="${GIT_USER_EMAIL:-${GIT_AUTHOR_EMAIL:-}}"
sign="${GIT_COMMIT_SIGN:-}"
use_gh=""
had_args=""

while [ "$#" -gt 0 ]; do
  case "$1" in
    -n) name="$2"; had_args="true"; shift 2 ;;
    -e) email="$2"; had_args="true"; shift 2 ;;
    -g) use_gh="true"; had_args="true"; shift ;;
    -s) sign="true"; had_args="true"; shift ;;
    -h) print_help; exit 0 ;;
    --name) name="$2"; had_args="true"; shift 2 ;;
    --email) email="$2"; had_args="true"; shift 2 ;;
    --gh) use_gh="true"; had_args="true"; shift ;;
    --sign) sign="true"; had_args="true"; shift ;;
    --help) print_help; exit 0 ;;
    *) print_help >&2; exit 1 ;;
  esac
done

if [ -z "$had_args" ] && [ -z "$name" ] && [ -z "$email" ] && [ -z "$sign" ]; then
  print_help
  exit 0
fi

repo_root="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [ -z "$repo_root" ]; then
  echo "Error: not inside a git repository." >&2
  exit 1
fi

cd "$repo_root"

if [ "$use_gh" = "true" ]; then
  if ! command -v gh >/dev/null 2>&1; then
    echo "Error: gh CLI not found. Install it or omit -g." >&2
    exit 1
  fi

  gh_name="$(gh api user --jq '.name // empty' 2>/dev/null || true)"
  gh_email="$(gh api user --jq '.email // empty' 2>/dev/null || true)"
  gh_login="$(gh api user --jq '.login // empty' 2>/dev/null || true)"
  gh_id="$(gh api user --jq '.id // empty' 2>/dev/null || true)"

  if [ -z "$name" ] && [ -n "$gh_name" ]; then
    name="$gh_name"
  fi

  if [ -z "$email" ] && [ -n "$gh_email" ]; then
    email="$gh_email"
  fi

  if [ -z "$email" ] && [ -n "$gh_id" ] && [ -n "$gh_login" ]; then
    email="${gh_id}+${gh_login}@users.noreply.github.com"
  fi
fi

if [ -z "$name" ]; then
  printf "Git user.name: "
  IFS= read -r name
fi

if [ -z "$email" ]; then
  printf "Git user.email: "
  IFS= read -r email
fi

git config --local user.name "$name"
git config --local user.email "$email"

if [ "$sign" = "true" ]; then
  git config --local commit.gpgsign true
fi

echo "Set repo-local git identity in $repo_root"
echo "user.name=$name"
echo "user.email=$email"
