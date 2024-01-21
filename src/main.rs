use core::arch::x86_64::*;

use std::time::Instant;
use std::hint::black_box;

fn main() {
    let start = Instant::now();

    unsafe {
        do_work();
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

#[target_feature(enable = "sse2,aes")]
unsafe fn do_work() {
    let key = _mm_set1_epi64x(black_box(42i64));
    let mut data = _mm_set1_epi64x(black_box(42i64));
    
    for _ in 0..10_000_000u64 {
        data = black_box(aes_encrypt(data, key));
    }
}

pub const KEY: [u32; 4] = [0xF2784542, 0xB09D3E21, 0x89C222E5, 0xFC3BC28E];

#[inline(never)]
#[target_feature(enable = "sse2,aes")]
pub unsafe fn aes_encrypt(data: __m128i, keys: __m128i) -> __m128i {
    _mm_aesenc_si128(data, keys)
}

// RESULTS
// 1) aes_encrypt w/ target_feature ❌
// 2) do_work w/ target_feature ✅
// 3) rustflags w/ target-feature ✅
// 4) rustflags + flags everywhere BUT no inlining: perf as shit as 1)
// Given these, we have the proof that target_feature disables inlining, which totally sucks ass