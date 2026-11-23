# Contributing to D0

Welcome to the **D0 Programming Language** project!  
D0 is designed by **Dinesh** and inspired by **Surya** to be a modern, independent language for Systems, Security, Automation, and AI.

We welcome contributions from developers worldwide!

## How Outside Developers Can Contribute

### 1. Prerequisites
- [Rust toolchain](https://rustup.rs/) (edition 2021, version 1.70+)
- Git
- VS Code (optional, but recommended)

### 2. Fork & Clone
```bash
git clone https://github.com/Dinesh0729/d0.git
cd d0
```

### 3. Build & Test Locally
Before submitting any changes, make sure all tests pass:
```bash
# Check compilation
cargo check

# Run automated test suite
cargo test

# Build optimized release binary
cargo build --release
```

### 4. Development Workflow
1. Create a descriptive feature branch:
   ```bash
   git checkout -b feature/my-new-feature
   ```
2. Write clean, idiomatic Rust with automated tests.
3. Test your changes against existing `.d0` example programs:
   ```bash
   cargo run -- check examples/valid_program.d0
   cargo run -- parse examples/functions.d0
   ```
4. Commit your work with clear messages:
   ```bash
   git commit -m "Add new feature to D0 compiler"
   ```
5. Push to your fork and submit a Pull Request (PR) to `main` on [https://github.com/Dinesh0729/d0](https://github.com/Dinesh0729/d0).

### 5. Code of Conduct
Please ensure all discussions, code reviews, and issue tracker interactions are constructive, respectful, and welcoming to all developers.
