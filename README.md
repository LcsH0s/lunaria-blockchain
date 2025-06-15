
## 🛠 Milestone 1: Basic Block & Chain Structure

**Goal:** Learn Rust’s data modeling, hashing, and basic I/O

* **Define your `Block` struct**
  * Fields: `index: u64`, `timestamp: u128`, `data: Vec<u8>`, `previous_hash: [u8;32]`, `hash: [u8;32]`

* **Implement block hashing**
  * Use `sha2` crate to SHA‑256 the concatenated fields

* **Build the `Chain` type**
  * A `Vec<Block>` that always starts with a hard‑coded “genesis” block

* **CLI to append blocks**
  * Read “data” from STDIN, mine the hash, append to chain, and write chain to disk (e.g. JSON or binary)

* **Tests**
  * Verify hash links, JSON (de)serialization, and genesis consistency

> **Rough effort:** \~1 week (Rust novice) to 2 days (experienced Rustacean)

---

## 🛡 Milestone 2: Simple Proof‑of‑Work (PoW)

**Goal:** Introduce consensus basics via mining

* **Add a `nonce: u64` field to `Block`**
* **Difficulty parameter**

  * e.g. find hash with first *n* zero bits
* **“Mining” function**

  * Loop over nonce until hash meets difficulty
* **Benchmark & tune**

  * Log average time per block; adjust difficulty dynamically or manually
* **CLI flag**

  * Allow user to choose difficulty

> **Rough effort:** 3–5 days

---

## 🌐 Milestone 3: Peer‑to‑Peer Networking

**Goal:** Distribute your chain across multiple nodes

* **Pick a networking stack**

  * Use `tokio` + `tokio‑serde` + `bincode` (or `serde_cbor`)
* **Basic TCP listener & dialer**

  * Each node maintains a list of peer addresses
* **Gossip protocol**

  * On new block, broadcast to peers; peers validate and append if valid
* **Simple RPC**

  * “Get latest chain” and “Submit block” endpoints

> **Rough effort:** 1–2 weeks

---

## 💸 Milestone 4: Transactions & UTXO (or Account) Model

**Goal:** Move from “raw data” blocks to meaningful value transfers

* **Design transaction struct**

  * For UTXO: `inputs: Vec<UTXORef>`, `outputs: Vec<(PubKey, u64)>`, `signature`
  * For account: `(from, to, amount, signature)`
* **Maintain state**

  * UTXO set or account balances in-memory + persistent snapshot
* **Transaction validation**

  * Signatures (`ed25519-dalek`), double‑spend checks, balance checks
* **Mempool**

  * Simple queue of pending transactions

> **Rough effort:** 1–2 weeks

---

## 🌲 Milestone 5: Merkle Trees & Block Validation

**Goal:** Efficiently prove inclusion of transactions

* **Integrate a Merkle tree crate** (e.g. `merkle_light`)
* **Store Merkle root in your `Block` header**
* **Build tree over transaction list**
* **Validation RPC**

  * “Prove transaction X is in block N”

> **Rough effort:** 4–7 days

---

## ⏱ Milestone 6: Proof‑of‑History (PoH) PoC

**Goal:** Prototype Solana’s innovation at a toy scale

* **Implement a sequential SHA‑256 “tick”**

  * Each tick: `state = SHA256(state)`
  * Record tick count + timestamp in block header
* **Use PoH to timestamp transactions**
* **Chain ordering**

  * Accept only blocks whose PoH tick matches expected height

> **Rough effort:** 1 week (prototype)

---

## ⚖️ Milestone 7: Lightweight Consensus (Optional)

**Goal:** Introduce validators and leadership rotation

* **Leader election**

  * Round‑robin over a fixed validator set
* **Vote messages**

  * Broadcast “I accept block X” signed by validator
* **Finality gadget**

  * Once 2/3 of validators vote, block is “final”

> **Rough effort:** 1–2 weeks

---

## 🧰 Milestone 8: CLI Wallet & Explorer

**Goal:** User‑facing tools for interacting with your chain

* **`wallet` binary**

  * Keypair generation, balance query, send transaction
* **`explorer` binary**

  * Show chain status: latest slot, TPS estimate, peer list
  * Pretty‑print blocks and transactions

> **Rough effort:** 1 week

---

## 🚀 Milestone 9: Performance & Asynchronous Optimization

**Goal:** Push TPS and low‑latency—get closer to a “real” blockchain

* **Tokio optimizations**

  * Batch networking I/O with `Framed` + backpressure
* **Zero‑copy serialization**

  * Use `bytes::Bytes` and `rkyv` for zero‑copy
* **Profiling**

  * Instrument with `tokio-console`, `perf`, or `pprof`
* **CI/CD**

  * Automated tests, benchmarks, and lints via GitHub Actions

> **Rough effort:** ongoing

---

### Tips & Resources

* **Crates**

  * Crypto: `sha2`, `ed25519-dalek`, `merkle-light`
  * Async: `tokio`, `bytes`, `rkyv`
  * Serialization: `serde`, `bincode`, `serde_cbor`
* **Learning**

  * *Mastering Rust* by Vesa Kaihlavirta
  * *Building Blockchain in Rust* tutorials online
  * Solana whitepapers (for PoH deep dive)
* **Testing**

  * Property‑based tests with `proptest`
  * Fuzzing transaction parsing

## 🔄 Transaction Lifecycle

1. Client signs and sends a `Transaction` to the cluster
2. Leader receives it and includes it in a PoH block (slot)
3. Validators replay the block and **execute each instruction** in order
4. State changes are committed
5. Rewards and fees are calculated post-block
