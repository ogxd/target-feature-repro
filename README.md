# target_feature repro

```sh
cargo build --release
cargo asm target_feature_repro::aes_encrypt | grep "aesenc"
```