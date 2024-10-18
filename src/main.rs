mod md;
mod consts;

use crate::md::{write_lite, write_students};
use crate::consts::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::clone::Clone;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;

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

fn assign_times(students: &mut Vec<Student>, start_hour: usize, start_min: usize, duration: usize) {
    students.shuffle(&mut thread_rng());
    let mins_per_student = duration / students.len();

    let mut hour = start_hour;
    let mut min = start_min;

    for student in students {
        student.slot = Some(format!("{hour:0>2}:{min:0>2} Uhr"));
        min += mins_per_student;
        hour += min / 60;
        min = min % 60;
    }
}

fn check_env() {
    dotenv::dotenv().expect("Unable to find .env file");

    assert_eq!(
        TUT_NAMES.len(),
        TUT_START_HORS.len(),
        "Each tut must have a name and start hour",
    );

    assert_eq!(
        TUT_NAMES.len(),
        TUT_START_MINS.len(),
        "Each tut must have a name and start min",
    );

    assert_eq!(
        TUT_NAMES.len(),
        TUT_DURATION_MINS.len(),
        "Each tut must have a name and a duration",
    );
}

fn main() {
    check_env();

    let csv = OpenOptions::new()
        .read(true)
        .open(&*CSV_NAME)
        .expect(&format!("Unable to open {}", *CSV_NAME));

    let reader = BufReader::new(csv);

    let mut tuts = HashMap::new();

    for name in &*TUT_NAMES {
        tuts.insert(name.clone(), Vec::new());
    }

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
              s.tutor == *TUTOR_NAME
          })
          .for_each(|s| {
              match tuts.get_mut(&s.tutorium) {
                  Some(vec) => vec.push(s),
                  None => panic!("Invalid tut: {}", s.tutorium),
              }
          });

    for (name, mut students) in tuts {
        let mut idx = 100;
        for i in 0..TUT_NAMES.len() {
            if TUT_NAMES[i] == name {
                idx = i;
                break;
            }
        }

        assert_ne!(idx, 100);

        assign_times(
            &mut students,
            TUT_START_HORS[idx],
            TUT_START_MINS[idx],
            TUT_DURATION_MINS[idx],
        );

        write_lite(students.clone(), &format!("public_{}.md", idx + 1));

        write_students(students, &format!("internal_{}.md", idx + 1));
    }
}
