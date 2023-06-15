use std::collections::HashMap;

pub mod history;

pub struct App {
    pub items: Vec<String>,
    pub content: HashMap<usize, String>,
    pub selected_index: usize,
    pub title: String,
}
