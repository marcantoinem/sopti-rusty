use crate::score::Score;

#[derive(Clone)]
pub struct Collection<const MAX_ITEMS: usize, S, ItemVariant>
where
    S: Score,
    ItemVariant: Eq,
{
    pub score: S,
    pub item_quantity: usize,
    pub item_variant: [ItemVariant; MAX_ITEMS],
}

impl<const MAX_ITEMS: usize, S: Score, ItemVariant: Eq + Default + Copy>
    Collection<MAX_ITEMS, S, ItemVariant>
{
    pub fn push(&self, item_variant: ItemVariant) -> Self {
        let mut new = self.clone();
        new.item_variant[self.item_quantity] = item_variant;
        new.item_quantity += 1;
        new
    }
}

impl<const MAX_ITEMS: usize, S: Score, ItemVariant: Eq + Default + Copy> Default
    for Collection<MAX_ITEMS, S, ItemVariant>
{
    fn default() -> Self {
        let score = S::default();
        let item_quantity = 0;
        let item_variant = [ItemVariant::default(); MAX_ITEMS];
        Self {
            score,
            item_quantity,
            item_variant,
        }
    }
}

impl<const MAX_ITEMS: usize, S: Score, ItemVariant: Eq> PartialEq
    for Collection<MAX_ITEMS, S, ItemVariant>
{
    fn eq(&self, other: &Self) -> bool {
        self.item_variant == other.item_variant
    }
}

impl<const MAX_ITEMS: usize, S: Score, ItemVariant: Eq> PartialOrd
    for Collection<MAX_ITEMS, S, ItemVariant>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<const MAX_ITEMS: usize, S: Score, ItemVariant: Eq> Eq
    for Collection<MAX_ITEMS, S, ItemVariant>
{
}

impl<const MAX_ITEMS: usize, S: Score, ItemVariant: Eq> Ord
    for Collection<MAX_ITEMS, S, ItemVariant>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}
