mod html;
mod json;
mod terminal;

pub use html::write_html_report;
pub use json::{render_json_report, write_json_report};
pub use terminal::print_terminal_report;
