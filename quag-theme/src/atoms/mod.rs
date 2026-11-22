//! ATOMS: the smallest parts. One control or one visual, with its own look and states.
//! An atom knows nothing about the app: `Button`, never `SaveButton`.
//! See STRUCTURE.md §2.

// quag:parts (one file per part; `quag add` copies them one by one)
mod common;
pub use common::*;
mod button;
pub use button::*;
mod checkbox;
pub use checkbox::*;
mod input;
pub use input::*;
mod text;
pub use text::*;
mod progress;
pub use progress::*;
mod containers;
pub use containers::*;
mod radio;
pub use radio::*;
mod badge;
pub use badge::*;
mod skeleton;
pub use skeleton::*;
mod link;
pub use link::*;
mod number_field;
pub use number_field::*;
mod slider;
pub use slider::*;
mod select;
pub use select::*;
mod charts;
pub use charts::*;
mod split_handle;
pub use split_handle::*;
mod severity;
pub use severity::*;
mod secret_field;
pub use secret_field::*;
mod servers;
pub use servers::*;
mod game_design;
pub use game_design::*;

#[cfg(test)]
mod tests;
