use std::vec::Vec;
use crate::models::structs::Submodule;
use crate::search::perform_search;
use tui::style::Color;

pub struct App {
    pub submodules: Vec<Submodule>,
    pub filtered_submodules: Vec<Submodule>,
    pub last_query: Option<String>,

    pub current_query: String,
    
    pub skin_color: Color,
}

impl Default for App {
    fn default() -> Self {
        App {
            filtered_submodules: vec![],
            submodules: vec![],

            current_query: String::new(),
            last_query: None,
            skin_color: Color::Rgb(244, 71, 2)
        }
    }
}

impl App {
    pub fn new(submodules: Vec<Submodule>, first_query: &str) -> Self {
        App {
            submodules,
            current_query: first_query.to_string(),
            ..App::default()
        }
    }

    // Performs search on submodules and updates app's internal
    // cached search. Only performs the search if it's needed.
    pub fn search(&mut self) {
        // TODO: Process query to extract tags and name searches
        if self.last_query.is_none() || self.current_query != self.last_query.as_deref().unwrap() {
            let mut final_query = vec![];
            let mut tags = vec![];

            for token in self.current_query.split(' ') {
                if let Some(tag_name) = token.strip_prefix("tag:") {
                    tags.push(tag_name);
                } else {
                    final_query.push(token);
                }
            }


            self.filtered_submodules = perform_search(&self.submodules, &final_query.join(" "), &tags).expect("can search through submodules"); 

            self.last_query = Some(self.current_query.clone());
        }
    }
}