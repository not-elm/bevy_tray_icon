# Release Automation Design

## Intent

Pushing a `v*` tag should publish the crate to crates.io and create a GitHub Release with auto-generated notes. The release notes replace the removed `CHANGELOG.md` and are written for library users.

## Goals

- `git push origin vX.Y.Z` publishes to crates.io and creates a GitHub Release with no manual steps.
- Release notes are generated from merged PRs, grouped by label, in the same format as `bevy_vrm1`.
- A tag that doesn't match the `Cargo.toml` version fails before publishing.

## Non-goals

- Commits pushed directly to main do not appear in the notes. Only PRs do.
- No automatic version bumping or changelog file.

## Approaches

| Approach | Trade-off | Decision |
|----------|-----------|----------|
| Port `bevy_vrm1`'s `publish.yml` + `release.yml` (GitHub-generated notes) | Same setup across the user's crates; PR-based notes only | **Chosen** |
| release-plz / git-cliff | Commit-based notes, but they expect Conventional Commits; this repo uses `add:` / `update:` | Rejected |
| crates.io Trusted Publishing (OIDC) instead of an API token | No long-lived secret, but differs from `bevy_vrm1` | Rejected for consistency |

## Design

1. **`.github/workflows/publish.yml`** (new): ported from `bevy_vrm1`. Runs on `v*` tag push and does three things:
   - checks that the tag matches the `Cargo.toml` version
   - runs `cargo publish --locked`, authenticated with `secrets.CARGO_REGISTRY_TOKEN`
   - runs `softprops/action-gh-release@v2` with `generate_release_notes: true`

   The apt packages differ from `bevy_vrm1`: they match this repo's `ci.yml` (`libwebkit2gtk-4.1-dev`, `libxdo-dev`, etc.), because `cargo publish` builds tray-icon while verifying the package.
2. **`.github/release.yml`** (new): the same categories as `bevy_vrm1`: Breaking Changes (`breaking-change`), New Features (`enhancement`), Improvements (`improvement`), Bug Fixes (`bug`), and Other Changes (`*`). The missing `improvement` label is created on the GitHub repo.
3. **README.md**: the ChangeLog section links to the GitHub Releases page instead of the deleted `CHANGELOG.md`.
4. **AGENTS.md**: drop the "update CHANGELOG.md" instruction and document the release steps: bump the `Cargo.toml` version, merge, then push the `vX.Y.Z` tag.
5. **Branch**: the work goes on `upgrade/bevy-0.20`, where `CHANGELOG.md` was removed. v0.4.0 becomes the first automated release.

## Manual setup (by the user)

- Create a crates.io API token with publish scope and store it as the repository secret `CARGO_REGISTRY_TOKEN`.

## Left to the builder

- How to verify the change locally, for example with `cargo publish --dry-run --locked` and a YAML sanity check.
