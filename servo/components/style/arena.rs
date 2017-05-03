/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Arena that never frees.

use std::mem;

/// Marker for types that should be arena-allocated.
pub trait ArenaAllocated {}

const CHUNK_SIZE: usize = 1 << 20;

/// Arena class.
pub struct Arena {
    chunks: Vec<Vec<u8>>,
}

impl Arena {
    /// Creates an arena.
    pub fn new() -> Self {
        Arena {
            chunks: Vec::new(),
        }
    }

    /// Allocates. Chunk will never be freed.
    pub fn allocate(&mut self, size: usize) -> *mut u8 {
        if let Some(mut last) = self.chunks.last_mut() {
            if last.len() + size <= last.capacity() {
                return Self::allocate_from(&mut last, size);
            }
        }

        assert!(size < CHUNK_SIZE);
        let mut v = Vec::with_capacity(CHUNK_SIZE);
        let allocated = Self::allocate_from(&mut v, size);
        self.chunks.push(v);
        allocated
    }

    fn allocate_from(v: &mut Vec<u8>, size: usize) -> *mut u8 {
        let new_len = v.len() + size;
        debug_assert!(new_len <= v.capacity());
        unsafe {
            let allocated = v.as_mut_ptr().offset(v.len() as isize);
            v.set_len(new_len);
            allocated
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        for mut v in self.chunks.drain(..) {
            v.shrink_to_fit();
            mem::forget(v);
        }
    }
}
