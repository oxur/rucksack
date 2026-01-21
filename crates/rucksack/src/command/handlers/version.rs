use anyhow::Result;

use rucksack_lib::util;

pub fn version() -> Result<()> {
    return util::display(crate::version().to_string().as_str());
}
