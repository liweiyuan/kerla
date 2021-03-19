#![cfg_attr(feature = "no_std", no_std)]
#![allow(unused)]

#[cfg(not(feature = "no_std"))]
#[macro_use]
extern crate std;

pub mod alignment;
pub mod buddy_allocator;
pub mod bump_allocator;
pub mod byte_size;
pub mod lazy;
pub mod once;