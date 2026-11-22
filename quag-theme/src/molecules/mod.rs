//! MOLECULES: a few atoms arranged together, usually in a box.
//! Still generic: a molecule has no idea what app it's in. It takes values and
//! labels as props; it doesn't fetch data or decide what buttons do.
//! See STRUCTURE.md §3.

// quag:parts (one file per part; `quag add` copies them one by one)
mod common;
pub use common::*;
mod button_group;
pub use button_group::*;
mod field;
pub use field::*;
mod search_bar;
pub use search_bar::*;
mod tabs;
pub use tabs::*;
mod key_hints;
pub use key_hints::*;
mod status_line;
pub use status_line::*;
mod list_row;
pub use list_row::*;
mod stat;
pub use stat::*;
mod check_list;
pub use check_list::*;
mod log_line;
pub use log_line::*;
mod helpers;
pub use helpers::*;
mod radio_group;
pub use radio_group::*;
mod breadcrumbs;
pub use breadcrumbs::*;
mod menu;
pub use menu::*;
mod table;
pub use table::*;
mod property_list;
pub use property_list::*;
mod tree_view;
pub use tree_view::*;
mod param_row;
pub use param_row::*;
mod bar_chart;
pub use bar_chart::*;
mod segmented;
pub use segmented::*;
mod servers;
pub use servers::*;
mod game_design;
pub use game_design::*;
mod node_graph;
pub use node_graph::*;

#[cfg(test)]
mod tests;
