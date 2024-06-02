use super::{
    group::Group,
    group_index::GroupIndex,
    group_sigle::{GroupType, SigleGroup},
};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Groups(Vec<Option<Group>>);

impl Default for Groups {
    fn default() -> Self {
        Self(vec![None; 3])
    }
}

impl Groups {
    pub fn insert_or_push(&mut self, new_group: Group) {
        let number = new_group.number.to_usize();
        if number >= self.0.len() {
            self.0
                .extend((0..(number - self.0.len() + 4)).into_iter().map(|_| None));
        }
        if let Some(group) = &mut self.0[number] {
            group.add_period(new_group);
        } else {
            self.0[number] = Some(new_group);
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = Group> {
        self.0.into_iter().filter_map(|g| g)
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

    pub fn get_closed(&self, group_type: GroupType, sigle: &CompactString) -> Vec<SigleGroup> {
        self.iter()
            .filter(|g| !g.open)
            .map(|g| SigleGroup::new(sigle.clone(), group_type, g.number))
            .collect()
    }
}

impl Index<GroupIndex> for Groups {
    fn index(&self, index: GroupIndex) -> &Self::Output {
        &self.0[index.to_usize()]
    }

    type Output = Option<Group>;
}

impl IndexMut<GroupIndex> for Groups {
    fn index_mut(&mut self, index: GroupIndex) -> &mut Self::Output {
        &mut self.0[index.to_usize()]
    }
}

impl Index<usize> for Groups {
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }

    type Output = Option<Group>;
}

impl IndexMut<usize> for Groups {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
