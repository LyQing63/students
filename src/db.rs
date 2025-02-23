use std::fs::File;
use std::io;
use crate::model::Student;

pub fn save_to_file(students: &Vec<Student>) -> io::Result<()> {
    let file = File::create("students.json")?;
    serde_json::to_writer(file, students)?;
    Ok(())
}

pub fn load_from_file() -> io::Result<Vec<Student>> {
    let file = File::open("students.json")?;
    let students = serde_json::from_reader(file)?;
    Ok(students)
}