use crate::{manager::user::core::UserData, api::utils::ApiDbError};

pub trait UserHandle {
    fn user_data(&self) -> &UserData;
    
    fn into_user_data(self) -> UserData;

    fn check_permission(&self) -> Result<(), ApiDbError> {
        todo!()
    }
}
