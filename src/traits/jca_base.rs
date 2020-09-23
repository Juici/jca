use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

/// Base trait for types in the library.
pub trait JcaBase: Clone + Display + Debug + for<'a> Deserialize<'a> + Serialize {}
