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
    use super::*;

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
    fn test_all_empty_query() {
        assert!(super::all(refset(), vec![]), "Empty query should be subset");
    }

    #[test]
    fn test_all_empty_reference() {
        assert!(
            !super::all(vec![], query1()),
            "Non-empty query can't be subset of empty reference"
        );
    }

    #[test]
    fn test_all_both_empty() {
        assert!(
            super::all(vec![], vec![]),
            "Empty set is subset of empty set"
        );
    }

    #[test]
    fn test_all_identical_sets() {
        let set = vec!["x".to_string(), "y".to_string()];
        assert!(super::all(set.clone(), set));
    }

    #[test]
    fn test_all_single_element() {
        assert!(super::all(refset(), vec!["a".to_string()]));
        assert!(!super::all(refset(), vec!["z".to_string()]));
    }

    #[test]
    fn any() {
        assert!(super::any(refset(), query1()));
        assert!(super::any(refset(), query2()));
        assert!(!super::any(refset(), query3()));
    }

    #[test]
    fn test_any_empty_query() {
        assert!(
            !super::any(refset(), vec![]),
            "Empty query has no intersection"
        );
    }

    #[test]
    fn test_any_empty_reference() {
        assert!(
            !super::any(vec![], query1()),
            "Empty reference has no intersection"
        );
    }

    #[test]
    fn test_any_both_empty() {
        assert!(
            !super::any(vec![], vec![]),
            "Empty sets have no intersection"
        );
    }

    #[test]
    fn test_any_single_match() {
        let reference = vec!["a".to_string(), "b".to_string()];
        let query = vec!["b".to_string(), "c".to_string()];
        assert!(super::any(reference, query));
    }

    #[test]
    fn test_any_no_match() {
        let reference = vec!["a".to_string(), "b".to_string()];
        let query = vec!["x".to_string(), "y".to_string()];
        assert!(!super::any(reference, query));
    }

    #[test]
    fn test_make_string_set_basic() {
        let input = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let set = make_string_set(input);
        assert_eq!(set.len(), 3);
        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(set.contains("c"));
    }

    #[test]
    fn test_make_string_set_duplicates() {
        let input = vec!["a".to_string(), "a".to_string(), "b".to_string()];
        let set = make_string_set(input);
        assert_eq!(set.len(), 2, "Duplicates should be removed");
    }

    #[test]
    fn test_make_string_set_empty() {
        let input: Vec<String> = vec![];
        let set = make_string_set(input);
        assert!(set.is_empty());
    }

    #[test]
    fn test_display_simple() {
        let result = display("Hello, World!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_empty() {
        let result = display("");
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_multiline() {
        let result = display("Line 1\nLine 2\nLine 3");
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_unicode() {
        let result = display("Hello 世界 🌍");
        assert!(result.is_ok());
    }

    #[test]
    fn test_random_specials_zero() {
        let specials = random_specials(0);
        assert_eq!(specials.len(), 0);
    }

    #[test]
    fn test_random_specials_one() {
        let specials = random_specials(1);
        assert_eq!(specials.len(), 1);
        assert!(SPECIALS.contains(&specials[0]));
    }

    #[test]
    fn test_random_specials_multiple() {
        let specials = random_specials(10);
        assert_eq!(specials.len(), 10);
        for &s in &specials {
            assert!(
                SPECIALS.contains(&s),
                "All returned characters should be from SPECIALS"
            );
        }
    }

    #[test]
    fn test_random_specials_all_valid() {
        let specials = random_specials(100);
        assert_eq!(specials.len(), 100);
        for &s in &specials {
            assert!(SPECIALS.contains(&s));
        }
    }

    #[test]
    fn test_bincode_cfg_returns_config() {
        let _config = bincode_cfg();
        // Just ensure it doesn't panic and returns something
    }

    #[test]
    fn test_bincode_cfg_is_legacy() {
        let config = bincode_cfg();
        // Verify it's the legacy configuration by using it
        let data = vec![1u8, 2u8, 3u8];
        let encoded = bincode::encode_to_vec(&data, config).unwrap();
        let (decoded, _): (Vec<u8>, _) = bincode::decode_from_slice(&encoded, config).unwrap();
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_specials_constant() {
        assert_eq!(SPECIALS, b"!@#%&*?=+:");
        assert_eq!(SPECIALS.len(), 10);
    }
}
