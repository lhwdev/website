#[macro_export]
macro_rules! edit {
    ($($original:ident).+ = $value:expr) => {
        if let Some(inner) = $value {
            $($original).+ = sea_orm::Set(inner);
        }
    };
}
