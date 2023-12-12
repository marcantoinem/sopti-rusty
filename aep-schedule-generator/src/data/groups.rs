use super::group::Group;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Groups(Vec<Group>);

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
