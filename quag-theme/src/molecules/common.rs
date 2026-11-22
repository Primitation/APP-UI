//! MOLECULE: shared by the molecules of this folder.


/// Gap between atoms inside one molecule (STRUCTURE.md: "inside a group").
pub use crate::consts::layout::GAP;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    #[default]
    Left,
    Right,
}
