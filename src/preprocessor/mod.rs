pub mod gist;
pub mod image;

pub use gist::preprocess_gists;
pub use image::preprocess_images;
pub use joaju_table as table;
pub use joaju_table::{preprocess_tables, render_table_block as render_single_table};

/// Run all markdown preprocessors sequentially (images, gists, tables).
pub fn preprocess_markdown(content: &str) -> String {
    let s = preprocess_images(content);
    let s = preprocess_gists(&s);
    preprocess_tables(&s)
}
