# screen-capture

A Rust-based application demonstrating the adversarial tactic of **screen capturing**.
This **Proof of Concept (PoC)** is intended for **security research** and **red team
exercises**.

## ⚠️ Disclaimer

This tool is intended for **educational and security research purposes only**.

**Unauthorized use of this software to capture screens without consent may violate local laws**

The author assumes **no liability** for misuse.

---

## 🛠️ Installation & Setup

### Prerequisites

- **Rust** (Install via [Rustup](https://rustup.rs/))
- **Windows OS** (Uses Win32 API for screen capture)

### Building the Project

Compile the crate using:

```shell
cargo build --release
```

## 🚀 Usage

### Run as a DLL

Capture one screenshot per second and save it to `c:\temp`:


```shell
cargo build --release
rundll32.exe .\target\release\screen_capture_lib.dll, ScreenCapture c:\temp
```

### Run as an EXE

Capture a single screenshot and save it to `c:\temp\screenshot.bmp`:

```shell
cargo run --release -- --file c:\temp\screenshot.bmp
```

Capture one screenshot per second and save it to `c:\temp`:

```shell
cargo run --release -- --dir c:\temp
```

## 📜 License

GNU General Public License v3.0 (GPLv3)

See [LICENSE](./LICENSE) for details.


## 🏴‍☠️ Ethical Considerations

This project is designed strictly for security research.

Do not use it without explicit authorization.
