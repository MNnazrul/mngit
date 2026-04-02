# mngit

A Git implementation from scratch in Rust — for learning how Git works internally.

---

## Progress

### Phase 0: Repository Setup

- [x] **`init`** — Create `.mngit/`, `.mngit/objects/`, `.mngit/refs/heads/`, `.mngit/HEAD`

### Phase 1: Object Database

- [x] **`hash-object [-w] <file>`** — File → blob object → SHA-1 → store in `.mngit/objects/`
- [x] **`cat-file -p <oid>`** — Read object → decompress → print content

### Phase 2: Tree (Directory Snapshot)

- [x] **`write-tree`** — Working directory → tree object
- [x] **`ls-tree <tree_oid>`** — Read tree object and list files/directories

### Phase 3: Commits & References

- [x] **`commit-tree <tree_oid> -m "message"`** — Tree → commit object
- [ ] **`update-ref` / `update-head`** — Update `refs/heads/main` or `HEAD`
- [ ] **`rev-parse <ref>`** — Resolve `HEAD` or branch name to an OID
- [ ] **`log`** — Traverse and display commit history

### Phase 4: Checkout

- [ ] **`checkout <commit | branch>`** — Commit → tree → recreate working directory

### Phase 5: Index / Staging Area

- [ ] **`add <path>`** — File → staging area (`.mngit/index`)
- [ ] **`status`** — Compare working directory vs index vs `HEAD`
- [ ] **`commit`** — Index → tree → commit

### Phase 6: Branching & Diff

- [ ] **`branch`** — Create and list branches
- [ ] **`diff`** — Compare changes (file/tree/commit)
- [ ] **`merge`** — Merge branches

### Phase 7: Advanced

- [ ] **`tag`**
- [ ] **`pack-objects / unpack-objects`**
- [ ] **`clone / fetch / push`**

---

## Project Structure

```
src/
  main.rs          # All commands implemented here (single-file for now)

tests/
  common/mod.rs    # Shared test helpers
  init_test.rs
  hash_object_test.rs
  cat_file_test.rs
  write_tree_test.rs
  ls_tree_test.rs
  roundtrip_test.rs
  error_test.rs
```

---

## Usage

```bash
# Initialize a repository
cargo run -- init

# Hash a file (print OID only)
cargo run -- hash-object <file>

# Hash a file and write to object store
cargo run -- -w hash-object <file>

# Print object content
cargo run -- cat-file -p <oid>

# Snapshot working directory as a tree object
cargo run -- write-tree

# List entries of a tree object
cargo run -- ls-tree <tree_oid>
```

## Running Tests

```bash
cargo test                      # run all tests
cargo test --test ls_tree_test  # run a specific test file
```
