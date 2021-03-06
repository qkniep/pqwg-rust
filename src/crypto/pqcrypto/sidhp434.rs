// Copyright (c) 2019 Quentin Kniep. All rights reserved.
// SPDX-License-Identifier: BSD-3-Clause

pub const PQ_SECRET_KEY_SIZE: usize = 28;
pub const PQ_PUBLIC_KEY_SIZE: usize = 330;
pub const PQ_CIPHERTEXT_SIZE: usize = 330;
pub const PQ_SHARED_KEY_SIZE: usize = 110;

#[link(name = "oqs")]
extern {
    #[link_name = "OQS_KEM_sidh_p434_keypair"]
    pub fn pq_keypair(public_key: *mut u8,
                      secret_key: *mut u8) -> u32;
    #[link_name = "OQS_KEM_sidh_p434_encaps"]
    pub fn pq_encaps(ciphertext: *mut u8,
                     shared_secret: *mut u8,
                     public_key: *const u8) -> u32;
    #[link_name = "OQS_KEM_sidh_p434_decaps"]
    pub fn pq_decaps(shared_secret: *mut u8,
                     ciphertext: *const u8,
                     secret_key: *const u8) -> u32;
}
