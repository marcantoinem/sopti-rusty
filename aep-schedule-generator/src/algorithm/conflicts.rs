use compact_str::CompactString;

pub struct Conflicts(Conflict, Conflict);

pub enum Conflict {
    Course { sigle: CompactString },
    Personal,
}
