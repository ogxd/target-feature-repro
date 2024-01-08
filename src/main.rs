use core::arch::aarch64::*;
use std::time::SystemTime;

fn main() {
    let epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    unsafe {
        let data = vreinterpretq_s8_u64(vdupq_n_u64(epoch));
        let key = vreinterpretq_s8_u32(vld1q_u32(KEY.as_ptr()));
        let encrypted = aes_encrypt(data, key);
        println!("Output: {:?}", encrypted);
    }
}

pub const KEY: [u32; 4] = [0xF2784542, 0xB09D3E21, 0x89C222E5, 0xFC3BC28E];

#[inline(never)]
#[target_feature(enable = "aes")]
pub unsafe fn aes_encrypt(data: int8x16_t, keys: int8x16_t) -> int8x16_t {
    // Encrypt
    let encrypted = vaeseq_u8(vreinterpretq_u8_s8(data), vdupq_n_u8(0));
    // Mix columns
    let mixed = vaesmcq_u8(encrypted);
    // Xor keys
    vreinterpretq_s8_u8(veorq_u8(mixed, vreinterpretq_u8_s8(keys)))
}