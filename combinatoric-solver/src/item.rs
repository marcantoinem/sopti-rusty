pub trait ItemVariant: Eq + Copy + Default {}

pub trait CombinatoricItem {
    type ItemVariant;
    fn variants(&self) -> impl IntoIterator<Item = Self::ItemVariant>;
}
