# Release Automation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Pushing a `v*` tag publishes `bevy_tray_icon` to crates.io and creates a GitHub Release with label-grouped, auto-generated notes.

**Architecture:** Port `bevy_vrm1`'s setup. `.github/release.yml` controls how GitHub groups the auto-generated notes, and `.github/workflows/publish.yml` runs on tag push to check the version, publish, and create the Release. Docs drop the old `CHANGELOG.md` references.

**Tech Stack:** GitHub Actions, `dtolnay/rust-toolchain`, `softprops/action-gh-release@v2`, `cargo publish`, `gh` CLI.

**Spec:** `docs/superpowers/specs/2026-11-14-release-automation-design.md`

## Global Constraints

- Branch: `upgrade/bevy-0.20` (where `CHANGELOG.md` was removed). v0.4.0 is the first automated release.
- Workflow trigger: `push.tags: ["v*"]`.
- Secret name: `CARGO_REGISTRY_TOKEN` (the user creates it; never ask for or handle the token value).
- Release notes are generated from PRs only, through `generate_release_notes: true`. No changelog file.
- apt packages in the publish job must match `.github/workflows/ci.yml`: `libasound2-dev libudev-dev libwebkit2gtk-4.1-dev pkg-config libssl-dev build-essential libxdo-dev`.
- Commit messages follow the repo style (`add: …` / `update: …`) and end with `Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>`.

## Review Focus

1. A tag that doesn't match `Cargo.toml`, such as `v0.4.1` while `Cargo.toml` says `0.4.0`, must fail before `cargo publish`. Task 2, Step 3 runs the check script against a matching tag and a mismatched one.
2. The version check must read `[package].version`, not a dependency's `version = …` line. Task 2, Step 3 asserts it reads `0.4.0` from the current `Cargo.toml`.
3. A stale `Cargo.lock` makes `cargo publish --locked` fail in CI. Task 2, Step 4 runs `cargo publish --dry-run --locked` locally.
4. If the publish succeeds but creating the Release fails, a re-run hits "crate already exists" and never creates the Release. Task 2 documents the manual recovery in AGENTS.md: `gh release create vX.Y.Z --generate-notes`.
5. A PR with no recognized label must still show up in the notes. Task 1's test asserts that the last category is the catch-all `"*"`.

---

### Task 1: Release notes configuration

**Files:**
- Create: `.github/release.yml`
- Modify: `README.md` (the `## ChangeLog` section)
- GitHub repo: create the `improvement` label

**Interfaces:**
- Produces: the label names `breaking-change`, `enhancement`, `improvement`, `bug`, which PR authors use and AGENTS.md (Task 2) mentions.

- [ ] **Step 1: Write the check script**

Save it to the session scratchpad, not the repo, as `check_release_yml.py`:

```python
import sys, yaml
cfg = yaml.safe_load(open(".github/release.yml", encoding="utf-8"))
cats = cfg["changelog"]["categories"]
assert [c["title"] for c in cats] == [
    "Breaking Changes 🛠",
    "Exciting New Features 🎉",
    "Improvements to existing features",
    "Bug Fixes",
    "Other Changes",
], cats
assert [c["labels"] for c in cats] == [
    ["breaking-change"], ["enhancement"], ["improvement"], ["bug"], ["*"],
], cats
print("release.yml OK")
```

- [ ] **Step 2: Run it to verify it fails**

Run: `python <scratchpad>/check_release_yml.py`
Expected: FAIL with `FileNotFoundError` for `.github/release.yml`.

- [ ] **Step 3: Create `.github/release.yml`**

Use the same content as `$HOME/workspace/bevy_vrm1/.github/release.yml`, with the titles and labels from Step 1.

- [ ] **Step 4: Run it to verify it passes**

Run: `python <scratchpad>/check_release_yml.py`
Expected: `release.yml OK`

- [ ] **Step 5: Create the `improvement` label**

Run: `gh label create improvement -R not-elm/bevy_tray_icon --description "Improvement to existing features" --color f9d0c4`

Verify: `gh label list -R not-elm/bevy_tray_icon | grep -w improvement` prints one row.

- [ ] **Step 6: Update the README ChangeLog section**

Replace the link target `https://github.com/not-elm/bevy_tray_icon/blob/main/CHANGELOG.md` with `https://github.com/not-elm/bevy_tray_icon/releases`, and keep the section heading.

Verify: `grep -n CHANGELOG README.md` prints nothing.

- [ ] **Step 7: Commit**

```bash
git add .github/release.yml README.md
git commit -m "add: release notes config" -m "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"
```

---

### Task 2: Publish workflow

**Files:**
- Create: `.github/workflows/publish.yml`
- Modify: `AGENTS.md` (the `## Notes` section and the PR guidelines)

**Interfaces:**
- Consumes: the label names from Task 1 (mentioned in AGENTS.md).
- Produces: the workflow `Publish` with job `publish`.

- [ ] **Step 1: Write `.github/workflows/publish.yml`**

Port `$HOME/workspace/bevy_vrm1/.github/workflows/publish.yml`, keeping the same structure:
- `on.push.tags: ["v*"]`
- `concurrency: publish-${{ github.ref }}`
- `permissions: contents: write`
- one `ubuntu-latest` job with these steps: checkout, rust toolchain (stable), apt install, "Verify tag matches Cargo.toml version", `cargo publish --locked` with `CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}`, and `softprops/action-gh-release@v2` with `generate_release_notes: true`.

The only difference from bevy_vrm1 is the apt step: use `sudo apt-get update && sudo apt-get install --no-install-recommends` with the package list from Global Constraints.

- [ ] **Step 2: Lint the workflow**

Run: `actionlint .github/workflows/publish.yml`
Expected: no output, exit 0.

- [ ] **Step 3: Test the version check locally**

Copy the `run:` body of the "Verify tag matches Cargo.toml version" step into `<scratchpad>/verify_tag.sh` unchanged, then run:
- `GITHUB_REF_NAME=v0.4.0 bash <scratchpad>/verify_tag.sh; echo "exit=$?"` → expected `exit=0`
- `GITHUB_REF_NAME=v0.4.1 bash <scratchpad>/verify_tag.sh; echo "exit=$?"` → expected an `::error::Tag version (0.4.1) does not match Cargo.toml version (0.4.0)` line and `exit=1`

- [ ] **Step 4: Dry-run the publish**

Run: `cargo publish --dry-run --locked --allow-dirty`
Expected: ends with `warning: aborting upload due to dry run`, with no errors.

`--allow-dirty` is only needed if uncommitted files exist at this point. CI runs without it.

- [ ] **Step 5: Update AGENTS.md**

- In `## Notes`, change "update `README.md` and `CHANGELOG.md` accordingly" to "update the compatibility table in `README.md`".
- Add a `## Releasing` section:
  1. Bump `version` in `Cargo.toml` and merge to main.
  2. `git tag vX.Y.Z && git push origin vX.Y.Z`. The Publish workflow then publishes to crates.io and creates the GitHub Release with generated notes.
  3. Release notes come from merged PRs only. Label PRs with `breaking-change`, `enhancement`, `improvement`, or `bug` to group them.
  4. Requires the repository secret `CARGO_REGISTRY_TOKEN`.
  5. If publishing succeeded but creating the Release failed, run `gh release create vX.Y.Z --generate-notes`.

Verify: `grep -n CHANGELOG AGENTS.md` prints nothing.

- [ ] **Step 6: Commit**

```bash
git add .github/workflows/publish.yml AGENTS.md
git commit -m "add: publish workflow for crates.io releases" -m "Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>"
```
