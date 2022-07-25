use serde::{Deserialize, Serialize};

pub trait ThinWrapper {}
pub trait ThinWrapperSerde<'de>: ThinWrapper + Serialize + Deserialize<'de> {}

// pub use utils_macro::ThinWrapper;
pub use utils_macro::ThinWrapperSerde;
