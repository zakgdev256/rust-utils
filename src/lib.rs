pub mod array;
pub mod functional;
pub mod parallelism;

pub use tap;

pub mod prelude {
    pub use super::array::*;
    pub use super::functional::*;
    pub use super::parallelism::*;
    pub use tap::prelude::*;
}