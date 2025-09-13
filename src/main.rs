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

fn check_if_cpu_supports_aes() -> bool {
    #[cfg(target_arch = "aarch64")]
    {
        std::arch::is_aarch64_feature_detected!("aes")
    }
    #[cfg(target_arch = "x86_64")]
    {
        std::arch::is_x86_feature_detected!("aes")
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        false
    }
}   

fn check_if_aes_is_enabled() -> bool {
    #[cfg(aes_force_soft)]
    {
        false  // Explicitly disabled by aes_force_soft
    }
    #[cfg(all(not(aes_force_soft), any(aes_armv8, target_feature = "aes")))]
    {
        true
    }
    #[cfg(all(not(aes_force_soft), not(any(aes_armv8, target_feature = "aes"))))]
    {
        false
    }
}

fn main() {
    println!("CPU reports AES feature: {}", check_if_cpu_supports_aes());
    println!("AES HW acceleration enabled in build: {}", check_if_aes_is_enabled());

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
    let backend = {
        #[cfg(aes_force_soft)]
        {
            "Software AES (aes_force_soft enabled)"
        }
        #[cfg(all(not(aes_force_soft), aes_armv8))]
        {
            "ARMv8 HW AES (aes_armv8)"
        }
        #[cfg(all(not(aes_force_soft), target_feature = "aes", target_arch = "x86_64", not(aes_armv8)))]
        {
            "x86_64 AES-NI HW AES (target_feature=aes)"
        }
        #[cfg(all(not(aes_force_soft), not(any(aes_armv8, all(target_feature = "aes", target_arch = "x86_64")))))]
        {
            "Software AES (no hardware acceleration)"
        }
    };

    println!("Encrypted {:.2} MiB in {:.3} s -> {:.1} MiB/s [{}]", mb, secs, throughput, backend);

    // Optional: runtime CPU feature probe
    #[cfg(target_arch = "aarch64")]
    {
        println!("CPU reports AES feature: {}", std::arch::is_aarch64_feature_detected!("aes"));
    }
    #[cfg(target_arch = "x86_64")]
    {
        println!("CPU reports AES-NI feature: {}", std::arch::is_x86_feature_detected!("aes"));
    }
}