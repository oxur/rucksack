use anyhow::Result;

use rucksack_lib::util;

pub fn version() -> Result<()> {
    util::display(crate::version().to_string().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        // Should not panic and should return Ok
        let result = version();
        assert!(result.is_ok());
    }
}
