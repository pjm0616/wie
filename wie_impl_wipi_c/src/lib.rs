#![no_std]
#![allow(unknown_lints)]
#![allow(clippy::needless_pass_by_ref_mut)]
extern crate alloc;

mod base;
mod error;
pub mod r#impl;
mod method;

pub use self::base::{WIPICContext, WIPICError, WIPICMemoryId, WIPICMethodBody, WIPICResult, WIPICWord};
