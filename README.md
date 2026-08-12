<div align="center">
  <h1>Distributed Event Streaming Platform</h1>
  <p><b>High-throughput, fault-tolerant message broker implemented in Rust.</b></p>
  
  <p>
    <img src="https://img.shields.io/badge/Rust-000000?style=flat-square&logo=rust&logoColor=white" alt="Rust" />
    <img src="https://img.shields.io/badge/Tokio-000000?style=flat-square" alt="Tokio" />
  </p>
</div>

---

## 📖 Overview

A bespoke distributed event streaming platform designed for ultra-low latency and high-throughput data pipelines. Inspired by Apache Kafka and Redpanda, this broker is implemented entirely in Rust utilizing asynchronous I/O via the Tokio runtime.

## ✨ Architecture & Consensus

```mermaid
graph LR
    P[Producer] -->|TCP stream| B1[Broker Leader]
    B1 -->|Replication| B2[Broker Follower 1]
    B1 -->|Replication| B3[Broker Follower 2]
    C[Consumer Group] -->|Fetch| B1
    
    subgraph Cluster
    B1
    B2
    B3
    end
```

## 🚀 Features

- **Asynchronous Core:** Built on `tokio` for handling tens of thousands of concurrent connections.
- **Append-Only Log:** High-performance disk persistence strategy mimicking LSM trees.
- **Memory Efficiency:** Zero-copy serialization strategies utilizing standard Rust ownership models.

## 🛠️ Usage

```bash
cargo build --release
cargo run --release
```
