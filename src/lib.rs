pub mod array;
pub mod fp;
pub mod sync;

pub use tap;

pub mod prelude {
    pub use super::array::*;
    pub use super::fp::*;
    pub use super::sync::*;
    pub use tap::prelude::*;
}