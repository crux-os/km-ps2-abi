//! km-ps2 ABI — Rust side, mirrors `abi/zig/ps2.zig`.
//!
//! Userspace drivers (um-keyboard-ps2, um-mouse-ps2) call
//! `SYS_INVOKE(SLOT_ID, OP_READ, device)` to drain the kernel's
//! per-device PS/2 ring.  The kernel itself never interprets these
//! constants — they are a contract between km-ps2 and its consumers.

#![allow(missing_docs)]
#![cfg_attr(not(test), no_std)]

pub const SLOT_ID: u64 = 0x2020_5350_324F_5053;     // 'PS2OPS  '

pub const OP_READ: u32 = 0;

pub const DEVICE_KEYBOARD: u8 = 0;
pub const DEVICE_MOUSE:    u8 = 1;

/// Driver return values are system statuses from the error registry
/// (zigbone_abi::errors); this driver only returns generic errors.
pub use zigbone_abi::errors::{Error, Status};
