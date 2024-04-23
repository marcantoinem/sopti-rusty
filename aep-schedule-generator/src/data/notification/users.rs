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

struct UsersNotify {
    courses: HashMap<SigleGroup, Vec<Arc<NotificationMethod>>>,
    push_notification_users: HashMap<AuthToken, Arc<NotificationMethod>>,
}