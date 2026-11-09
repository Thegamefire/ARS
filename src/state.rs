use crate::messages::NetworkItem;
use crate::messages::client::create_hints::HintStatus;
use crate::seed::Seed;
use crate::structs::{LocationId, SlotAlias, SlotId, TeamId};
use std::collections::{HashMap, HashSet};
use crate::messages::client::create_hints::HintStatus::Found;

pub struct MultiworldState {
    collected_locations: HashMap<SlotId, HashSet<LocationId>>,
    hinted_locations: HashMap<SlotId, HashMap<LocationId, HintStatus>>,
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
            s.hinted_locations.insert(slot, HashMap::new());
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
        let location_set = self
            .collected_locations
            .get_mut(slot)
            .expect("Adding locations not in seed");
        let mut empty = HashMap::new();
        let location_hints = self.hinted_locations.get_mut(slot).unwrap_or(&mut empty);
        for loc in locations {
            location_set.insert(loc);
            if location_hints.contains_key(&loc) {
                location_hints.insert(loc, Found);
            }
        }
    }

    pub fn hinted_locations(&self, slot: &SlotId) -> Vec<LocationId> {
        match self.hinted_locations.get(slot) {
            None => vec![],
            Some(set) => set.iter().map(|l| *l.0).collect(),
        }
    }

    pub fn add_hints(&mut self, locations: &Vec<(LocationId, HintStatus)>, slot: &SlotId) {
        match self.hinted_locations.get_mut(slot) {
            None => {
                let mut map = HashMap::new();
                for (l, s) in locations {
                    map.insert(*l, *s);
                }
                self.hinted_locations.insert(*slot, map);
            }
            Some(map) => {
                for (l, s) in locations {
                    if !map.contains_key(l) {
                        // Dont Update Status for existing hints (that should be done with another method)
                        map.insert(*l, *s);
                    }
                }
            }
        }
    }

    pub fn update_hint_status(
        &mut self,
        location_owner: SlotId,
        location: LocationId,
        status: HintStatus,
    ) {
        let Some(slot_hints) = self.hinted_locations.get_mut(&location_owner) else {
            return;
        };

        slot_hints.insert(location, status);
    }
}
