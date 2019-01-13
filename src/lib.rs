//! "insta" is a simple snapshot testing library for Rust.
//! 
//! # How it Operates
//! 
//! This crate exports two basic macros for snapshot testing:
//! `assert_snapshot_matches!` for comparing basic string snapshots and
//! `assert_debug_snapshot_matches!` for snapshotting the debug print output of
//! a type.  Additionally if the `serialization` feature is enabled the
//! `assert_serialized_snapshot_matches!` macro becomes available which
//! serializes an object with `serde` to yaml before snapshotting.
//! 
//! Snapshots are stored in the `snapshots` folder right next to the test file
//! where this is used.  The name of the file is `<module>__<name>.snap` where
//! the `name` of the snapshot has to be provided to the assertion macro.
//! 
//! To update the snapshots export the `INSTA_UPDATE` environment variable
//! and set it to `1`.  The snapshots can then be committed.
//! 
//! # Example
//! 
//! ```rust,ignore
//! use insta::assert_debug_snapshot_matches;
//! 
//! #[test]
//! fn test_snapshots() {
//!     let value = vec![1, 2, 3];
//!     assert_debug_snapshot_matches!("snapshot_name", value);
//! }
//! ```
//! 
//! The recommended flow is to run the tests once, have them fail and check
//! if the result is okay.  Once you are satisifed run the tests again with
//! `INSTA_UPDATE` set to `1` and updates will be stored:
//! 
//! ```ignore
//! $ INSTA_UPDATE=1 cargo test
//! ```
//! 
//! # Snapshot files
//! 
//! The committed snapshot files will have a header with some meta information
//! that can make debugging easier and the snapshot:
//! 
//! ```ignore
//! Created: 2019-01-13T22:16:48.669496+00:00
//! Creator: insta@0.1.0
//! Source: tests/test_snapshots.rs
//! 
//! [
//!     1,
//!     2,
//!     3
//! ]
//! ```
#[macro_use]
mod macros;
mod runtime;
#[cfg(test)]
mod test;

#[doc(hidden)]
pub mod _macro_support {
    pub use crate::runtime::assert_snapshot;
    #[cfg(feature = "serialization")]
    pub use crate::runtime::serialize_value;
}
