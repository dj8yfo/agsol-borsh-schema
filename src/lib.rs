//! This library aims to facilitate the workflow between Rust and TypeScript data
//! structures by auto-generating ! TypeScript classes and respective serialization
//! layouts used for Borsh (de)serialization. Check out
//! [`borsh-js`](https://github.com/near/borsh-js) and
//! [`borsh-rs`](https://docs.rs/borsh/0.9.1/borsh/index.html) for more details.


/// Intermediate data structures used for generating
/// schema an TypeScript class layouts.
mod layout;

#[cfg(test)]
mod test;

mod utils;

pub use layout::Layout;
pub use utils::*;

