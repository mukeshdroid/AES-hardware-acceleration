use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};
use std::time::Instant;

fn encrypt_in_place(cipher: &Aes128, data: &mut [u8]) {
    for chunk in data.chunks_exact_mut(16) {
        // View the 16-byte chunk as an AES block and encrypt it
        let block = GenericArray::from_mut_slice(chunk);
        cipher.encrypt_block(block);
    }
}

fn main() {
    let size_bytes: usize = 64 * 1024 * 1024;
    let mut buf = vec![0u8; size_bytes];

    // Fixed key for repeatability (don’t do this in real code)
    let key = GenericArray::from([0u8; 16]);
    let cipher = Aes128::new(&key);

    // Warm up CPU/caches/JIT paths
    encrypt_in_place(&cipher, &mut buf);

    // Time it
    let start = Instant::now();
    encrypt_in_place(&cipher, &mut buf);
    let elapsed = start.elapsed();

    let mb = (size_bytes as f64) / (1024.0 * 1024.0);
    let secs = elapsed.as_secs_f64();
    let throughput = mb / secs;

    // Helpful hint about which backend you intended to use
    #[cfg(aes_armv8)]
    let backend = "ARMv8 HW AES (aes_armv8)";
    #[cfg(not(aes_armv8))]
    let backend = "Software AES (no aes_armv8)";

    println!("Encrypted {:.2} MiB in {:.3} s -> {:.1} MiB/s [{}]", mb, secs, throughput, backend);

    // Optional: runtime CPU feature probe on aarch64
    #[cfg(target_arch = "aarch64")]
    {
        println!("CPU reports AES feature: {}", std::arch::is_aarch64_feature_detected!("aes"));
    }
}