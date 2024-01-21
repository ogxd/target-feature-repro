use core::arch::x86_64::*;

use std::hint::black_box;

fn main() {
    unsafe {
        do_work();
    }
}

#[inline(never)]
//#[target_feature(enable = "aes")]
unsafe fn do_work() {
    let key =  _mm_loadu_si128(black_box(KEY.as_ptr() as *const __m128i));
    let data = _mm_set1_epi64x(black_box(42i64));
    
    let value = aes_encrypt_wrapper(data, key);
    dbg!(value);
}

pub const KEY: [u32; 4] = [0xF2784542, 0xB09D3E21, 0x89C222E5, 0xFC3BC28E];

#[inline]
#[target_feature(enable = "aes")]
pub unsafe fn aes_encrypt(data: __m128i, keys: __m128i) -> __m128i {
    _mm_aesenc_si128(data, keys)
}

#[inline]
#[target_feature(enable = "aes")]
pub unsafe fn aes_encrypt_wrapper(data: __m128i, keys: __m128i) -> __m128i {
    black_box(aes_encrypt(data, keys))
}