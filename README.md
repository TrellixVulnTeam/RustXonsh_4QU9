# RustXonsh

The goal of RustXonsh is to rewrite [Xonsh](https://github.com/xonsh/xonsh) in Rust, based on code from [RustPython](https://github.com/RustPython/RustPython).

## Features

- [x] Support for Python 3.8 syntax (from RustPython)
- [x] [Path string literals](https://xon.sh/tutorial.html#advanced-string-literals) (e.g. `p"/etc/passwd"`)
- [ ] Executing subprocess commands using `$[]`
- [ ] Capturing subprocess command output using `$()`
- [ ] Executing subprocess commands using regular shell syntax (e.g. `echo hello`)

## Usage

RustXonsh requires the latest stable version of Rust (since it uses RustPython).

To build RustXonsh and run the demo program:
```
git clone https://github.com/yaxollum/RustXonsh.git
cd RustXonsh
cargo run --release demo.xsh
```
