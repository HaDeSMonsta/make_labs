mod md;

use crate::md::{write_lite, write_students};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::clone::Clone;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

const CSV_NAME: &'static str = "original.csv";
const MINS_PER_TUT: usize = 90;
const FIRST_TUT_FNAME: &'static str = "first.md";
const SECOND_TUT_FNAME: &'static str = "second.md";
const FIRST_TUT_FNAME_PUB: &'static str = "first_pub.md";
const SECOND_TUT_FNAME_PUB: &'static str = "second_pub.md";

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
    pub tutor: String,
    pub tutorium: String,
    pub slot: Option<String>,
    pub matr_num: String,
    pub email: String,
}

fn assign_times(students: &mut Vec<Student>, start_hour: usize, start_min: usize) {
    students.shuffle(&mut thread_rng());
    let mins_per_student = MINS_PER_TUT / students.len();

    let mut hour = start_hour;
    let mut min = start_min;

    for student in students {
        student.slot = Some(format!("{hour:0>2}:{min:0>2} Uhr"));
        min += mins_per_student;
        hour += min / 60;
        min = min % 60;
    }
}

fn main() {
    let csv = OpenOptions::new()
        .read(true)
        .open(CSV_NAME)
        .expect(&format!("Unable to open {CSV_NAME}"));

    let reader = BufReader::new(csv);

    let mut first_tut = Vec::new();
    let mut second_tut = Vec::new();

    reader.lines()
                  .skip(1)
                  .map(|l| {
                      l.expect("Invalid line")
                  })
                  .map(|l| {
                      l.split(",")
                       .map(|s| {
                           String::from(s)
                       })
                       .collect::<Vec<_>>()
                  })
                  .map(|vec| {
                      Student {
                          first_name: String::from(vec[1].trim()),
                          last_name: String::from(vec[0].trim()),
                          tutor: String::from(vec[10].trim()),
                          tutorium: String::from(vec[7].trim()),
                          slot: None,
                          matr_num: String::from(vec[2].trim()),
                          email: String::from(vec[4].trim()),
                      }
                  })
                  .filter(|s| {
                      s.tutor == "Leon"
                  })
                  .for_each(|s| {
                      match s.tutorium.as_str() {
                          "Online Zoom 06" => first_tut.push(s),
                          "Online Zoom 07" => second_tut.push(s),
                          tut => panic!("Invalid tut: {tut}")
                      }
                  });

    assign_times(&mut first_tut, 8, 15);
    assign_times(&mut second_tut, 10, 15);

    write_lite(first_tut.clone(), FIRST_TUT_FNAME_PUB);
    write_lite(second_tut.clone(), SECOND_TUT_FNAME_PUB);

    write_students(first_tut, FIRST_TUT_FNAME);
    write_students(second_tut, SECOND_TUT_FNAME);
}
