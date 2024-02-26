use super::{group::Group, group_index::GroupIndex};
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Groups(Vec<Option<Group>>);

impl Default for Groups {
    fn default() -> Self {
        Self(vec![None; 5])
    }
}

impl Groups {
    pub fn insert_or_push(&mut self, new_group: Group) {
        let number = new_group.number;
        if number.to_usize() >= self.0.len() {
            self.0.extend(
                (0..=number.to_usize() - self.0.len())
                    .into_iter()
                    .map(|_| None),
            );
        }
        if let Some(group) = &mut self.0[number.to_usize()] {
            group.add_period(new_group);
        } else {
            self.0[number.to_usize()] = Some(new_group);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Group> {
        self.0.iter().filter_map(|g| g.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Group> {
        self.0.iter_mut().filter_map(|g| g.as_mut())
    }

    pub fn is_empty(&self) -> bool {
        self.0.iter().all(|g| *g == None)
    }

    pub fn get_mut(&mut self, index: GroupIndex) -> Option<&mut Group> {
        self[index].as_mut()
    }
}

impl Index<GroupIndex> for Groups {
    fn index(&self, index: GroupIndex) -> &Option<Group> {
        &self.0[index.to_usize()]
    }

    type Output = Option<Group>;
}

impl IndexMut<GroupIndex> for Groups {
    fn index_mut(&mut self, index: GroupIndex) -> &mut Self::Output {
        &mut self.0[index.to_usize()]
    }
}
