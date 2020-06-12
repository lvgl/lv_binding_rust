mod arc;
mod bar;
mod gauge;
mod label;

include!("generated.rs");

use crate::Widget;
pub use arc::*;
pub use bar::*;
pub use gauge::*;
pub use label::*;
