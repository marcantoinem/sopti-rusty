use std::{cmp::Reverse, collections::BinaryHeap, num::NonZeroUsize};

use crate::{
    collection::Collection,
    item::{CombinatoricItem, ItemVariant},
    parameters::Parameters,
    score::Score,
};

pub struct Solver<const MAX_ITEMS: usize, S, V, I, Parameters>
where
    S: Score,
    V: ItemVariant,
    I: CombinatoricItem,
{
    /// Min-heap to keep the best schedules and fetch the worst of the best in O(1)
    top_items: BinaryHeap<Reverse<Collection<MAX_ITEMS, S, V>>>,
    max_capacity: NonZeroUsize,
    parameters: Parameters,
    items: Vec<I>,
}

impl<'a, const MAX_ITEMS: usize, S, V, I, P> Solver<MAX_ITEMS, S, V, I, P>
where
    S: Score,
    V: ItemVariant,
    I: CombinatoricItem<ItemVariant = V>,
    P: Parameters<V>,
{
    pub fn new(max_capacity: NonZeroUsize, items: Vec<I>, parameters: P) -> Self {
        let top_items = BinaryHeap::with_capacity(max_capacity.into());
        Self {
            top_items,
            max_capacity,
            parameters,
            items,
        }
    }
    pub fn solve(mut self) -> Vec<I> {
        let item = &self.items;
        Self::rec_iter(
            0,
            Collection::default(),
            self.max_capacity,
            &self.parameters,
            item,
            &mut self.top_items,
        );
        vec![]
    }
    fn rec_iter(
        i: usize,
        collection: Collection<MAX_ITEMS, S, V>,
        max_capacity: NonZeroUsize,
        parameters: &P,
        items: &'a [I],
        top_items: &mut BinaryHeap<Reverse<Collection<MAX_ITEMS, S, V>>>,
    ) {
        if i >= items.len() {
            // If it reach this branch it is already valid and better than the worst top schedule, so we can push
            if top_items.len() >= max_capacity.into() {
                top_items.pop();
            }
            top_items.push(Reverse(collection));
            return;
        }

        for variant in items[i].variants() {
            if !parameters.is_valid(collection.item_variant(i), variant) {
                return;
            }
            let collections = collection.push(i, variant);
            // SAFETY: if the len is greater than the max_capacity, it is greater than 0 enforced by the NonZeroUsize, which allows us to unwrap without checking if it is None
            if top_items.len() >= max_capacity.into()
                && unsafe { top_items.peek().unwrap_unchecked().0 >= collections }
            {
                return;
            }
            Self::rec_iter(
                i + 1,
                collections,
                max_capacity,
                parameters,
                items,
                top_items,
            );
        }
    }
}
