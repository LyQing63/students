use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub grades: HashMap<String, f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grade {
    pub subject_name: String,
    pub score: f32,
}

impl Student {
    pub fn new(id: u32, subject_name: &str, grades: HashMap<String, f32>) -> Student {
        Self { id, name: String::from(subject_name), grades, }
    }

    pub fn add_grade(&mut self, grade: Grade) {
        self.grades.insert(grade.subject_name, grade.score);
    }
}