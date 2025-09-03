use time::{Duration, PrimitiveDateTime as DateTime};

pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn after(start: DateTime) -> DateTime {
    let billion_seconds = 10i64.pow(9);
    let duration = Duration::seconds(billion_seconds);

    start.checked_add(duration).unwrap()
}

// to-do cli
pub fn todo_cli() {}

fn main() {}
