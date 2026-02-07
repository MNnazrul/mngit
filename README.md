## 🧱 Phase 0: Repository Setup

* [ ] **`init`**
  Create `.git/`, `.git/objects/`, `.git/refs/heads/`, `.git/HEAD`

---

## 🧬 Phase 1: Object Database (Git Core)

* [ ] **`hash-object -w <file>`**
  File → blob object → SHA-1 → store in `.git/objects/`

* [ ] **`cat-file -p <oid>`**
  Read object → decompress → print content

---

## 🌳 Phase 2: Tree (Directory Snapshot)

* [ ] **`write-tree`**
  Working directory → tree object

* [ ] **`ls-tree <tree_oid>`**
  Read tree object and list files/directories

---

## 🧾 Phase 3: Commits & References (History)

* [ ] **`commit-tree <tree_oid> -m "message"`**
  Tree → commit object

* [ ] **`update-ref` / `update-head`**
  Update `refs/heads/main` or `HEAD`

* [ ] **`rev-parse <ref>`**
  Resolve `HEAD` or branch name to an object id

* [ ] **`log`**
  Traverse and display commit history

---

## 🔄 Phase 4: Checkout (Restore Files)

* [ ] **`checkout <commit | branch>`**
  Commit → tree → recreate working directory

---

## 📦 Phase 5: Index / Staging Area

* [ ] **`add <path>`**
  File → staging area (`.git/index`)

* [ ] **`status`**
  Compare working directory vs index vs `HEAD`

* [ ] **`commit`**
  Index → tree → commit

---

## 🌿 Phase 6: Branching & Diff

* [ ] **`branch`**
  Create and list branches

* [ ] **`diff`**
  Compare changes (file/tree/commit)

* [ ] **`merge`**
  Merge branches

---

## 🌐 Phase 7: Advanced Git (Later)

* [ ] **`tag`**
* [ ] **`pack-objects / unpack-objects`**
* [ ] **`clone / fetch / push`**

---

```
src/
  main.rs
  cli/
    mod.rs
    init.rs
    hash_object.rs
    cat_file.rs
  app/
    mod.rs
    init_repo.rs
    hash_object.rs
    read_object.rs
    commit.rs
  domain/
    mod.rs
    oid.rs
    object/
      mod.rs
      blob.rs
      tree.rs
      commit.rs
    refs.rs
  infra/
    mod.rs
    repo_layout.rs
    object_store_fs.rs
    ref_store_fs.rs
    index_store_fs.rs
```
---
