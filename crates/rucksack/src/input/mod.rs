#[doc(hidden)]
pub mod config;
#[doc(hidden)]
pub mod constant;
#[doc(hidden)]
pub mod model;
#[doc(hidden)]
pub mod options;
#[doc(hidden)]
pub mod prompt;
#[doc(hidden)]
pub mod query;
#[doc(hidden)]
pub mod testing;

pub use config::Config;
pub use model::{Flag, Inputs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inputs_reexport() {
        let inputs = Inputs::default();
        assert_eq!(inputs.logging.level, "");
    }

    #[test]
    fn test_flag_enum() {
        // Just verify Flag enum variants exist
        let _flag_one = Flag::One;
        let _flag_many = Flag::Many;
        assert!(true);
    }
}
