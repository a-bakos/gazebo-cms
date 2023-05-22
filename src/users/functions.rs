use crate::allocator::ID_Allocator;
use crate::app::App;
use crate::database::columns;
use crate::users::roles::UserRole;
use crate::users::user::{User, UserID};

#[allow(dead_code)]
pub fn get_next_available_user_id(app: &mut App) -> UserID {
    UserID::allocate(app)
}

#[allow(dead_code)]
fn get_current_user(app: &App) -> Option<User> {
    if app.current_user.is_some() {
        // get user row from DB and fill in details
        let current_user_id = app.current_user.clone();
        let current_user = get_user_by_id(current_user_id.unwrap());
        return Some(current_user);
    }
    None
}

pub fn get_current_user_id(app: &App) -> UserID {
    if app.current_user.is_some() {
        return app.current_user.clone().unwrap();
    }
    crate::consts::SYSTEM_USER_ID
}

fn add_role_to_user(_user_id: u32, _role: UserRole) -> bool {
    // get user by id
    // check role
    // change role

    true
}

// get user by id
#[allow(dead_code)]
fn get_user_by_id(_user_id: UserID) -> User {
    todo!()
    // get user
}

pub fn get_user_by_email(_email: &str) -> Option<User> {
    todo!()
}

pub fn turn_row_into_user(row: &csv::StringRecord) -> User {
    User {
        login_name: row.get("login".parse().unwrap()).unwrap().to_string(),
        email: row.get("email".parse().unwrap()).unwrap().to_string(),
        id: UserID(
            row.get("id".parse().unwrap())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        ),
        role: crate::users::roles::get_role_variant(row.get("role".parse().unwrap()).unwrap()),
        password: row.get("password".parse().unwrap()).unwrap().to_string(),
        registered: row.get("registered".parse().unwrap()).unwrap().to_string(),
    }
}
