use serde::{Deserialize, Serialize};

pub mod sea_orm_json;

pub trait ThinWrapper {}
pub trait ThinWrapperSerde<'de>: ThinWrapper + Serialize + Deserialize<'de> {}

pub use utils_macro::ThinWrapper;
pub use utils_macro::ThinWrapperSerde;
