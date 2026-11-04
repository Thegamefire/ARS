use crate::messages::NetworkItem;
use crate::seed::Seed;
use crate::structs::{LocationId, SlotAlias, SlotId, TeamId};
use std::collections::{HashMap, HashSet};

pub struct MultiworldState {
    collected_locations: HashMap<SlotId, HashSet<LocationId>>,
    hinted_locations: HashMap<SlotId, HashSet<LocationId>>,
    slot_aliases: HashMap<(TeamId, SlotId), SlotAlias>,
}
impl MultiworldState {
    pub fn new(seed: &Seed) -> MultiworldState {
        let mut s = MultiworldState {
            collected_locations: HashMap::new(),
            hinted_locations: HashMap::new(),
            slot_aliases: HashMap::new(),
        };
        for slot in seed.slot_ids() {
            s.collected_locations.insert(slot, HashSet::new());
            s.hinted_locations.insert(slot, HashSet::new());
        }
        s
    }
    pub fn missing_locations<'a>(&'a self, seed: &'a Seed, slot: &SlotId) -> Vec<&'a LocationId> {
        let all_locations = seed
            .locations
            .get(slot)
            .map(|v| v.keys().collect())
            .unwrap_or(vec![]);
        let collected_locations = self.collected_locations(slot);
        all_locations
            .into_iter()
            .filter(|loc| !collected_locations.contains(loc))
            .collect()
    }
    pub fn collected_locations(&self, slot: &SlotId) -> Vec<&LocationId> {
        self.collected_locations
            .get(slot)
            .map(|s| s.iter().collect())
            .unwrap_or(vec![])
    }

    pub fn received_items(&self, seed: &Seed, slot: &SlotId) -> Vec<NetworkItem> {
        let mut result = vec![];
        for (location_slot, location_mapping) in &seed.locations {
            for (location, item) in location_mapping {
                if item.1 == *slot {
                    result.push(NetworkItem::new(item.0, *location, *location_slot, item.2))
                }
            }
        }
        result
    }

    pub fn check_locations(&mut self, slot: &SlotId, locations: Vec<LocationId>) {
        let location_set = self.collected_locations.get_mut(slot).expect("Adding locations not in seed");
        for loc in locations {
            location_set.insert(loc);
        }
    }

    pub fn hinted_locations(&self, slot: &SlotId) -> Vec<LocationId> {
        match self.hinted_locations.get(slot) {
            None => vec![],
            Some(set) => set.iter().map(|l| *l).collect()
        }
    }

    pub fn add_hints(&mut self, locations: &Vec<LocationId>, slot: &SlotId) {
        match self.hinted_locations.get_mut(slot) {
            None => {
                let mut set = HashSet::new();
                for l in locations {
                    set.insert(*l);
                }
                self.hinted_locations.insert(*slot, set);
            }
            Some(set) => {
                for l in locations {
                    set.insert(*l);
                }
            }
        }
    }
}
