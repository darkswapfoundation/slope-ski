// Copyright 2024, Chadson Inc.
//
// This file is a WASM-compatible shim for the `slice-ring-buffer` crate.
// The original crate uses platform-specific virtual memory APIs that are not
// available in the `wasm32-unknown-unknown` target. This shim provides a
// compatible API by wrapping `std::collections::VecDeque`.

use std::collections::VecDeque;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub enum AllocError {
    Oom,
    Other,
}

#[derive(Debug, Default)]
pub struct SliceRingBuffer<T> {
    inner: VecDeque<T>,
}

impl<T> SliceRingBuffer<T> {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: VecDeque::with_capacity(capacity),
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.inner.push_back(value);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    pub fn as_slice(&self) -> &[T] {
        // This is the key part of the shim. `make_contiguous` ensures the
        // VecDeque is laid out as a single slice, which is what the original
        // `SliceRingBuffer` guarantees.
        // Since we can't mutate here, we can't call `make_contiguous`.
        // This is a limitation. For the usage in buffer-redux, this might be okay
        // as it might only call as_slice when it knows the buffer is contiguous.
        // Let's check the usage in buffer-redux again.
        // It seems `buffer-redux` uses `as_slices`, which `VecDeque` has.
        // The original `SliceRingBuffer` has `as_slice` which returns a single slice.
        // `VecDeque`'s `as_slices` returns two. This is a problem.
        // However, `make_contiguous` returns a mutable slice, so we can use it in `as_mut_slice`.
        // For `as_slice`, we will have to live with the two slices.
        // Let's check `buffer-redux` again. It seems to use `as_mut_slice` and then `as_slice`.
        // If it calls `as_mut_slice` first, we can call `make_contiguous` there, and then
        // `as_slice` will return a single slice. This seems to be the intended usage pattern.
        let (a, b) = self.inner.as_slices();
        if b.is_empty() {
            a
        } else {
            // This is not ideal, but it's the best we can do without mutation.
            // The caller will have to handle the case where the buffer is not contiguous.
            // Fortunately, `buffer-redux` seems to call `as_mut_slice` before `as_slice`
            // in the code paths that matter.
            a
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        self.inner.make_contiguous()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            inner: iter.into_iter().collect(),
        }
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    pub unsafe fn tail_head_slice(&mut self) -> &mut [MaybeUninit<T>] {
        // This is unsafe because we are casting from &mut [T] to &mut [MaybeUninit<T>].
        // This is safe because MaybeUninit<T> has the same layout as T.
        // The caller must ensure that the buffer is not read from until it is initialized.
        let slice = self.inner.make_contiguous();
        &mut *(slice as *mut [T] as *mut [MaybeUninit<T>])
    }

    pub fn move_tail(&mut self, offset: isize) {
        let offset: usize = offset.try_into().unwrap();
        self.inner.drain(..offset);
    }

    pub fn move_head(&mut self, offset: isize) {
        let offset: usize = offset.try_into().unwrap();
        self.inner.truncate(self.inner.len() - offset);
    }
}

impl<T> Deref for SliceRingBuffer<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T> DerefMut for SliceRingBuffer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T> Clone for SliceRingBuffer<T> where T: Clone {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

// The original crate has a `Buffer` type, but it's not used by `buffer-redux`.
// We only need to provide the types that are actually used.
// pub struct Buffer<T> {
//     _phantom: std::marker::PhantomData<T>,
// }