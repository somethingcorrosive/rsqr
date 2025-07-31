# 🦀 rsqr — Really Simple QR

**rsqr** is a dead-simple, fast QR code generator written in Rust.

It generates **terminal-friendly ASCII QR codes**, and can optionally export them as **high-resolution PNG images**.  
Designed to be tiny, testable, and do exactly one thing — very well.

---

## ✨ Features

- ✅ Generate terminal QR codes from any string
- ✅ Export to PNG with custom scale
- ✅ Toggle quiet zone padding (`--no-quiet`)
- ✅ Invert light/dark rendering (`--invert`)
- ✅ Use different character styles (`--charset`)
- ✅ Built-in test suite and QA shell script
- ✅ Shell completions for bash/zsh/fish ( make completions )

---

## 📦 Install

Clone the repo and build:

    git clone https://github.com/somethingcorrosive/rsqr.git
    cd rsqr
    cargo build --release

Then copy the binary to your path:

    cp target/release/rsqr /usr/local/bin/

---

## 🚀 Usage

### ➤ Basic terminal QR code

    rsqr "https://rust-lang.org"

### ➤ Inverted output

    rsqr "cool contrast" --invert

### ➤ No quiet zone (padding)

    rsqr "tight fit" --no-quiet

### ➤ Custom character sets

    rsqr "hello hash" --charset hash
    rsqr "dot style"  --charset dot

> ⚠️ Some charsets may not scan well. `rsqr` will warn you if you're using a non-standard one.

---

### ➤ Export PNG

    rsqr "https://example.com" --png qr.png

### ➤ PNG with custom scale

    rsqr "scaled out" --png big.png --scale 20

### ➤ Full feature combo

    rsqr "full test" --invert --no-quiet --charset block --png final.png --scale 16

---

## 🧪 Testing & QA

Run all unit + integration tests:

    cargo test

Run full CLI QA script:

    make qa

Run one step manually( make sure to cargo build prior ):

    ./qa.sh

---

## 🐚 Shell Completions

    make completions

Generated completions are saved to:

    completions/

---

## 🧼 Clean Build

    make clean

---

## 🪪 License

MIT License  
© 2025 [RandomName](https://github.com/somethingcorrosive)

---

## 🙌 Contributing

This project is intentionally scoped small.  
Bug reports, tests, and useful flags are welcome. Pull requests encouraged!

