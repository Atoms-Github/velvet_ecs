use crate::{GlobalEntityID, ZType};
use crate::comp_store::CompStorage;
use rdxsort::{RdxSort, RdxSortTemplate};

#[derive(Clone)]
struct SortEnt{
    pub ent_id: GlobalEntityID,
    pub value: u16,
}

pub fn go(store: &CompStorage, entities: Vec<GlobalEntityID>,
          sort_by: fn(&CompStorage, GlobalEntityID) -> ZType) -> Vec<GlobalEntityID>{
    let sortable = entities.iter().map(|id| {
        let value = sort_by(store, *id);
        SortEnt{
            ent_id: *id,
            value
        }
    });
    let mut sortable : Vec<SortEnt> = sortable.collect();
    sortable.rdxsort();
    return sortable.iter().map(|sort_ent|{sort_ent.ent_id}).collect();
}
impl RdxSortTemplate for SortEnt{
    #[inline]
    fn cfg_nbuckets() -> usize {
        256
    }

    #[inline]
    fn cfg_nrounds() -> usize {
        2
    }

    #[inline]
    fn get_bucket(&self, round: usize) -> usize {
        if round == 0 {
            self.value as u8 as usize
        } else {
            (self.value >> 8) as u8 as usize
        }
    }

    #[inline]
    fn reverse(_round: usize, _bucket: usize) -> bool {
        // not required in our case
        false
    }
}