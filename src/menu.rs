use crate::db::save_to_file;
use crate::model::Student;
use std::collections::HashMap;
use std::io;
use std::io::Write;

pub fn display_menu(students: &mut Vec<Student>) {

    let mut choice = String::new();

    loop {
        choice.clear();
        println!("1. 添加学生");
        println!("2. 查看学生成绩");
        println!("3. 保存数据");
        println!("4. 退出");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut choice)
            .expect("读入选择时出错");

        match choice.trim().parse::<i32>() {
            Ok(1) => add_student(students),
            Ok(2) => println!("{:?}", students),
            Ok(3) => save_to_file(students).expect("保存失败"),
            Ok(4) => break,
            _ => break,
        }
    }

}

pub fn add_student(students: &mut Vec<Student>) {

    let mut choice = String::new();
    let mut id:u32 = 0;
    let mut name = String::new();
    let mut grades: HashMap<String, f32> = HashMap::new();

    loop {
        choice.clear();
        println!("1. 输入学生id");
        println!("2. 输入学生姓名");
        println!("3. 输入学生成绩");
        println!("4. 退出");

        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut choice)
            .expect("读入选择时出错");
        let student: Student;

        match choice.trim().parse::<i32>() {
            Ok(1) => {
                let mut tmp = String::new();
                println!("请输入id: ");
                io::stdin()
                    .read_line(&mut tmp)
                    .expect("读入id时出错");
                id = tmp.trim().parse().expect("id输入格式错误!");
            },
            Ok(2) => {
                println!("请输入name: ");
                io::stdin()
                    .read_line(&mut name)
                    .expect("读入name时出错");
            },
            Ok(3) => add_grade(&mut grades),
            _ => {
                student = Student::new(id, &name.trim(), grades);
                students.push(student);
                break;
            },
        }
    }

}

pub fn add_grade(grades: &mut HashMap<String, f32>) {

    let mut choice = String::new();
    let mut name = String::new();
    let mut score: f32 = 0.0;

    loop {
        choice.clear();
        println!("1. 输入科目名称");
        println!("2. 输入科目分数");
        println!("3. 退出");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .expect("读入选择时出错");

        match choice.trim().parse::<i32>() {
            Ok(1) => {
                println!("请输入科目名称: ");

                io::stdin()
                    .read_line(&mut name)
                    .expect("读入name时出错");
            },
            Ok(2) => {
                let mut tmp = String::new();
                println!("请输入分数: ");
                io::stdin()
                    .read_line(&mut tmp)
                    .expect("读入分数时出错");
                score = tmp.trim().parse().expect("分数输入格式错误!");
            },
            _ => {
                grades.insert(String::from(&name.trim()), score);
                break;
            },
        }
    }
}