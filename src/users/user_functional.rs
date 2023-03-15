use crate::allocator::ID_Allocator;
use crate::app::App;
use crate::users::user::{User, UserID};

pub fn get_next_available_user_id(app: &mut App) -> UserID {
    UserID::allocate(app)
}

fn get_current_user() -> User {
    todo!()
}
