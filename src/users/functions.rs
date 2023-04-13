use crate::allocator::ID_Allocator;
use crate::app::App;
use crate::database::columns;
use crate::users::roles::UserRole;
use crate::users::user::{User, UserID};
use std::env::current_exe;

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

// get user by id
fn get_user_by_id(user_id: UserID) -> User {
    todo!()
    // get user
}

pub fn turn_row_into_user(row: &csv::StringRecord) -> User {
    User {
        first_name: row
            .get(columns::COL_INDEX_USER_FIRST_NAME)
            .unwrap()
            .to_string(),
        last_name: row
            .get(columns::COL_INDEX_USER_LAST_NAME)
            .unwrap()
            .to_string(),
        login_name: row
            .get(columns::COL_INDEX_USER_LOGIN_NAME)
            .unwrap()
            .to_string(),
        email: row.get(columns::COL_INDEX_USER_EMAIL).unwrap().to_string(),
        id: UserID(
            row.get(columns::COL_INDEX_USER_ID)
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        ),
        role: crate::users::roles::get_role_variant(row.get(columns::COL_INDEX_USER_ROLE).unwrap()),
        password: row
            .get(columns::COL_INDEX_USER_PASSWORD)
            .unwrap()
            .to_string(),
        registered: row
            .get(columns::COL_INDEX_USER_REGISTERED)
            .unwrap()
            .to_string(),
    }
}
