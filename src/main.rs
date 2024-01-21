#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use std::time::Instant;
use std::hint::black_box;

fn main() {
    let start = Instant::now();

    unsafe {
        fun_name();
    }

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}

#[target_feature(enable = "aes")]
unsafe fn fun_name() {
    let key = get_key();
    let data = get_data();
    
    for _ in 0..10_000_000u64 {
        black_box(aes_encrypt(black_box(data), black_box(key)));
    }
}

#[inline(always)]
#[cfg(target_arch = "aarch64")]
unsafe fn get_key(i: u64) {
    let data = vreinterpretq_s8_u64(vdupq_n_u64(i));
    vreinterpretq_s8_u32(vld1q_u32(KEY.as_ptr()))
}

#[inline]
//#[target_feature(enable = "sse2")]
#[cfg(target_arch = "x86_64")]
unsafe fn get_key() -> __m128i {
    _mm_loadu_si128(KEY.as_ptr() as *const __m128i)
}

#[inline(always)]
#[cfg(target_arch = "aarch64")]
unsafe fn get_data() {
    vreinterpretq_s8_u64(vdupq_n_u64(42u64))
}

#[inline]
//#[target_feature(enable = "sse2")]
#[cfg(target_arch = "x86_64")]
unsafe fn get_data() -> __m128i {
    _mm_set1_epi64x(42i64)
}

pub const KEY: [u32; 4] = [0xF2784542, 0xB09D3E21, 0x89C222E5, 0xFC3BC28E];

#[inline]
//#[target_feature(enable = "aes")]
#[cfg(target_arch = "aarch64")]
pub unsafe fn aes_encrypt(data: int8x16_t, keys: int8x16_t) -> int8x16_t {
    // Encrypt
    let encrypted = vaeseq_u8(vreinterpretq_u8_s8(data), vdupq_n_u8(0));
    // Mix columns
    let mixed = vaesmcq_u8(encrypted);
    // Xor keys
    vreinterpretq_s8_u8(veorq_u8(mixed, vreinterpretq_u8_s8(keys)))
}

#[inline]
//#[target_feature(enable = "aes")]
#[cfg(target_arch = "x86_64")]
pub unsafe fn aes_encrypt(data: __m128i, keys: __m128i) -> __m128i {
    _mm_aesenc_si128(data, keys)
}