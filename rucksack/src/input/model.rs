//! # Input Data Model
//!
//! This is the data model that unifies all input to the application. Possible
//! sources of input include:
//!
//! * ENV variables
//! * CLI flags / options / args
//! * Values in a config file
//! * CLI parser defaults
//! * Module-level consts/defaults
//! * Library-level consts/defaults
//!
//! The ordering of this list represents the order precedence for these as
//! well, from highest priority to lowest priority.
