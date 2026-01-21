# Rucksack Rust Code Quality Audit - Detailed Implementation Plan

## Overview

This plan outlines the comprehensive Rust code quality audit for the Rucksack project and provides detailed, actionable steps for systematic modernization aligned with release milestones.

## Release Strategy

### Release 0.9.0 (Immediate - Critical Fixes)

**Goal:** Fix all panics and critical issues without breaking API

- Phase 1A: Critical unwrap/panic fixes
- Phase 1B: Urgent non-breaking improvements
- Test coverage improvements (separate work stream)

### Release 0.10.0 (Breaking Changes)

**Goal:** Complete all API-breaking modernization

- Phase 2A: Crucial breaking changes (String → PathBuf)
- Phase 2B: Lower priority breaking changes (API refinements)

### Release 0.11.0 (Polish & Enhancements)

**Goal:** Complete non-breaking improvements

- Phase 3: Performance optimizations
- Phase 4: Documentation completion
- Phase 5: Additional enhancements (non-audit items)

---

## Release 0.9.0: Critical Fixes

### Phase 1A: Critical unwrap/panic Fixes

**Priority:** 🔴 CRITICAL - Must complete before 0.9.0 release
**Breaking Changes:** None
**Estimated Effort:** 2-3 hours

#### Task 1: Fix Encryption unwrap() - SECURITY CRITICAL

**File:** `crates/rucksack-db/src/crypto.rs`

**Current Code (line 23):**

```rust
pub fn encrypt(data: Vec<u8>, pwd: String, salt: String) -> Vec<u8> {
    let key = sized_key(pwd, KeySize::KeySize256);
    let nonce = sized_nonce(salt);
    let cipher = Aes256Gcm::new(&key.into());
    cipher.encrypt(nonce, &data[..]).unwrap()  // ❌ SECURITY RISK
}
```

**Action:**

```rust
pub fn encrypt(data: Vec<u8>, pwd: String, salt: String) -> Result<Vec<u8>> {
    let key = sized_key(pwd, KeySize::KeySize256);
    let nonce = sized_nonce(salt);
    let cipher = Aes256Gcm::new(&key.into());

    cipher.encrypt(nonce, &data[..])
        .map_err(|e| anyhow!("encryption failed: {}", e))
}
```

**Callers to Update:**

1. Search for all calls to `crypto::encrypt()`:

   ```bash
   rg "crypto::encrypt\(" --type rust crates/
   ```

2. Update each caller to handle Result:

   ```rust
   // Before
   let encrypted = crypto::encrypt(data, pwd, salt);

   // After
   let encrypted = crypto::encrypt(data, pwd, salt)?;
   // or
   let encrypted = crypto::encrypt(data, pwd, salt)
       .context("failed to encrypt record")?;
   ```

**Testing:**

```bash
# Run crypto tests
cargo test -p rucksack-db crypto

# Verify encryption error handling
# Add test case for encryption failure if possible
```

**Validation Checklist:**

- [ ] Return type changed to `Result<Vec<u8>>`
- [ ] Error properly wrapped with anyhow
- [ ] All callers updated (compiler will help)
- [ ] Tests pass
- [ ] No unwrap remains in function

---

#### Task 2: Fix DB Lock panic()

**File:** `crates/rucksack-db/src/db/manager.rs:288`

**Current Code:**

```rust
pub fn update(/* ... */) {
    match self.hash_map.get_mut(&key) {
        Some(mut r) => {
            // update record
        }
        None => {
            let msg = "Couldn't get lock for update";
            log::error!("{}", msg);
            panic!("{}", msg)  // ❌ CRASHES APPLICATION
        }
    }
}
```

**Action:**

```rust
pub fn update(&mut self, key: String, /* ... */) -> Result<()> {
    let mut entry = self.hash_map.get_mut(&key)
        .ok_or_else(|| anyhow!("record '{}' not found or locked", key))?;

    // Update record logic
    // ... (preserve existing update code)

    Ok(())
}
```

**Callers to Update:**

1. Find all calls to `db.update()`:

   ```bash
   rg "\.update\(" crates/rucksack-db/src/ crates/rucksack/src/
   ```

2. Update each caller to handle Result:

   ```rust
   // Before
   db.update(key, /* ... */);

   // After
   db.update(key, /* ... */)?;
   // or
   db.update(key, /* ... */)
       .with_context(|| format!("failed to update record '{}'", key))?;
   ```

**Testing:**

```bash
# Test update scenarios
cargo test -p rucksack-db update

# Add test for concurrent access if needed
```

**Validation Checklist:**

- [ ] Return type changed to `Result<()>`
- [ ] None case returns error instead of panic
- [ ] All callers updated
- [ ] Tests pass
- [ ] Error message is clear and actionable

---

#### Task 3: Fix Handler panic()

**File:** `crates/rucksack/src/command/handlers/show.rs:67`

**Current Code:**

```rust
pub fn run(matches: &ArgMatches, app: &App) -> Result<()> {
    match file::read(file_name) {
        Ok(data) => {
            // process data
        }
        Err(e) => panic!("{}", e),  // ❌ CRASHES CLI
    }
}
```

**Action:**

```rust
pub fn run(matches: &ArgMatches, app: &App) -> Result<()> {
    let data = file::read(file_name)
        .with_context(|| format!("failed to read file '{}'", file_name))?;

    // process data (preserve existing logic)

    Ok(())
}
```

**Testing:**

```bash
# Test show command
cargo test -p rucksack handlers::show

# Manual test with non-existent file
cargo run -- show nonexistent.txt
# Should show error, not panic
```

**Validation Checklist:**

- [ ] panic!() removed
- [ ] Error propagated with ?
- [ ] Context added for clarity
- [ ] Tests pass
- [ ] Manual test shows nice error message

---

#### Task 4: Fix Library Version unwrap() Calls

**Files:**

- `crates/rucksack-db/src/lib.rs:16`
- `crates/rucksack-lib/src/lib.rs:7`
- `crates/rucksack-db/src/records/mod.rs:19`

**Current Code (all similar):**

```rust
pub fn version() -> versions::SemVer {
    versions::SemVer::new(env!("CARGO_PKG_VERSION")).unwrap()
}
```

**Action:**

```rust
pub fn version() -> versions::SemVer {
    versions::SemVer::new(env!("CARGO_PKG_VERSION"))
        .expect("CARGO_PKG_VERSION must be valid semver format")
}
```

**Rationale:**

- `env!("CARGO_PKG_VERSION")` is compile-time constant from Cargo.toml
- If it fails, it's a build configuration error (should never happen)
- `expect()` with clear message is acceptable for invariants
- Better than `unwrap()` because it documents why it's safe

**Testing:**

```bash
# Verify versions can be retrieved
cargo test -p rucksack-db version
cargo test -p rucksack-lib version

# Check that versions are valid
cargo run -- --version
```

**Validation Checklist:**

- [ ] All 3 files updated
- [ ] unwrap() replaced with expect() + message
- [ ] Tests pass
- [ ] CLI --version works

---

#### Task 5: Fix VersionedDB.version() unwrap()

**File:** `crates/rucksack-db/src/db/versioned.rs:61`

**Current Code:**

```rust
pub fn version(&self) -> versions::SemVer {
    versions::SemVer::new(self.version.as_str()).unwrap()
}
```

**Strategy:** Validate version in constructor

**Action:**

**Step 1 - Update constructor (around line 20):**

```rust
impl VersionedDB {
    pub fn new(bytes: Vec<u8>, version: String) -> Result<Self> {
        // Validate version format immediately
        versions::SemVer::new(&version)
            .map_err(|e| anyhow!("invalid version format '{}': {}", version, e))?;

        Ok(Self { bytes, version })
    }

    // Alternative: from_parts if you need fallible construction
    pub fn from_bytes(bytes: Vec<u8>, version: String) -> Result<Self> {
        Self::new(bytes, version)
    }
}
```

**Step 2 - Update version() method:**

```rust
pub fn version(&self) -> versions::SemVer {
    // SAFETY: Version string is validated in constructor
    versions::SemVer::new(self.version.as_str())
        .expect("version validated in constructor")
}
```

**Step 3 - Update all constructor call sites:**

```bash
# Find all VersionedDB::new() calls
rg "VersionedDB::new\(|VersionedDB \{" --type rust crates/
```

Update to handle Result:

```rust
// Before
let db = VersionedDB::new(bytes, version);

// After
let db = VersionedDB::new(bytes, version)?;
```

**Testing:**

```bash
cargo test -p rucksack-db versioned

# Add test for invalid version
#[test]
fn test_versioned_db_invalid_version() {
    let result = VersionedDB::new(vec![], "invalid-version".to_string());
    assert!(result.is_err());
}
```

**Validation Checklist:**

- [ ] Constructor validates version
- [ ] Constructor returns Result
- [ ] version() method uses expect() with comment
- [ ] All callers updated
- [ ] Tests pass
- [ ] Test added for invalid version

---

#### Task 6: Fix shared::version() unwrap()

**File:** `crates/rucksack-db/src/records/shared.rs:9`

**Current Code:**

```rust
pub fn version(v: &str) -> versions::SemVer {
    trim_version(versions::SemVer::new(v).unwrap())
}
```

**Action:**

```rust
pub fn version(v: &str) -> Result<versions::SemVer> {
    versions::SemVer::new(v)
        .map(trim_version)
        .map_err(|e| anyhow!("invalid version string '{}': {}", v, e))
}
```

**Callers to Update:**

```bash
rg "shared::version\(" --type rust crates/rucksack-db/src/
```

Update each caller:

```rust
// Before
let ver = shared::version(version_str);

// After
let ver = shared::version(version_str)?;
```

**Testing:**

```bash
cargo test -p rucksack-db shared

# Add test for invalid version
#[test]
fn test_version_invalid() {
    let result = shared::version("not-a-version");
    assert!(result.is_err());
}
```

**Validation Checklist:**

- [ ] Return type changed to Result
- [ ] Error properly wrapped
- [ ] All callers updated
- [ ] Tests pass
- [ ] Test added for invalid input

---

#### Phase 1A Completion Checklist

**Before Release:**

- [ ] All 6 tasks completed
- [ ] Zero unwrap() in non-test library code

  ```bash
  rg "\.unwrap\(\)" --type rust crates/rucksack-db/src/ | grep -v test | grep -v "^//"
  rg "\.unwrap\(\)" --type rust crates/rucksack-lib/src/ | grep -v test | grep -v "^//"
  ```

- [ ] Zero panic!() for recoverable errors

  ```bash
  rg "panic!\(" --type rust crates/ | grep -v test | grep -v "unreachable"
  ```

- [ ] All tests pass

  ```bash
  cargo test --all
  ```

- [ ] Clippy clean on unwrap/panic

  ```bash
  cargo clippy --all-targets -- -W clippy::unwrap_used -W clippy::panic
  ```

- [ ] Manual CLI testing

  ```bash
  cargo run -- --version
  cargo run -- add --url test.com --user test --password pass
  cargo run -- list
  cargo run -- show test.com
  ```

---

### Phase 1B: Urgent Non-Breaking Fixes

**Priority:** 🟠 HIGH - Complete before 0.9.0 release
**Breaking Changes:** None
**Estimated Effort:** 2-4 hours

These are improvements that don't break the API but significantly improve code quality.

---

#### Task 7: Add Safety Documentation to Remaining unwrap() Calls

**Goal:** Document why remaining unwrap() calls are safe

**Files to Audit:**

```bash
rg "\.unwrap\(\)" --type rust crates/ | grep -v test | grep -v "^//"
```

**For each remaining unwrap(), add comment:**

```rust
// SAFETY: Password generator library guarantees successful generation
// with valid parameters. This unwrap is safe because we use default
// settings that are known to work.
let password = pg.generate_one().unwrap();

// SAFETY: Epoch 0 (1970-01-01 00:00:00 UTC) is always a valid timestamp.
// This unwrap will never fail.
let epoch = Utc.timestamp_millis_opt(0).unwrap();

// SAFETY: Path operations on non-root paths always have a parent.
// We verify path is not root before calling this.
let parent = path.parent().unwrap();
```

**Validation:**

- [ ] All non-test unwrap() calls have SAFETY comments
- [ ] Comments explain why unwrap is safe
- [ ] Comments reference preconditions or invariants

---

#### Task 8: Fix Generator unwrap() (Defensive)

**File:** `crates/rucksack-lib/src/generator/password.rs`

**Current Code:**

```rust
pub fn generate_password(/* ... */) -> String {
    let pg = PasswordGenerator::new(/* ... */);
    pg.generate_one().unwrap()
}
```

**Action (Defensive Programming):**

```rust
pub fn generate_password(/* ... */) -> Result<String> {
    let pg = PasswordGenerator::new(/* ... */);
    pg.generate_one()
        .map_err(|e| anyhow!("password generation failed: {}", e))
}
```

**Callers to Update:**

```bash
rg "generate_password\(" --type rust crates/
```

**Note:** This is defensive - the library likely guarantees success, but returning Result is safer.

**Validation:**

- [ ] Returns Result instead of unwrapping
- [ ] Callers updated
- [ ] Tests pass

---

#### Task 9: Improve Error Messages with Context

**Goal:** Add context to error propagation throughout codebase

**Pattern to Apply:**

```rust
// ❌ Less helpful
std::fs::read_to_string(path)?

// ✅ More helpful
std::fs::read_to_string(&path)
    .with_context(|| format!("failed to read config file: {}", path.display()))?
```

**Files to Review:**

- `crates/rucksack-db/src/db/manager.rs` - Database operations
- `crates/rucksack-db/src/crypto.rs` - Encryption errors
- `crates/rucksack-lib/src/file.rs` - File operations
- `crates/rucksack/src/command/handlers/*.rs` - Command handlers

**Action:**
Add `.with_context()` to all error propagation sites where context is helpful:

```rust
// File operations
file::read(path)
    .with_context(|| format!("failed to read database file: {}", path))?

// Database operations
db.insert(key, value)
    .with_context(|| format!("failed to insert record '{}'", key))?

// Encryption operations
crypto::decrypt(data, pwd, salt)
    .context("failed to decrypt database - password may be incorrect")?
```

**Validation:**

- [ ] Major error paths have context
- [ ] Error messages are user-friendly
- [ ] Tests pass

---

#### Task 10: Add #[must_use] Attributes

**Goal:** Prevent accidental error ignoring

**Pattern:**

```rust
#[must_use]
pub fn encrypt(/* ... */) -> Result<Vec<u8>> { }

#[must_use]
pub fn decrypt(/* ... */) -> Result<Vec<u8>> { }

#[must_use = "database operations must be checked for errors"]
pub fn insert(&mut self, /* ... */) -> Result<()> { }
```

**Files:**

- `crates/rucksack-db/src/crypto.rs` - All functions
- `crates/rucksack-db/src/db/manager.rs` - All Result-returning methods
- `crates/rucksack-lib/src/file.rs` - All Result-returning functions

**Validation:**

```bash
cargo build --all
# Compiler will warn on unused Results
```

---

#### Task 11: Fix Inconsistent Secret Handling

**File:** `crates/rucksack-db/src/db/manager.rs`

**Current Code:**

```rust
pub struct DB {
    salt: Option<String>,       // ⚠️ Should be SecretString
    store_pwd: Option<String>,  // ⚠️ Should be SecretString
}
```

**Action:**

```rust
use secrecy::SecretString;

pub struct DB {
    salt: Option<SecretString>,
    store_pwd: Option<SecretString>,
    // ...
}

impl DB {
    pub fn salt(&self) -> Option<&SecretString> {
        self.salt.as_ref()
    }

    pub fn store_pwd(&self) -> Option<&SecretString> {
        self.store_pwd.as_ref()
    }
}
```

**Impact:** Non-breaking if only internal usage (check callers)

**Callers to Update:**

```bash
rg "\.salt\(\)|\.store_pwd\(\)" --type rust crates/
```

Update to use `expose_secret()` when needed:

```rust
// Before
let salt_str = db.salt();

// After
let salt_str = db.salt()
    .map(|s| s.expose_secret())
    .ok_or_else(|| anyhow!("salt not set"))?;
```

**Validation:**

- [ ] Fields changed to SecretString
- [ ] Accessors return references
- [ ] Callers updated
- [ ] Tests pass
- [ ] Secrets not in debug output

---

#### Phase 1B Completion Checklist

**Before Release:**

- [ ] Tasks 7-11 completed
- [ ] All safety comments added
- [ ] Error messages improved
- [ ] #[must_use] attributes added
- [ ] Secret handling consistent
- [ ] All tests pass
- [ ] Manual testing confirms improvements

---

### Release 0.9.0 - Final Validation

**Complete Checklist:**

- [ ] Phase 1A complete (critical fixes)
- [ ] Phase 1B complete (urgent non-breaking)
- [ ] Test coverage work complete (separate stream)
- [ ] All tests pass: `cargo test --all`
- [ ] Coverage ≥95%: `make coverage`
- [ ] Clippy clean: `make lint`
- [ ] Format clean: `make format`
- [ ] Manual CLI smoke tests pass
- [ ] CHANGELOG.md updated
- [ ] Version bumped to 0.9.0 in all Cargo.toml files
- [ ] Git commit with message: "Release 0.9.0: Critical fixes and test coverage"

---

## Release 0.10.0: Breaking Changes

### Phase 2A: Crucial Breaking Changes

**Priority:** 🔴 CRITICAL for 0.10.0
**Breaking Changes:** YES - Major API changes
**Estimated Effort:** 6-10 hours

This phase includes the most important breaking changes that modernize the core APIs.

---

#### Task 12: String → PathBuf Migration (Systematic)

**Goal:** Replace all String usage for file paths with PathBuf

**Scope:** This is the largest refactor, affecting 20+ locations

##### Step 1: Update Core Structures

**File:** `crates/rucksack-db/src/db/manager.rs`

**Current:**

```rust
pub struct DB {
    pub file_name: String,
    backup_dir: String,
    // ...
}
```

**Action:**

```rust
use std::path::{Path, PathBuf};

pub struct DB {
    file_name: PathBuf,        // Changed
    backup_dir: PathBuf,       // Changed
    // ... (rest unchanged)
}

impl DB {
    pub fn new(
        file_name: impl Into<PathBuf>,
        backup_dir: impl Into<PathBuf>,
        // ... other params
    ) -> Self {
        Self {
            file_name: file_name.into(),
            backup_dir: backup_dir.into(),
            // ...
        }
    }

    // Getters return Path references
    pub fn file_name(&self) -> &Path {
        &self.file_name
    }

    pub fn backup_dir(&self) -> &Path {
        &self.backup_dir
    }

    // Setters (if needed) take impl Into<PathBuf>
    pub fn set_file_name(&mut self, path: impl Into<PathBuf>) {
        self.file_name = path.into();
    }
}
```

**Validation:**

```bash
cargo build -p rucksack-db
# Compiler will show all places that need updating
```

---

##### Step 2: Update EncryptedDB

**File:** `crates/rucksack-db/src/db/encrypted.rs`

**Current:**

```rust
pub struct EncryptedDB {
    path: String,
    // ...
}
```

**Action:**

```rust
use std::path::{Path, PathBuf};

pub struct EncryptedDB {
    path: PathBuf,
    // ...
}

impl EncryptedDB {
    pub fn new(
        bytes: Vec<u8>,
        path: impl Into<PathBuf>,
        pwd: SecretString,
        salt: SecretString,
    ) -> Self {
        Self {
            bytes,
            path: path.into(),
            pwd,
            salt,
            // ...
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}
```

---

##### Step 3: Update StoreManager Trait

**File:** `crates/rucksack-db/src/store/manager.rs`

**Current:**

```rust
pub trait StoreManager {
    fn backup(&self, src_file: String, dest_dir: String, version: String) -> Result<String>;
    fn read(&self, path: String, pwd: String, salt: String) -> Result<EncryptedDB>;
    fn write(&self, db: &EncryptedDB) -> Result<()>;
}
```

**Action:**

```rust
use std::path::{Path, PathBuf};

pub trait StoreManager {
    fn backup(
        &self,
        src_file: &Path,
        dest_dir: &Path,
        version: &str,
    ) -> Result<PathBuf>;

    fn read(
        &self,
        path: &Path,
        pwd: &str,
        salt: &str,
    ) -> Result<EncryptedDB>;

    fn write(&self, db: &EncryptedDB) -> Result<()>;
}
```

**Note:** Also changed String → &str for pwd/salt/version (Task 13)

---

##### Step 4: Update FileSystemBackend

**File:** `crates/rucksack-db/src/store/backend/filesystem.rs`

**Action:**

```rust
use std::path::{Path, PathBuf};

pub struct FileSystemBackend;

impl StoreManager for FileSystemBackend {
    fn backup(
        &self,
        src_file: &Path,
        dest_dir: &Path,
        version: &str,
    ) -> Result<PathBuf> {
        backup::copy(src_file, dest_dir, version)
    }

    fn read(
        &self,
        path: &Path,
        pwd: &str,
        salt: &str,
    ) -> Result<EncryptedDB> {
        // Update implementation
        let bytes = std::fs::read(path)
            .with_context(|| format!("failed to read database: {}", path.display()))?;

        let encrypted = EncryptedDB::new(
            bytes,
            path,
            SecretString::new(pwd.to_string()),
            SecretString::new(salt.to_string()),
        );

        encrypted.from_encrypted()
    }

    fn write(&self, db: &EncryptedDB) -> Result<()> {
        std::fs::write(db.path(), db.bytes())
            .with_context(|| format!("failed to write database: {}", db.path().display()))
    }
}
```

---

##### Step 5: Update ReDBBackend

**File:** `crates/rucksack-db/src/store/backend/redb.rs`

**Action:** Similar to FileSystemBackend - update all methods to use Path/PathBuf

---

##### Step 6: Update Backup Module

**File:** `crates/rucksack-db/src/store/backend/backup.rs`

**Current:**

```rust
pub fn copy(src_file: String, dest_dir: String, version: String) -> Result<String>
pub fn backup_name(src_file: String, version: String) -> String
pub fn list(backup_dir: String) -> Result<file::Listing>
pub fn restore(backup_path: PathBuf, old_name: String, dest_path: PathBuf) -> Result<()>
```

**Action:**

```rust
use std::path::{Path, PathBuf};

pub fn copy(
    src_file: &Path,
    dest_dir: &Path,
    version: &str,
) -> Result<PathBuf> {
    let backup_path = backup_path(src_file, dest_dir, version);
    std::fs::copy(src_file, &backup_path)
        .with_context(|| format!(
            "failed to copy {} to {}",
            src_file.display(),
            backup_path.display()
        ))?;
    Ok(backup_path)
}

pub fn backup_name(src_file: &Path, version: &str) -> String {
    format!(
        "{}.{}.bak",
        src_file.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("database"),
        version
    )
}

pub fn list(backup_dir: &Path) -> Result<file::Listing> {
    file::files(backup_dir)
}

pub fn restore(
    backup_path: &Path,
    dest_path: &Path,
) -> Result<()> {
    std::fs::copy(backup_path, dest_path)
        .with_context(|| format!(
            "failed to restore {} to {}",
            backup_path.display(),
            dest_path.display()
        ))?;
    Ok(())
}
```

---

##### Step 7: Update rucksack-lib file Module

**File:** `crates/rucksack-lib/src/file.rs`

**Current:**

```rust
pub fn abs_path(path_name: String) -> io::Result<PathBuf>
pub fn config_file(project: &str) -> String
pub fn db_file(project: &str) -> String
pub fn dir_parent(dir: String) -> String
pub fn files(dir: String) -> Result<Listing>
pub fn read(file_name: String) -> Result<Vec<u8>>
pub fn write(data: Vec<u8>, path: String) -> Result<()>
```

**Action:**

```rust
use std::path::{Path, PathBuf};

pub fn abs_path(path_name: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path_name.as_ref();
    let expanded = shellexpand::tilde(
        path.to_str()
            .ok_or_else(|| io::Error::new(
                io::ErrorKind::InvalidInput,
                "path contains invalid UTF-8"
            ))?
    );
    let clean = path_clean::clean(&*expanded);
    clean.canonicalize()
}

pub fn config_file(project: &str) -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(project)
        .join("config.toml")
}

pub fn db_file(project: &str) -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(project)
        .join("secrets.db")
}

pub fn dir_parent(dir: impl AsRef<Path>) -> Option<PathBuf> {
    dir.as_ref().parent().map(|p| p.to_path_buf())
}

pub fn files(dir: impl AsRef<Path>) -> Result<Listing> {
    let dir = dir.as_ref();
    let entries = std::fs::read_dir(dir)
        .with_context(|| format!("failed to read directory: {}", dir.display()))?;

    // ... rest of implementation
}

pub fn read(file_name: impl AsRef<Path>) -> Result<Vec<u8>> {
    let path = file_name.as_ref();
    let expanded_path = abs_path(path)?;

    std::fs::read(&expanded_path)
        .with_context(|| format!("failed to read file: {}", expanded_path.display()))
}

pub fn write(data: Vec<u8>, path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();
    let parent = path.parent()
        .ok_or_else(|| anyhow!("path has no parent directory"))?;

    if !parent.exists() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory: {}", parent.display()))?;
    }

    std::fs::write(path, data)
        .with_context(|| format!("failed to write file: {}", path.display()))
}
```

---

##### Step 8: Update All Callers

**Strategy:** Let the compiler guide you

```bash
# Build will show all call sites that need updating
cargo build --all 2>&1 | tee /tmp/build_errors.txt

# Common patterns:
# 1. String literals: Just pass as-is, Into<PathBuf> handles it
file::read("config.toml")  // Still works!

# 2. String variables: Convert to PathBuf or pass as &str
let path_str: String = get_path();
file::read(path_str)  // Into<PathBuf> converts

# 3. Owned PathBuf: Pass directly
let path: PathBuf = get_path();
file::read(&path)  // or just path

# 4. Format into path:
let path = format!("{}/data.db", dir);
file::read(path)  // Into<PathBuf> handles String
```

---

##### Step 9: Update Tests

**All test files need updating:**

```bash
find crates/*/tests -name "*.rs" -type f
```

**Common test patterns:**

```rust
// Before
let path = "/tmp/test.db".to_string();
db.set_file_name(path);

// After
let path = PathBuf::from("/tmp/test.db");
db.set_file_name(path);
// Or just:
db.set_file_name("/tmp/test.db");
```

**Use TempDir for tests:**

```rust
use tempfile::TempDir;

#[test]
fn test_database_operations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("test.db");

    let db = DB::new(db_path, temp_dir.path(), /* ... */);
    // ...
}
```

---

##### Step 10: Validation

```bash
# Must build successfully
cargo build --all-features

# All tests must pass
cargo test --all

# Clippy should be happy
cargo clippy --all-targets --all-features

# Manual testing
cargo run -- --help
cargo run -- add --url test.com --user test --password pass
cargo run -- list
```

---

##### Task 12 Completion Checklist

- [ ] All structs use PathBuf for paths
- [ ] All functions accept impl AsRef<Path>
- [ ] All functions return PathBuf (owned) or &Path (borrowed)
- [ ] StoreManager trait updated
- [ ] FileSystemBackend updated
- [ ] ReDBBackend updated
- [ ] Backup module updated
- [ ] file.rs module updated
- [ ] All callers updated (compiler verified)
- [ ] All tests updated and passing
- [ ] Manual CLI testing passes

---

#### Task 13: String → &str for Read-Only Parameters

**Goal:** Functions that don't need ownership should accept &str

**Scope:** Crypto functions, utility functions, configuration

##### Step 1: Update Crypto Module

**File:** `crates/rucksack-db/src/crypto.rs`

**Current:**

```rust
pub fn encrypt(data: Vec<u8>, pwd: String, salt: String) -> Result<Vec<u8>>
pub fn decrypt(encrypted: Vec<u8>, pwd: String, salt: String) -> Result<Vec<u8>>
fn sized_key(source: String, key_size: KeySize) -> Vec<u8>
fn sized_nonce(source: String) -> Vec<u8>
```

**Action:**

```rust
pub fn encrypt(data: Vec<u8>, pwd: &str, salt: &str) -> Result<Vec<u8>> {
    let key = sized_key(pwd, KeySize::KeySize256);
    let nonce = sized_nonce(salt);
    // ... rest unchanged (already returns Result from Task 1)
}

pub fn decrypt(encrypted: Vec<u8>, pwd: &str, salt: &str) -> Result<Vec<u8>> {
    let key = sized_key(pwd, KeySize::KeySize256);
    let nonce = sized_nonce(salt);
    // ... rest unchanged
}

fn sized_key(source: &str, key_size: KeySize) -> Vec<u8> {
    let bytes = source.as_bytes();
    // ... rest unchanged
}

fn sized_nonce(source: &str) -> Vec<u8> {
    let bytes = source.as_bytes();
    // ... rest unchanged
}
```

**Benefits:**

```rust
// Before: Had to own or clone
crypto::encrypt(data, password.clone(), salt.clone());

// After: Just borrow
crypto::encrypt(data, &password, &salt);
crypto::encrypt(data, "literal_pwd", "literal_salt");  // Works!
```

---

##### Step 2: Update StoreManager (already done in Task 12)

The StoreManager trait was already updated to use &str for pwd/salt/version in Task 12 Step 3.

---

##### Step 3: Update Callers

**Pattern:**

```rust
// Before
let pwd: String = get_password();
let salt: String = get_salt();
crypto::encrypt(data, pwd, salt);

// After
let pwd: String = get_password();
let salt: String = get_salt();
crypto::encrypt(data, &pwd, &salt);
// Or with SecretString:
crypto::encrypt(data, pwd.expose_secret(), salt.expose_secret());
```

**Validation:**

```bash
cargo build -p rucksack-db
# Compiler will show all call sites
```

---

##### Task 13 Completion Checklist

- [ ] crypto.rs functions accept &str
- [ ] All internal crypto functions accept &str
- [ ] All callers updated
- [ ] Tests updated
- [ ] Tests pass

---

#### Task 14: Make DB.file_name Private

**Goal:** Encapsulate implementation, provide controlled access

**File:** `crates/rucksack-db/src/db/manager.rs`

**Current:**

```rust
pub struct DB {
    pub file_name: PathBuf,  // ❌ Public
    // ...
}
```

**Action:**

```rust
pub struct DB {
    file_name: PathBuf,  // ✅ Private
    // ...
}

impl DB {
    // Getter returns reference
    pub fn file_name(&self) -> &Path {
        &self.file_name
    }

    // Setter with validation (if mutation needed)
    pub fn set_file_name(&mut self, path: impl Into<PathBuf>) -> Result<()> {
        let new_path = path.into();

        // Validation example
        if let Some(parent) = new_path.parent() {
            if !parent.exists() {
                return Err(anyhow!(
                    "parent directory does not exist: {}",
                    parent.display()
                ));
            }
        }

        self.file_name = new_path;
        Ok(())
    }
}
```

**Callers to Update:**

```bash
rg "\.file_name" --type rust crates/
```

```rust
// Before
let path = db.file_name.clone();
db.file_name = new_path;

// After
let path = db.file_name().to_path_buf();  // If you need owned
let path = db.file_name();  // If reference is enough
db.set_file_name(new_path)?;
```

**Validation:**

- [ ] Field is private
- [ ] Getter provided
- [ ] Setter provided (if needed)
- [ ] All callers updated
- [ ] Tests pass

---

### Phase 2A Completion Checklist

**Before Proceeding to Phase 2B:**

- [ ] Task 12 complete (PathBuf migration)
- [ ] Task 13 complete (&str parameters)
- [ ] Task 14 complete (private file_name)
- [ ] All breaking changes documented in CHANGELOG.md
- [ ] Migration guide written for users
- [ ] All tests pass
- [ ] Manual testing confirms no regressions

---

### Phase 2B: Lower Priority Breaking Changes

**Priority:** 🟡 MEDIUM for 0.10.0
**Breaking Changes:** YES - API refinements
**Estimated Effort:** 2-4 hours

---

#### Task 15: Consistent SecretString Usage

**Goal:** Use SecretString for all passwords/salts throughout

**Already started in Task 11, now make it pervasive**

**Files:**

- Any remaining String usage for passwords/salts
- Function parameters that take secrets

**Action:**

```rust
// Anywhere secrets are passed, use SecretString
pub fn some_function(password: &SecretString) {
    // Use expose_secret() when needed
    let pwd_str = password.expose_secret();
    // ...
}
```

**Validation:**

```bash
rg "pwd|password|salt" --type rust crates/ | grep -i "string"
# Review each to ensure SecretString is used appropriately
```

---

#### Task 16: Remove Unnecessary Public Exports

**Goal:** Tighten API surface, make internals truly internal

**Audit public exports:**

```bash
rg "^pub " --type rust crates/rucksack-db/src/ crates/rucksack-lib/src/
```

**Action:**

```rust
// Make internal modules/functions pub(crate)
pub(crate) fn internal_helper() { }

pub(crate) mod internal {
    // ...
}

// Keep truly public items as pub
pub fn public_api() { }
```

**Validation:**

- Build succeeds
- No external crates break

---

#### Task 17: Add #[non_exhaustive] to Public Enums

**Goal:** Allow future enum additions without breaking changes

**Files:** Search for public enums

```bash
rg "^pub enum" --type rust crates/rucksack-db/src/
```

**Action:**

```rust
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Archived,
    Deleted,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Password,
    Account,
    // Can add more variants without breaking
}
```

**Validation:**

- Build succeeds
- Tests pass

---

### Phase 2B Completion Checklist

- [ ] Task 15 complete (SecretString usage)
- [ ] Task 16 complete (tighten exports)
- [ ] Task 17 complete (#[non_exhaustive])
- [ ] All tests pass
- [ ] Clippy clean

---

### Release 0.10.0 - Final Validation

**Complete Checklist:**

- [ ] Phase 2A complete (crucial breaking changes)
- [ ] Phase 2B complete (lower priority breaking changes)
- [ ] CHANGELOG.md updated with all breaking changes
- [ ] Migration guide written (0.9.0 → 0.10.0)
- [ ] All tests pass: `cargo test --all`
- [ ] Coverage maintained ≥95%
- [ ] Clippy clean: `make lint`
- [ ] Format clean: `make format`
- [ ] Manual CLI smoke tests pass
- [ ] Version bumped to 0.10.0 in all Cargo.toml files
- [ ] Git commit: "Release 0.10.0: Breaking changes - PathBuf migration and API modernization"

---

## Release 0.11.0: Polish & Enhancements

### Phase 3: Performance Optimizations

**Priority:** 🟢 LOW for 0.11.0
**Breaking Changes:** None
**Estimated Effort:** 3-5 hours

---

#### Task 18: Remove Unnecessary Clones from Getters

**Goal:** Return references instead of clones

**Files:**

- `crates/rucksack-db/src/db/versioned.rs`
- `crates/rucksack-db/src/db/encrypted.rs`
- `crates/rucksack-db/src/db/manager.rs`

**Current Pattern:**

```rust
pub fn bytes(&self) -> Vec<u8> {
    self.bytes.clone()  // ❌ Unnecessary allocation
}
```

**Action:**

```rust
pub fn bytes(&self) -> &[u8] {
    &self.bytes  // ✅ Zero-cost reference
}

// If caller needs owned:
// let owned = db.bytes().to_vec();
```

**All getters to update:**

```rust
// versioned.rs
pub fn bytes(&self) -> &[u8] { &self.bytes }
pub fn version_str(&self) -> &str { &self.version }

// encrypted.rs
pub fn bytes(&self) -> &[u8] { &self.bytes }
pub fn path(&self) -> &Path { &self.path }  // Already done in Task 12

// manager.rs
pub fn backup_dir(&self) -> &Path { &self.backup_dir }
pub fn file_name(&self) -> &Path { &self.file_name }
// ... etc
```

**Callers:**
Most callers won't need changes - they can use references directly.
If they need owned, add `.to_vec()` / `.to_path_buf()` explicitly.

**Validation:**

```bash
cargo test --all
```

---

#### Task 19: Optimize Iteration Clones

**Goal:** Remove clones from loops

**File:** `crates/rucksack-db/src/db/manager.rs`

**Current (around line 240):**

```rust
let mut data: Vec<(String, EncryptedRecord)> = vec![];
for i in self.hash_map.iter() {
    data.push((i.key().clone(), i.value().clone()));
}
data.sort_by_key(|k| k.0.clone());
```

**Action:**

```rust
// Option 1: Return references (non-breaking if internal)
pub(crate) fn records_sorted(&self) -> Vec<(&str, &EncryptedRecord)> {
    let mut data: Vec<_> = self.hash_map.iter()
        .map(|entry| (entry.key().as_str(), entry.value()))
        .collect();

    data.sort_by_key(|k| k.0);  // No clone needed
    data
}

// Option 2: If ownership truly needed, move instead of clone
pub fn into_records_sorted(self) -> Vec<(String, EncryptedRecord)> {
    let mut data: Vec<_> = self.hash_map.into_iter().collect();
    data.sort_by_key(|k| k.0.clone());  // Clone for sort key is acceptable
    data
}
```

**Validation:**

```bash
cargo test -p rucksack-db
# Benchmark if needed
```

---

#### Task 20: Review and Optimize String Operations

**Goal:** Find string allocation hot spots

**Action:**

```bash
# Find format! in loops
rg "for .* \{" -A 10 --type rust crates/ | rg "format!"

# Find string concatenation
rg "\+ \"" --type rust crates/
```

**Optimize where found:**

```rust
// Before: Multiple allocations
let mut result = String::new();
for item in items {
    result = result + &item.to_string() + ", ";
}

// After: Pre-allocate and push
let mut result = String::with_capacity(items.len() * 20);
for item in items {
    result.push_str(&item.to_string());
    result.push_str(", ");
}

// Or better: Use join
let result = items.iter()
    .map(|item| item.to_string())
    .collect::<Vec<_>>()
    .join(", ");
```

---

### Phase 3 Completion Checklist

- [ ] Task 18 complete (getter clones removed)
- [ ] Task 19 complete (iteration optimized)
- [ ] Task 20 complete (string operations optimized)
- [ ] All tests pass
- [ ] Benchmarks show improvement or no regression
- [ ] No performance regressions in CLI usage

---

### Phase 4: Documentation Completion

**Priority:** 🟢 LOW for 0.11.0
**Breaking Changes:** None
**Estimated Effort:** 4-6 hours

---

#### Task 21: Document All Public Functions

**Goal:** Every public function has /// doc comment

**Pattern:**

```rust
/// Encrypts data using AES-256-GCM.
///
/// # Arguments
///
/// * `data` - The plaintext data to encrypt
/// * `pwd` - Password used to derive encryption key
/// * `salt` - Salt used for key derivation
///
/// # Errors
///
/// Returns an error if:
/// - Encryption fails (data too large, cipher error)
/// - Key derivation fails
///
/// # Examples
///
/// ```
/// use rucksack_db::crypto;
///
/// let encrypted = crypto::encrypt(
///     b"secret data".to_vec(),
///     "password",
///     "salt"
/// )?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn encrypt(data: Vec<u8>, pwd: &str, salt: &str) -> Result<Vec<u8>> {
    // ...
}
```

**Files to document:**

```bash
# Find public functions without docs
rg "^pub fn" --type rust crates/rucksack-lib/src/ crates/rucksack-db/src/ \
  | while read -r line; do
      file=$(echo "$line" | cut -d: -f1)
      linenum=$(echo "$line" | cut -d: -f2)
      prev_line=$((linenum - 1))
      if ! sed -n "${prev_line}p" "$file" | grep -q "///"; then
        echo "$line"
      fi
    done
```

**Sections to include:**

- Summary (one sentence)
- Arguments (if any)
- Return value description
- Errors section (for Result returns)
- Panics section (if it can panic)
- Examples (where helpful)

---

#### Task 22: Document All Public Structs/Enums

**Pattern:**

```rust
/// Represents an encrypted database with all metadata.
///
/// This structure wraps encrypted data along with encryption
/// parameters and provides methods for decryption and access.
///
/// # Examples
///
/// ```
/// use rucksack_db::EncryptedDB;
///
/// let db = EncryptedDB::new(/* ... */);
/// let decrypted = db.decrypt()?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct EncryptedDB {
    // ...
}

/// Database record status.
///
/// Indicates whether a record is active, archived, or deleted.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    /// Record is active and in use
    Active,

    /// Record is archived but not deleted
    Archived,

    /// Record is marked for deletion
    Deleted,
}
```

---

#### Task 23: Add Module-Level Documentation

**Files:** All mod.rs and lib.rs files

**Pattern:**

```rust
//! Database encryption and storage.
//!
//! This module provides encrypted storage for password records with
//! support for multiple storage backends (filesystem, embedded database).
//!
//! # Architecture
//!
//! The database is encrypted using AES-256-GCM with keys derived from
//! user passwords using a salt. Data is serialized using bincode and
//! supports versioning for schema evolution.
//!
//! # Examples
//!
//! ```
//! use rucksack_db::DB;
//!
//! let mut db = DB::new("secrets.db", "backups", "password", "salt")?;
//! db.insert("key", record)?;
//! # Ok::<(), anyhow::Error>(())
//! ```

use anyhow::Result;
// ...
```

---

#### Task 24: Validate Documentation

**Commands:**

```bash
# Build docs without warnings
cargo doc --all --no-deps

# Check for missing docs
cargo rustdoc -p rucksack-db -- -D missing_docs
cargo rustdoc -p rucksack-lib -- -D missing_docs

# Build and open docs
cargo doc --all --no-deps --open

# Check examples compile
cargo test --doc
```

---

### Phase 4 Completion Checklist

- [ ] Task 21 complete (function docs)
- [ ] Task 22 complete (struct/enum docs)
- [ ] Task 23 complete (module docs)
- [ ] Task 24 validation passes
- [ ] cargo doc builds without warnings
- [ ] Doc examples compile and run
- [ ] Documentation is clear and helpful

---

### Phase 5: Additional Enhancements

**Priority:** 🟢 LOW for 0.11.0
**Breaking Changes:** None
**Estimated Effort:** 2-4 hours

This phase includes non-audit items and polish.

---

#### Task 25: Implement Missing Traits

**Goal:** Add helpful trait implementations

**Pattern:**

```rust
// Add Hash to hashable types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordKey(String);

// Add From/Into for conversions
impl From<v080::Record> for v090::Record {
    fn from(old: v080::Record) -> Self {
        // Migration logic
    }
}

// Add Display for user-facing types
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Active => write!(f, "active"),
            Status::Archived => write!(f, "archived"),
            Status::Deleted => write!(f, "deleted"),
        }
    }
}
```

---

#### Task 26: Add Constants for Magic Values

**Goal:** Replace magic numbers with named constants

**Audit:**

```bash
rg "0x[0-9a-fA-F]+" --type rust crates/
rg "\b[0-9]{3,}\b" --type rust crates/ | grep -v test
```

**Action:**

```rust
/// Maximum timeout in seconds (24 hours)
const MAX_TIMEOUT_SECS: u64 = 60 * 60 * 24;

/// AES-GCM nonce size in bytes
const NONCE_SIZE: usize = 12;

/// Default buffer size for file operations
const DEFAULT_BUFFER_SIZE: usize = 4096;
```

---

#### Task 27: Improve Error Messages

**Goal:** Make error messages more user-friendly

**Pattern:**

```rust
// Before
Err(anyhow!("failed"))

// After
Err(anyhow!(
    "failed to decrypt database - password may be incorrect or database is corrupted"
))

// With context
std::fs::read(path)
    .with_context(|| format!(
        "failed to read database file '{}' - check that the file exists and you have read permission",
        path.display()
    ))?
```

---

### Phase 5 Completion Checklist

- [ ] Task 25 complete (trait implementations)
- [ ] Task 26 complete (magic value constants)
- [ ] Task 27 complete (improved error messages)
- [ ] All tests pass
- [ ] User experience improved

---

### Release 0.11.0 - Final Validation

**Complete Checklist:**

- [ ] Phase 3 complete (performance optimizations)
- [ ] Phase 4 complete (documentation)
- [ ] Phase 5 complete (additional enhancements)
- [ ] CHANGELOG.md updated
- [ ] All tests pass: `cargo test --all`
- [ ] Coverage maintained ≥95%
- [ ] Clippy clean: `make lint`
- [ ] Format clean: `make format`
- [ ] Documentation complete and builds clean
- [ ] Manual CLI testing passes
- [ ] Version bumped to 0.11.0 in all Cargo.toml files
- [ ] Git commit: "Release 0.11.0: Performance improvements, complete documentation, and enhancements"

---

## Audit Completion

### Final Validation

**Comprehensive Checklist:**

- [ ] All 27 tasks completed
- [ ] Zero unwrap() in library code (except documented)
- [ ] Zero panic() for recoverable errors
- [ ] PathBuf used for all file paths
- [ ] &str used for read-only string parameters
- [ ] All secrets use SecretString
- [ ] All public items documented
- [ ] All tests pass (100% success rate)
- [ ] Coverage ≥95%
- [ ] Clippy clean with strict lints
- [ ] All three releases tagged

### Success Metrics Achieved

✅ Code quality: Modern Rust idioms throughout
✅ API quality: Clean, ergonomic, properly typed
✅ Documentation: Comprehensive and helpful
✅ Test coverage: ≥95% maintained
✅ Performance: Optimized, minimal allocations
✅ Security: Secrets properly protected

### Post-Audit Maintenance

**Going Forward:**

1. Run clippy in CI with strict lints
2. Maintain documentation standards
3. Add new tests for new features
4. Review PRs against anti-patterns
5. Update CLAUDE.md with lessons learned

---

**Plan prepared by:** Claude Code (Sonnet 4.5)
**Date:** 2026-01-20
**Audit Reference:** Based on comprehensive codebase exploration and Rust best practices
