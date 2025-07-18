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
- [x] Expression validity checks (e.g., unbalanced parentheses, invalid tokens).
- [x] Support for multi-digit numbers (currently parses only single-digit).
- [x] Support for floating point numbers.
- [ ] New Expression Language implementation
- [ ] Support for variables
- [ ] Support for strictly numerical vector datatypes