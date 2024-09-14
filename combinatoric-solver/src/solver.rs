use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{
    collection::Collection,
    item::{CombinatoricItem, ItemVariant},
    score::Score,
};

pub struct Solver<const MAX_ITEMS: usize, S, V, I, Parameters>
where
    S: Score,
    V: ItemVariant,
    I: CombinatoricItem,
{
    top_items: BinaryHeap<Reverse<Collection<MAX_ITEMS, S, V>>>,
    max_capacity: usize,
    parameters: Parameters,
    items: Vec<I>,
}

impl<'a, const MAX_ITEMS: usize, S, V, I, Parameters> Solver<MAX_ITEMS, S, V, I, Parameters>
where
    S: Score,
    V: ItemVariant,
    I: CombinatoricItem<ItemVariant = V>,
{
    pub fn new(max_capacity: usize, items: Vec<I>, parameters: Parameters) -> Self {
        let top_items = BinaryHeap::with_capacity(max_capacity);
        Self {
            top_items,
            max_capacity,
            parameters,
            items,
        }
    }
    pub fn solve(mut self) -> Vec<I> {
        let item = &self.items;
        Self::rec_iter(0, Collection::default(), item, &mut self.top_items);
        vec![]
    }
    fn rec_iter(
        i: usize,
        collection: Collection<MAX_ITEMS, S, V>,
        items: &'a [I],
        top_items: &mut BinaryHeap<Reverse<Collection<MAX_ITEMS, S, V>>>,
    ) {
        if i >= items.len() {
            return;
        }

        for variant in items[i].variants() {
            let collections = collection.push(variant);
            Self::rec_iter(i + 1, collections, items, top_items);
        }
    }
}
