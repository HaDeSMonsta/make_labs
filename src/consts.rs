use std::cell::LazyCell;
use std::env;

pub const TUTOR_NAME: LazyCell<String> = LazyCell::new(|| {
    env::var("NAME").expect("Tutor name is not set")
});
pub const CSV_NAME: LazyCell<String> = LazyCell::new(|| {
    env::var("SOURCE_FILE").unwrap_or(String::from("original.csv"))
});

pub const TUT_NAMES: LazyCell<Vec<String>> = LazyCell::new(|| {
    let mut names = Vec::new();

    for i in 1..10 {
        let key = format!("TUT_{i}_NAME");
        match env::var(key) {
            Ok(name) => names.push(name),
            Err(_) => break,
        }
    }

    names
});

pub const TUT_START_HORS: LazyCell<Vec<usize>> = LazyCell::new(|| {
    let mut hours = Vec::new();

    for i in 1..10 {
        let key = format!("TUT_{i}_START_HOUR");
        match env::var(key) {
            Ok(hour) => hours.push(
                hour
                    .parse()
                    .expect(&format!("{hour} is not a valid usize"))
            ),
            Err(_) => break,
        }
    }

    hours
});

pub const TUT_START_MINS: LazyCell<Vec<usize>> = LazyCell::new(|| {
    let mut mins = Vec::new();

    for i in 1..10 {
        let key = format!("TUT_{i}_START_MIN");
        match env::var(key) {
            Ok(min) => mins.push(
                min
                    .parse()
                    .expect(&format!("{min} is not a valid usize"))
            ),
            Err(_) => break,
        }
    }

    mins
});

pub const TUT_DURATION_MINS: LazyCell<Vec<usize>> = LazyCell::new(|| {
    let mut durs = Vec::new();

    for i in 1..10 {
        let key = format!("TUT_{i}_DURATION_MIN");
        match env::var(key) {
            Ok(duration) => durs.push(
                duration
                    .parse()
                    .expect(&format!("{duration} is not a valid usize"))
            ),
            Err(_) => break,
        }
    }

    durs
});
