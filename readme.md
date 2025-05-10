# 🧮 Expression Evaluator in Rust <img src="https://www.rustacean.net/assets/rustacean-flat-happy.svg" width="40" alt="Ferris the Crab">

A simple Rust-based expression evaluator that parses and computes mathematical expressions using basic arithmetic operations.

---
## ✨ Features

- Basic arithmetic: `+`, `-`, `*`, `/`
- expression parsing using expression tree
- Simple and readable Rust implementation
---
---

## 🚀 Getting Started

### 🛠 Dev Build
```bash
cargo build 
```
### 🛠 Release Build
```bash
cargo build --release
```
### 🛠 Dev Run
```bash
cargo run "1+2"
```
```bash
cargo run "(4 - 1) * 5"
```

### 🛠 Dev Test
```bash
cargo test
```

## Further Development
- [*] Expression validity checks (e.g., unbalanced parentheses, invalid tokens).
- [ ] Support for multi-digit numbers (currently parses only single-digit).
- [ ] Support for negative integers.