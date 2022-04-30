//! Types used across the library.

mod core;
mod error;
mod node;
mod preset_node;

// Exports.
pub use self::core::*;
pub use self::error::Error;
pub use self::node::{Auth, Node};
pub use self::preset_node::PresetNode;

/// The canonical [Result] type used across the library, with [Error] as the error type.
///
/// [Result]: ::core::result::Result
/// [Error]: self::error::Error
pub type Result<T> = ::core::result::Result<T, self::error::Error>;
