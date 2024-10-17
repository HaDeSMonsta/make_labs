use std::cmp::max;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use crate::Student;

pub fn write_lite(students: Vec<Student>, file_name: &str) {
    let header = vec![
        "Last Name",
        "First Name",
        "Time",
    ].into_iter()
     .map(|s| String::from(s))
     .collect::<Vec<_>>();
    
    let mut sep = Vec::new();

    for _ in 0..header.len() {
        sep.push(String::from("-"))
    }

    let mut students = students.into_iter()
                               .map(|s| {
                                   vec![
                                       s.last_name,
                                       s.first_name,
                                       s.slot.unwrap(),
                                   ]
                               })
                               .collect();

    let mut md = vec![header, sep];
    md.append(&mut students);
    write(&md, file_name);
}

pub fn write_students(students: Vec<Student>, file_name: &str) {
    let mut header = vec![
        "Last Name",
        "First Name",
        "Matr. Num",
        "Email",
        "Time",
    ].into_iter()
        .map(|s| String::from(s))
        .collect::<Vec<_>>();
    
    for i in 1..=14 {
        header.push(format!("{i}"));
    }
    
    let mut sep = Vec::new();
    
    for _ in 0..header.len() {
        sep.push(String::from("-"))
    }
    
    let mut students = students.into_iter()
        .map(|s| {
            vec![
                s.last_name,
                s.first_name,
                s.matr_num,
                s.email,
                s.slot.unwrap(),
            ]
        })
        .collect();
    
    let mut md = vec![header, sep];
    md.append(&mut students);
    write(&md, file_name);
}

fn write(md: &Vec<Vec<String>>, file_name: &str) {
    
    let mut max_len = 0;
    md.iter().for_each(|v| max_len = max(max_len, v.len()));
    
    let file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(file_name)
        .expect(&format!("Unable to open {file_name}"));
    
    let mut writer = BufWriter::new(file);
    
    for line in md {
        for i in 0..max_len {
            let cell = match line.get(i) {
                Some(s) => String::from(s),
                None => String::from(" "),
            };
            write!(writer, "|{cell}").unwrap();
        }
        writeln!(writer, "|").unwrap();
    }
}