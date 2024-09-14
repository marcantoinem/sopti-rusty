use crate::{item::ItemVariant, score::Score};

#[derive(Clone)]
pub struct Collection<const MAX_ITEMS: usize, S, V>
where
    S: Score,
    V: ItemVariant,
{
    pub score: S,
    pub item_quantity: usize,
    pub item_variant: [V; MAX_ITEMS],
}

impl<const MAX_ITEMS: usize, S: Score, V: ItemVariant> Collection<MAX_ITEMS, S, V> {
    pub fn push(&self, item_variant: V) -> Self {
        let mut new = self.clone();
        new.item_variant[self.item_quantity] = item_variant;
        new.item_quantity += 1;
        new
    }
}

impl<const MAX_ITEMS: usize, S: Score, V: ItemVariant> Default for Collection<MAX_ITEMS, S, V> {
    fn default() -> Self {
        let score = S::default();
        let item_quantity = 0;
        let item_variant = [V::default(); MAX_ITEMS];
        Self {
            score,
            item_quantity,
            item_variant,
        }
    }
}

impl<const MAX_ITEMS: usize, S: Score, V: ItemVariant> PartialEq for Collection<MAX_ITEMS, S, V> {
    fn eq(&self, other: &Self) -> bool {
        self.item_variant == other.item_variant
    }
}

impl<const MAX_ITEMS: usize, S: Score, V: ItemVariant> PartialOrd for Collection<MAX_ITEMS, S, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<const MAX_ITEMS: usize, S: Score, V: ItemVariant> Eq for Collection<MAX_ITEMS, S, V> {}

impl<const MAX_ITEMS: usize, S: Score, V: ItemVariant> Ord for Collection<MAX_ITEMS, S, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}
