/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Vec of vecs, each one with twice the capacity of the last. This gives
//! us the general performance characteristics of a vec while avoiding
//! append slowdowns with a large number of elements.

#![deny(missing_docs)]

use smallvec::SmallVec;
use std::iter::FlatMap;
use std::slice::Iter;

/// to balance cache locality while reading against
/// growing overhead while appending. Unmeasured.
const FIRST_CHUNK_SIZE: usize = 8;

/// ChunkVec struct. This wraps a smallvec of vecs. We use a smallvec with
/// one element of inline storage so that we don't need an extra heap
/// allocation for small numbers of elements.
#[derive(Debug)]
pub struct ChunkVec<T>(SmallVec<[Vec<T>; 1]>);

impl<T> Default for ChunkVec<T> {
    fn default() -> Self {
        ChunkVec(SmallVec::new())
    }
}

impl<T> ChunkVec<T> {
    /// Appends an element to the ChunkVec.
    pub fn push(&mut self, value: T) {
        if let Some(last) = self.0.last_mut() {
            if last.len() != last.capacity() {
                debug_assert!(last.len() < last.capacity());
                last.push(value);
                return;
            }
        }

        let capacity = self.0.last().map_or(FIRST_CHUNK_SIZE,
                                            |last| last.capacity() * 2);
        let mut v = Vec::with_capacity(capacity);
        v.push(value);
        self.0.push(v)
    }

    /// Emptys the ChunkVec.
    pub fn clear(&mut self) {
        self.0.clear()
    }

    /// Returns whether the ChunkVec has no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of elements in the ChunkVec.
    pub fn len(&self) -> usize {
        if self.0.is_empty() {
            return 0;
        }
        let len = self.0.len();
        if len == 1 {
            return self.0[0].len()
        }

        let cap_of_largest_full_buffer = FIRST_CHUNK_SIZE << (len - 2);
        let cap_of_full_buffers = cap_of_largest_full_buffer - FIRST_CHUNK_SIZE;
        cap_of_full_buffers + self.0.last().unwrap().len()
    }

    /// Returns an iterator over the ChunkVec.
    pub fn iter(&self) -> FlatMap<Iter<Vec<T>>, Iter<T>, fn(&Vec<T>) -> Iter<T>> {
        // We need a proper function here, rather than a closure, because
        // closures have unique types that we can't specify in the return
        // type above.
        fn iter_vec<T>(v: &Vec<T>) -> Iter<T> { v.iter() }
        self.0.iter().flat_map(iter_vec)
    }
}

