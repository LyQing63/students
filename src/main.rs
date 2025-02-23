use crate::db::load_from_file;
use crate::menu::display_menu;

mod model;
mod db;
mod utils;
mod menu;

fn main() {
    println!("欢迎使用学生成绩系统!");
    let mut students = load_from_file().expect("读取数据出错");
    // 页面绘制
    display_menu(&mut students);
}
