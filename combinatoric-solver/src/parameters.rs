use crate::item::ItemVariant;

pub trait Parameters<V>
where
    V: ItemVariant,
{
    fn is_valid(&self, previous_variants: &[V], new_variant: V) -> bool;
}
