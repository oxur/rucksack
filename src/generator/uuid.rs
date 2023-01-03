use rand::distributions::{Distribution, Uniform};
use std::str;

use uuid::Uuid;

use crate::util;

pub fn v4_string() -> String {
    Uuid::new_v4().to_string()
}

pub fn v4_with_uppers() -> String {
    let uuid = v4_string();
    let parts: Vec<&str> = uuid.split('-').collect();
    let (first_part, rest_parts) = parts.split_at(1);
    let first = first_part.to_vec().pop().unwrap().to_uppercase();
    let mut rest = rest_parts.to_vec();
    let last = rest.pop().unwrap().to_uppercase();
    rest.insert(0, &first);
    rest.push(&last);
    rest.join("-")
}

pub fn v4_with_specials(count: usize) -> String {
    let mut rng = rand::thread_rng();
    let uuid = v4_with_uppers();
    let mut parts: Vec<String> = uuid.split("").map(|s| s.to_string()).collect();
    let len = parts.len();
    let die = Uniform::from(1..len);
    let specials = util::random_specials(count);
    for special in specials.iter().take(count) {
        let throw = die.sample(&mut rng);
        parts[throw] = String::from_utf8_lossy(&[*special]).to_string();
    }
    parts.join("")
}
