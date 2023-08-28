use crate::account::gb_account::AccountID;
use crate::entry::entry_id::EntryID;
use serde::{Deserialize, Serialize};

//#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[allow(non_camel_case_types)]
pub struct GB_Log {
    pub entry_id: EntryID,
    pub account_id: AccountID,
    pub account_email: String,
    pub entry_url: Option<String>,
    pub entry_title: Option<String>,
    pub event_code: GB_EventCode,
    pub event_date: String,
    pub description: Option<String>,
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
