# iwiywi — Sub-Plan 3: Distribution

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Release `iwiywi` under the `universal-grindset` GitHub org with a GitHub Actions release pipeline (builds binary, attaches to release) and a Homebrew tap for one-command install.

**Prerequisite:** Sub-Plans 1 + 2 complete. `cargo build --release` succeeds for `aarch64-apple-darwin`.

**Architecture:** On `git tag v*`, GitHub Actions cross-compiles for `aarch64-apple-darwin`, creates a GitHub Release, uploads the tarball, and prints the SHA256. The Homebrew formula in `universal-grindset/homebrew-iwiywi` is updated manually after each release (or automated in a future task).

**Tech Stack:** GitHub Actions, `cargo`, Homebrew Ruby formula

**Repos:**
- `https://github.com/universal-grindset/iwiywi` — main source
- `https://github.com/universal-grindset/homebrew-iwiywi` — tap

---

## File Map

| File | Responsibility |
|------|---------------|
| `.github/workflows/release.yml` | CI: build binary on tag, create GitHub Release |
| `homebrew-iwiywi/Formula/iwiywi.rb` | Homebrew formula (separate repo) |

---

## Task 1: GitHub Actions Release Workflow

**Files:**
- Create: `.github/workflows/release.yml`

- [ ] **Step 1: Create workflow directory**

```bash
mkdir -p .github/workflows
```

- [ ] **Step 2: Write release workflow**

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - "v*"

permissions:
  contents: write

jobs:
  build:
    name: Build and release
    runs-on: macos-14  # Apple Silicon runner

    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: aarch64-apple-darwin

      - name: Build release binary
        run: cargo build --release --target aarch64-apple-darwin

      - name: Package tarball
        run: |
          VERSION="${GITHUB_REF_NAME}"
          TARBALL="iwiywi-${VERSION}-aarch64-apple-darwin.tar.gz"
          tar -czf "$TARBALL" -C target/aarch64-apple-darwin/release iwiywi
          echo "TARBALL=$TARBALL" >> $GITHUB_ENV
          echo "VERSION=$VERSION" >> $GITHUB_ENV
          sha256sum "$TARBALL" > "${TARBALL}.sha256"
          cat "${TARBALL}.sha256"

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: |
            ${{ env.TARBALL }}
            ${{ env.TARBALL }}.sha256
          generate_release_notes: true
```

- [ ] **Step 3: Commit and tag**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add GitHub Actions release workflow"

# Push to GitHub (create repo first if needed)
gh repo create universal-grindset/iwiywi --public --push --source=.

# Tag and push to trigger release
git tag v0.1.0
git push origin main --tags
```

- [ ] **Step 4: Verify release was created**

```bash
gh release view v0.1.0 --repo universal-grindset/iwiywi
```

Expected: Release page shows `iwiywi-v0.1.0-aarch64-apple-darwin.tar.gz` and `.sha256` as assets.

- [ ] **Step 5: Note the SHA256 for the Homebrew formula**

```bash
gh release download v0.1.0 --repo universal-grindset/iwiywi --pattern "*.sha256"
cat iwiywi-v0.1.0-aarch64-apple-darwin.tar.gz.sha256
```

Copy this hash — you'll need it in Task 2.

---

## Task 2: Homebrew Tap

**Files:**
- Create: `Formula/iwiywi.rb` (in separate repo `universal-grindset/homebrew-iwiywi`)

- [ ] **Step 1: Create the tap repo**

```bash
gh repo create universal-grindset/homebrew-iwiywi --public --description "Homebrew tap for iwiywi"
git clone https://github.com/universal-grindset/homebrew-iwiywi.git /tmp/homebrew-iwiywi
cd /tmp/homebrew-iwiywi
mkdir Formula
```

- [ ] **Step 2: Write the formula**

Create `/tmp/homebrew-iwiywi/Formula/iwiywi.rb` — replace `<SHA256>` and `<VERSION>` with actual values from Task 1:

```ruby
class Iwiywi < Formula
  desc "It Works If You Work It — daily AA readings in your terminal"
  homepage "https://github.com/universal-grindset/iwiywi"
  version "<VERSION>"  # e.g. "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/universal-grindset/iwiywi/releases/download/v#{version}/iwiywi-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "<SHA256>"  # paste SHA256 from Task 1 Step 5
    end
  end

  def install
    bin.install "iwiywi"
  end

  test do
    assert_match "iwiywi", shell_output("#{bin}/iwiywi --help")
  end
end
```

- [ ] **Step 3: Commit and push formula**

```bash
cd /tmp/homebrew-iwiywi
git add Formula/iwiywi.rb
git commit -m "feat: add iwiywi formula v<VERSION>"
git push origin main
```

- [ ] **Step 4: Test tap locally**

```bash
brew tap universal-grindset/iwiywi
brew install universal-grindset/iwiywi/iwiywi
iwiywi --help
```

Expected: binary installs cleanly and `--help` works.

- [ ] **Step 5: Verify test block**

```bash
brew test universal-grindset/iwiywi/iwiywi
```

Expected: PASS.

---

## Updating the Formula for Future Releases

After each new tagged release:

1. Get new SHA256: `gh release download v<VERSION> --repo universal-grindset/iwiywi --pattern "*.sha256"`
2. Update `version` and `sha256` in `Formula/iwiywi.rb`
3. Commit: `git commit -m "feat: bump to v<VERSION>"`
4. Push to `homebrew-iwiywi` — users get it on next `brew upgrade`

---

## Done Signal

Sub-Plan 3 is complete when:
- `gh release view v0.1.0 --repo universal-grindset/iwiywi` shows the binary tarball asset
- `brew install universal-grindset/iwiywi/iwiywi` installs successfully on a clean Mac
- `brew test universal-grindset/iwiywi/iwiywi` passes
