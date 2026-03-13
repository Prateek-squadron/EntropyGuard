# 🔐 EntropyGuard

**EntropyGuard** is a Rust-based CLI tool that analyzes password strength using **entropy calculations, pattern detection, and brute-force crack-time estimation**.

Instead of only checking password length or character types, EntropyGuard models **how an attacker would attempt to crack a password**, giving users a more realistic view of password security.

---

## 🚀 Features

✅ **Password Length Analysis**
Detects the total length of the password.

✅ **Character Type Detection**

* Uppercase letters
* Numbers
* Special characters

✅ **Entropy Calculation**
Estimates password randomness using entropy mathematics.

✅ **Pattern Detection**
Flags predictable sequences like:

* `123`
* `abcd`
* `qwerty`

✅ **Crack-Time Estimation**
Simulates brute-force attacks and estimates how long it would take to crack the password.

✅ **Risk Level Classification**
Categorizes passwords as:

* 🔴 HIGH RISK
* 🟡 MODERATE
* 🟢 STRONG

---

## 📦 Installation

Clone the repository:

```bash
git clone https://github.com/Prateek-squadron/EntropyGuard.git
cd EntropyGuard
```

Build and run:

```bash
cargo run
```

Make sure you have **Rust installed**:

```bash
rustc --version
```

If not, install Rust from:

👉 https://rustup.rs

---

## 🖥️ Example Usage

```text
==============================
EntropyGuard v1.0
==============================

Enter password:
> P@ssword123

Length              : 11
Uppercase present   : true
Numbers present     : true
Special chars       : true

Entropy             : 72.10 bits
Weak pattern        : true

Estimated crack time: 2 hours
Risk level          : HIGH RISK
```

---

## 🧠 How It Works

EntropyGuard evaluates passwords using multiple metrics:

### 🔢 Entropy Calculation

Entropy measures the **randomness of a password**.

Higher entropy means the password is **harder to guess**.

Entropy is estimated using:

```
Entropy = Length × log₂(Character Set Size)
```

---

### 🔎 Pattern Detection

Humans often create predictable passwords.

EntropyGuard detects patterns such as:

```
123456
abcdef
qwerty
password123
```

These patterns significantly reduce real password security.

---

### ⏳ Crack Time Estimation

The tool estimates how long an attacker would take to brute-force the password based on:

```
Total Combinations = Charset Size ^ Password Length
```

Then it simulates attack speed using high-speed guessing hardware.

---

## 📂 Project Structure

```
EntropyGuard/
│
├── src/
│   ├── main.rs
│   └── password.rs
│
├── Cargo.toml
└── README.md
```

---

## 🛠️ Built With

* 🦀 **Rust**
* Rust standard library
* CLI input/output

---

## 📌 Version

Current release:

```
EntropyGuard v1.0
```

---

## 🔮 Future Improvements (v2.0)

Planned features:

* 📚 Dictionary attack detection
* ⌨️ Keyboard walk detection (`qwerty`, `asdf`)
* 📄 External wordlist support
* 🎨 Colored CLI output
* ⚙️ Command-line arguments with `clap`
* 📊 Batch password analysis

---

## 👨‍💻 Author

**Prateek**
Cybersecurity & Digital Forensics Student
VIT Bhopal

---

## 📜 License

This project is licensed under the **MIT License**.

---

## ⭐ Support

If you find this project useful, consider giving the repository a **star ⭐ on GitHub**.
