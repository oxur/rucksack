use std::collections::HashSet;

use anyhow::Result;
use rand::Rng;

const SPECIALS: &[u8] = b"!@#%&*?=+:";

// If all of elements in the query data set are present in the
// reference data set, return `true`.
pub fn all(reference: Vec<String>, query: Vec<String>) -> bool {
    let r = make_string_set(reference);
    let q = make_string_set(query);
    q.is_subset(&r)
}

// If any of the elements in the query data set are present in the
// reference data set, return `true`.
pub fn any(reference: Vec<String>, query: Vec<String>) -> bool {
    let r = make_string_set(reference);
    let q = make_string_set(query);
    if r.intersection(&q).count() == 0 {
        return false;
    }
    true
}

pub fn bincode_cfg() -> bincode::config::Configuration<
    bincode::config::LittleEndian,
    bincode::config::Fixint,
    bincode::config::NoLimit,
> {
    bincode::config::legacy()
}

pub fn display(text: &str) -> Result<()> {
    println!("{text}");
    Ok(())
}

pub fn make_string_set(input: Vec<String>) -> HashSet<String> {
    input.into_iter().collect()
}

pub fn random_specials(count: usize) -> Vec<u8> {
    let mut specials: Vec<u8> = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 1..count + 1 {
        specials.push(SPECIALS[rng.gen_range(0..SPECIALS.len())])
    }
    specials
}

#[cfg(test)]
mod tests {
    fn refset() -> Vec<String> {
        ["a", "b", "c", "d", "e", "f"]
            .iter()
            .map(|e| e.to_string())
            .collect()
    }

    fn query1() -> Vec<String> {
        vec!["b".to_string(), "e".to_string()]
    }

    fn query2() -> Vec<String> {
        vec!["b".to_string(), "g".to_string()]
    }

    fn query3() -> Vec<String> {
        vec!["h".to_string(), "g".to_string()]
    }

    #[test]
    fn all() {
        assert!(super::all(refset(), query1()));
        assert!(!super::all(refset(), query2()));
        assert!(!super::all(refset(), query3()));
    }

    #[test]
    fn any() {
        assert!(super::any(refset(), query1()));
        assert!(super::any(refset(), query2()));
        assert!(!super::any(refset(), query3()));
    }
}
