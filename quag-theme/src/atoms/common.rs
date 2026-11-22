//! ATOM: shared by the atoms of this folder.


/// Interaction state shared by all interactive atoms.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Normal,
    Focused,
    Disabled,
}
