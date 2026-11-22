//! ELEMENTS: molecules put to use. An element has a job in the app: it is wired
//! to data and actions, and it owns its states (ideal / empty / loading / error).
//!
//! Most elements are app-specific and live in the app (`src/ui/elements/`), e.g.
//! `TaskActions` = a `ButtonGroup` with "Mark done / Edit / Delete" for the selected task.
//! The ones below are needed by every app, so they live here.
//! See STRUCTURE.md §4.

// quag:parts (one file per part; `quag add` copies them one by one)
mod app_header;
pub use app_header::*;
mod status_bar;
pub use status_bar::*;
mod empty_state;
pub use empty_state::*;
mod confirm_dialog;
pub use confirm_dialog::*;
mod error_state;
pub use error_state::*;
mod toast;
pub use toast::*;
mod command_palette;
pub use command_palette::*;
mod help_overlay;
pub use help_overlay::*;
mod log_viewer;
pub use log_viewer::*;
mod type_to_confirm;
pub use type_to_confirm::*;
mod diff_view;
pub use diff_view::*;
mod run_progress;
pub use run_progress::*;

#[cfg(test)]
mod tests;
