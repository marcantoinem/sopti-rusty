use super::group::Group;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Groups(pub Vec<Group>);

impl Default for Groups {
    fn default() -> Self {
        Self(Vec::with_capacity(3))
    }
}

impl Groups {
    pub fn insert_or_push(&mut self, new_group: Group) {
        if let Some(group) = self.0.iter_mut().find(|g| g.number == new_group.number) {
            group.add_period(new_group);
        } else {
            self.0.push(new_group);
        }
    }

    pub fn get_mut(&mut self, number: u8) -> Option<&mut Group> {
        self.0.iter_mut().find(|g| g.number == number)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Group> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Index<usize> for Groups {
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }

    type Output = Group;
}

impl IndexMut<usize> for Groups {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl IntoIterator for Groups {
    type Item = Group;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
