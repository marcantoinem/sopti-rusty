use std::{collections::HashMap, sync::Arc};
use compact_str::CompactString;
use crate::data::group_index::GroupIndex;
use super::{auth_token::AuthToken, NotificationMethod};

#[derive(Debug, Clone, Copy, Hash)]
#[repr(u8)]
enum GroupType {
    LabGroup,
    TheoGroup,
}

#[derive(Debug, Clone, Hash)]
pub struct SigleGroup {
    sigle: CompactString,
    group_type: GroupType,
    group_index: GroupIndex,
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