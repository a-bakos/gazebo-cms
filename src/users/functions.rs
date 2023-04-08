use crate::allocator::ID_Allocator;
use crate::app::App;
use crate::users::user::{User, UserID};

#[allow(dead_code)]
pub fn get_next_available_user_id(app: &mut App) -> UserID {
    UserID::allocate(app)
}

#[allow(dead_code)]
fn get_current_user() -> User {
    todo!()
}

// get user by id
