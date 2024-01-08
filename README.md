# target_feature repro

```sh
cargo build --release
cargo asm target_feature_repro::aes_encrypt
```

```sh
RUSTFLAGS="-C target-feature=+aes" cargo build --release
cargo asm target_feature_repro::aes_encrypt
```