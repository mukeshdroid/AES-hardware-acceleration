#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== AES Hardware Acceleration Test ===${NC}"

# Detect architecture
ARCH=$(uname -m)
echo -e "${YELLOW}Detected architecture: ${ARCH}${NC}"

# Function to run objdump and check for AES instructions
check_aes_instructions() {
    local build_type="$1"
    echo -e "\n${BLUE}Checking for AES instructions in ${build_type} build:${NC}"

    if [[ "$ARCH" == "arm64" ]] || [[ "$ARCH" == "aarch64" ]]; then
        # ARM architecture - look for ARMv8 AES instructions
        local aes_count=$(rust-objdump -d target/release/aes_hardware | grep -Ei 'aes(enc|dec|imc|keygenassist)|vaes' | wc -l)
        rust-objdump -d target/release/aes_hardware | grep -Ei 'aes(enc|dec|imc|keygenassist)|vaes' | head -5
    else
        # x86_64 architecture - look for AES-NI instructions
        local aes_count=$(rust-objdump -d target/release/aes_hardware | grep -Ei 'aes(enc|enclast|dec|declast|keygenassist)|vaes' | wc -l)
        rust-objdump -d target/release/aes_hardware | grep -Ei 'aes(enc|enclast|dec|declast|keygenassist)|vaes' | head -5
    fi

    if [[ $aes_count -gt 0 ]]; then
        echo -e "${GREEN}✓ Found ${aes_count} AES hardware instructions${NC}"
        return 0
    else
        echo -e "${RED}✗ No AES hardware instructions found${NC}"
        return 1
    fi
}

# Function to run the program and show output
run_program() {
    local build_type="$1"
    echo -e "\n${BLUE}Running ${build_type} build:${NC}"
    ./target/release/aes_hardware
}

echo -e "\n${YELLOW}=== Testing WITHOUT hardware acceleration ===${NC}"

echo -e "${BLUE}Building without AES acceleration...${NC}"
if [[ "$ARCH" == "arm64" ]] || [[ "$ARCH" == "aarch64" ]]; then
    cargo build -r 2>/dev/null
else
    # For x86, explicitly disable AES to force software implementation
    RUSTFLAGS="-C target-feature=-aes" cargo build -r 2>/dev/null
fi

run_program "software-only"
check_aes_instructions "software-only" || true

echo -e "\n${YELLOW}=== Testing WITH hardware acceleration ===${NC}"

if [[ "$ARCH" == "arm64" ]] || [[ "$ARCH" == "aarch64" ]]; then
    echo -e "${BLUE}Building with ARMv8 AES acceleration...${NC}"
    RUSTFLAGS="--cfg aes_armv8" cargo build -r 2>/dev/null

    run_program "ARMv8 hardware-accelerated"
    if check_aes_instructions "ARMv8 hardware-accelerated"; then
        echo -e "${GREEN}✓ ARMv8 AES hardware acceleration is working!${NC}"
    else
        echo -e "${RED}✗ ARMv8 AES hardware acceleration failed${NC}"
    fi

else
    echo -e "${BLUE}Building with x86 AES-NI acceleration (target-cpu=native)...${NC}"
    RUSTFLAGS="-C target-cpu=native" cargo build -r 2>/dev/null

    run_program "x86 AES-NI hardware-accelerated (native)"
    if check_aes_instructions "x86 AES-NI hardware-accelerated (native)"; then
        echo -e "${GREEN}✓ x86 AES-NI hardware acceleration is working!${NC}"
    else
        echo -e "${YELLOW}Trying with explicit AES feature flag...${NC}"
        RUSTFLAGS="-C target-feature=+aes" cargo build -r 2>/dev/null

        run_program "x86 AES-NI hardware-accelerated (explicit)"
        if check_aes_instructions "x86 AES-NI hardware-accelerated (explicit)"; then
            echo -e "${GREEN}✓ x86 AES-NI hardware acceleration is working!${NC}"
        else
            echo -e "${RED}✗ x86 AES-NI hardware acceleration failed - CPU may not support AES-NI${NC}"
        fi
    fi
fi

echo -e "Architecture: ${ARCH}"