use crate::account::gb_account::AccountID;
use crate::entry::entry_id::EntryID;
use serde::{Deserialize, Serialize};

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

pub enum GB_EventCode {
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
    EntryDeleted = 203,
    // Category events
    CategoryAdded = 300,
    CategoryChanged = 301,
    CategoryRemoved = 302,
}
