use crate::account::gb_account::AccountID;
use std::collections::HashMap;

// Define a procedural macro to generate the enum variants and values.
macro_rules! generate_enum {
    ($enum_name:ident { $($variant:ident = $value:expr),* }) => {
        pub enum $enum_name {
            $($variant = $value),*
        }

        impl $enum_name {
            pub fn from_value(value: u32) -> Self {
                match value {
                    $($value => $enum_name::$variant),*,
                    _ => $enum_name::Unknown,
                }
            }
        }
    };
}

generate_enum!(GB_EventCode {
    // Account events
    AccountRegistered = 100,
    AccountLogin = 101,
    AccountLogout = 102,
    AccountKicked = 103,
    AccountRemoved = 104,
    AccountRoleChanged = 105,
    AccountEmailChanged = 106,
    AccountPasswordChanged = 107,
    // Entry events
    EntryAdded = 200,
    EntryStatusChanged = 201,
    EntryTrashed = 202,
    EntryUntrashed = 203,
    EntryDeleted = 204,
    // Category events
    CategoryAdded = 300,
    CategoryChanged = 301,
    CategoryRemoved = 302,
    // File
    FileUploaded = 400,
    FileDeleted = 401,
    // Other
    Unknown = 999
});

//#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct GB_Log {
    // The account ID and email associated with this event - email optional for system events
    pub account_id: AccountID,
    pub account_email: Option<String>,
    // Event meta data:
    pub event_code: GB_EventCode,
    pub event_date: String,
    // Affected item details:
    pub subject_id: u32,
    pub subject_url: Option<String>,
    pub subject_title: Option<String>,
    pub subject_description: Option<String>,
    // From/To fields where applicable
    pub subject_from: Option<String>,
    pub subject_to: Option<String>,
}

impl GB_Log {
    pub fn get_event_description(&self) -> String {
        match self.event_code {
            GB_EventCode::AccountRegistered => "Registered an account".to_string(),
            GB_EventCode::AccountLogin => "Logged in".to_string(),
            GB_EventCode::AccountLogout => "Logged out".to_string(),
            GB_EventCode::AccountKicked => "Kicked a user out".to_string(),
            GB_EventCode::AccountRemoved => "Deleted an account".to_string(),
            GB_EventCode::AccountRoleChanged => "Changed account role".to_string(),
            GB_EventCode::AccountEmailChanged => "Changed account email".to_string(),
            GB_EventCode::AccountPasswordChanged => "Changed password".to_string(),
            GB_EventCode::EntryAdded => "Deleted an entry".to_string(),
            GB_EventCode::EntryStatusChanged => "Changed entry status".to_string(),
            GB_EventCode::EntryTrashed => "Moved an entry to the trash".to_string(),
            GB_EventCode::EntryUntrashed => "Restored an entry from the trash".to_string(),
            GB_EventCode::EntryDeleted => "Permanently delete an entry".to_string(),
            GB_EventCode::CategoryAdded => "Created a category".to_string(),
            GB_EventCode::CategoryChanged => "Edited a category".to_string(),
            GB_EventCode::CategoryRemoved => "Deleted a category".to_string(),
            GB_EventCode::FileUploaded => "Uploaded a file".to_string(),
            GB_EventCode::FileDeleted => "Deleted a file".to_string(),
            GB_EventCode::Unknown => "Unknown event".to_string(),
        }
    }
}

fn is_admin_only_event(event_code: GB_EventCode) -> bool {
    match event_code {
        GB_EventCode::AccountRegistered => true,
        GB_EventCode::AccountLogin => false,
        GB_EventCode::AccountLogout => false,
        GB_EventCode::AccountKicked => true,
        GB_EventCode::AccountRemoved => true,
        GB_EventCode::AccountRoleChanged => true,
        GB_EventCode::AccountEmailChanged => true,
        GB_EventCode::AccountPasswordChanged => true,
        GB_EventCode::EntryAdded => false,
        GB_EventCode::EntryStatusChanged => false,
        GB_EventCode::EntryTrashed => false,
        GB_EventCode::EntryUntrashed => false,
        GB_EventCode::EntryDeleted => false,
        GB_EventCode::CategoryAdded => false,
        GB_EventCode::CategoryChanged => false,
        GB_EventCode::CategoryRemoved => false,
        GB_EventCode::FileUploaded => false,
        GB_EventCode::FileDeleted => false,
        GB_EventCode::Unknown => false,
    }
}
