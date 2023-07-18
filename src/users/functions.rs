use crate::{
    app::App,
    users::{
        roles::UserRole,
        user::{User, UserID},
    },
};

#[allow(dead_code)]
fn get_current_user(app: &App) -> Option<User> {
    todo!()
}

#[allow(dead_code)]
pub fn get_current_user_id(app: &App) -> UserID {
    todo!()
}

#[allow(dead_code)]
fn add_role_to_user(_user_id: u32, _role: UserRole) -> bool {
    // get user by id
    // check role
    // change role

    true
}

// add to route
#[allow(dead_code)]
pub fn get_user_by_email(_email: &str) -> Option<User> {
    todo!()
}
