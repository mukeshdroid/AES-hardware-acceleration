# AES Hardware Acceleration Tests

## Quick Start

Run the automated test script to detect your architecture and test AES acceleration:

```bash
./test_aes.sh
```

This script will:
- Detect your CPU architecture (ARM64 or x86_64)
- Test both software-only and hardware-accelerated builds
- Show performance comparison and verify AES instructions are used
- Use appropriate build flags for your platform

## Manual Testing

This tool tests AES hardware acceleration on both ARM and x86 processors.

## ARM (Apple Silicon / ARMv8)

Compile using the flag:

```bash
RUSTFLAGS="--cfg aes_armv8" cargo build -r
```

Check for ARMv8 AES instructions:

```bash
rust-objdump -d target/release/aes_hardware | grep -Ei 'aes(enc|dec|imc|keygenassist)|vaes'
```

## x86_64 (Intel/AMD with AES-NI)

Compile using CPU-native optimizations (recommended):

```bash
RUSTFLAGS="-C target-cpu=native" cargo build -r
```

Or enable AES-NI specifically:

```bash
RUSTFLAGS="-C target-feature=+aes" cargo build -r
```

Check for x86 AES-NI instructions:

```bash
rust-objdump -d target/release/aes_hardware | grep -Ei 'aes(enc|enclast|dec|declast|keygenassist)|vaes'
```

## Example Output

### ARM (with hardware acceleration)

The dump contains ARMv8 AES instructions:

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

### x86_64 (with AES-NI acceleration)

The dump should contain x86 AES-NI instructions like:

```bash
aesenc   %xmm1, %xmm0
aesenclast %xmm2, %xmm0
aesdec   %xmm3, %xmm0
aesdeclast %xmm4, %xmm0
aeskeygenassist $0x1, %xmm0, %xmm1
```

### Without hardware acceleration

When compiled without the appropriate flags:

```bash
cargo build -r
```

The objdump will show no AES hardware instructions, indicating software-only implementation:

```bash
rust-objdump -d target/release/aes_hardware | grep -Ei 'aes(enc|dec|imc|keygenassist)|vaes'
# No output - using software AES implementation
```
