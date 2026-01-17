mod diff_view;
mod file_list;
mod status_bar;

pub use diff_view::render_diff_view;
pub use file_list::render_file_list;
pub use status_bar::{render_header, render_status_bar};
