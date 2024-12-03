rustc --crate-type=cdylib -C opt-level=3 -C panic=abort -C lto=fat -o cord. src/lib.rs
