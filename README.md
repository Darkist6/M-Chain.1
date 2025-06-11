# M-Chain

> A simple Apple Silicon-locked Proof-of-Work blockchain demo written in Rust

---

## 🔧 Features

- ⛏️ Block mining with adjustable difficulty
- 🧠 Auto-resumes from saved blocks (`.json`)
- 🔐 SHA256-based hash integrity
- 💾 Filesystem storage (in `mchain_data/`)
- 🧱 CLI subcommands to mine, verify, list, and reset
- 🔒 Only runs on Apple Silicon (M1/M2/M3/M4...)

---

## 🚀 Getting Started

```bash
git clone https://github.com/Darkist6/M-Chain.1.git
cd M-Chain.1
cargo build
```

---

## 🔨 Usage

### ⛏️ Mine blocks
```bash
cargo run -- mine --blocks 3 --difficulty 5 --data "Testing MChain"
```

### 🔍 Verify block integrity
```bash
cargo run -- verify
```

### 📋 List existing blocks
```bash
cargo run -- list
```

### 🗑️ Delete all stored blocks
```bash
cargo run -- reset
```

---

## 📂 Block Storage

Blocks are stored in JSON format under the `mchain_data/` directory.
Each file is named:
```
block_<index>.json
```

---

## 🔐 Platform Restriction
This demo only works on **Apple Silicon Macs** and will exit with an error on Intel or non-Apple machines.

---

## 🧭 Roadmap
- [x] Phase 1–7 (local persistence + CLI)
- [ ] Phase 8: Peer-to-peer networking
- [ ] Phase 9: Simulated wallet + mining rewards
- [ ] Phase 10: Cloud syncing + dashboard

---

## 📄 License
MIT
