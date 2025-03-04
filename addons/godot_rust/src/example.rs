use super::*;

mod node;
mod refcounted;
mod singleton;
mod virtual_func;

// Singleton depends on MyNode
// Library depends on Singleton

pub use self::{node::*, refcounted::*, singleton::*, virtual_func::*};
