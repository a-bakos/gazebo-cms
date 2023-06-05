use crate::app::App;
use crate::users::roles::UserRole;
use crate::users::user::{User, UserID};

#[allow(dead_code)]
fn get_current_user(app: &App) -> Option<User> {
    if app.current_user.is_some() {
        // get user row from DB and fill in details
        let _current_user_id = app.current_user.clone();
        todo!()
        // let current_user = get_user_by_id(current_user_id.unwrap());
        // return Some(current_user);
    }
    None
}

#[allow(dead_code)]
pub fn get_current_user_id(app: &App) -> UserID {
    if app.current_user.is_some() {
        return app.current_user.clone().unwrap();
    }
    crate::consts::SYSTEM_USER_ID
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
