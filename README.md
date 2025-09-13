# AES Hardware Acceleration Tests for Mac

Compile using the flag

```bash
RUSTFLAGS="--cfg aes_armv8" cargo build -r 
```

```bash
rust-objdump -d target/release/aes_hardware | grep -Ei 'aes(enc|dec|imc|keygenassist)|vaes'
```

The dump contains aes instructions as seen below.

```bash
100000d2c: 4e287821     aesimc.16b      v1, v1
100000d30: 4e287842     aesimc.16b      v2, v2
100000d34: 4e287863     aesimc.16b      v3, v3
100000d38: 4e287884     aesimc.16b      v4, v4
100000d3c: 4e2878a5     aesimc.16b      v5, v5
100000d40: 4e2878c6     aesimc.16b      v6, v6
100000d44: 4e2878e7     aesimc.16b      v7, v7
100000d48: 4e287a10     aesimc.16b      v16, v16
100000d4c: 4e287a31     aesimc.16b      v17, v17
```

However is compiled without the flag

```bash
cargo build -r 
```

and then the object file is dumped.

```bash
rust-objdump -d target/release/aes_hardware | grep -Ei 'aes(enc|dec|imc|keygenassist)|vaes'
```

There are no aes instructions present in the dymo file.
