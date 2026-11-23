# Getting Started with D0 (for Outside Developers)

Welcome to **D0** (pronounced *"D-Zero"*), an independent programming language engineered for **Systems Programming**, **Defensive Cybersecurity**, **Automation**, and **AI**.

---

## 1. Installation

### Quick Install (macOS / Linux / Windows)

#### Step 1: Install Rust (if not already installed)
- **macOS / Linux**:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  source "$HOME/.cargo/env"
  ```
- **Windows**: Download and run [rustup-init.exe](https://rustup.rs).

#### Step 2: Clone the D0 Repository
```bash
git clone https://github.com/Dinesh0729/d0.git
cd d0
```

#### Step 3: Build the D0 Compiler Frontend (`d0c`)
```bash
cargo build --release
```
The compiled binary will be located at `target/release/d0c`.

To make `d0c` accessible from anywhere on your terminal:
```bash
# macOS / Linux
sudo cp target/release/d0c /usr/local/bin/
```

---

## 2. Writing Your First D0 Program

Create a file named `hello.d0`:

```d0
module MyFirstApp {
    fn main() -> Void {
        print("Hello, World from D0!");
        print("Independent Programming Language by Dinesh");
    }
}
```

---

## 3. Using the D0 Compiler (`d0c`)

### 1. Check Syntax and Types (Semantic Analysis)
```bash
d0c check hello.d0
```
*Output:* `hello.d0: semantic analysis passed`

### 2. Inspect the Abstract Syntax Tree (AST)
```bash
d0c parse hello.d0
```

### 3. Tokenize Source Code (Lexer)
```bash
d0c lex hello.d0
```

---

## 4. Install the D0 VS Code Extension

For syntax highlighting, code comments, and bracket matching in Visual Studio Code:

```bash
# Copy extension to your VS Code directory
cp -r editor/vscode ~/.vscode/extensions/d0-language
```
Restart VS Code and open any `.d0` file!
