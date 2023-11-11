use super::group::Group;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Groups(Vec<Option<Group>>);

impl Default for Groups {
    fn default() -> Self {
        Self(vec![None; 3])
    }
}

impl Groups {
    pub fn insert_or_push(&mut self, new_group: Group) {
        let n = new_group.number as usize;
        if self.0.len() <= n {
            self.0.extend((0..=n).map(|_| None));
        }
        if let Some(group) = &mut self.0[n] {
            for new_period in new_group.periods {
                if let Some(period) = group
                    .periods
                    .iter_mut()
                    .find(|p| p.day == new_period.day && p.room == new_period.room)
                {
                    period.hours |= new_period.hours;
                } else {
                    group.periods.push(new_period);
                }
            }
        } else {
            self.0[n] = Some(new_group);
        }
    }

    pub fn get_mut(&mut self, number: u8) -> Option<&mut Group> {
        self.0[number as usize].as_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Group> {
        self.0.iter().filter_map(|g| g.as_ref())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
