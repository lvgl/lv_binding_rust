#![feature(try_trait)]
#![no_std]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate bitflags;

mod display;
mod global;
mod support;
#[macro_use]
mod lv_core;
pub mod widgets;

pub use display::DisplayDriver;
pub use global::UI;
pub use lv_core::*;
pub use support::*;
