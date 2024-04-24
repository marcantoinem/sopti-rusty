use std::{collections::HashMap, fmt::Display, sync::Arc};
use compact_str::CompactString;
use crate::data::group_index::GroupIndex;
use super::{auth_token::AuthToken, NotificationMethod};

#[derive(Debug, Clone, Copy, Hash)]
#[repr(u8)]
pub enum GroupType {
    LabGroup,
    TheoGroup,
}

impl Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LabGroup => f.write_str("laboratoire"),
            Self::TheoGroup => f.write_str("théorie"),
        }
    }
}

#[derive(Debug, Clone, Hash)]
pub struct SigleGroup {
    pub sigle: CompactString,
    pub group_type: GroupType,
    pub group_index: GroupIndex,
}

struct UsersToNotify {
    courses: HashMap<SigleGroup, Vec<Arc<NotificationMethod>>>,
    users: HashMap<AuthToken, (Arc<NotificationMethod>, Vec<SigleGroup>)>,
    to_notify: Vec<NotificationMethod>
}

impl UsersToNotify {
    async fn send_all_emails() {
        
    }
}