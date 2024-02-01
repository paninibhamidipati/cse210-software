
use std::collections::HashMap;

#[derive(Debug)]
pub struct ClanSystem {
    pub clans: HashMap<String, Vec<String>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem {
            clans: HashMap::new(),
        }
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        self.clans.get(clan_id).cloned().unwrap_or_else(Vec::new)
    }

    /**
     * Returns the number of clans currently in existence.
     */
    
    // Retrieves the count of members in the clan identified by `clan_id`.
    // The `get` method is used to access the value (Vec<String> of crab names) associated with the `clan_id` key in the `clans` HashMap.
    // The `map_or` function is then applied to handle two cases:
    // 1. If the clan exists (`Some` case), it maps the value to its length, effectively counting the number of members in the clan.
    // 2. If the clan does not exist (`None` case), it returns a default value of 0.
    // This approach ensures that we gracefully handle the absence of a clan without causing a runtime error.
    pub fn get_clan_count(&self) -> usize {
        self.clans.len()
    }
    
    // Adds a crab with the specified name to the clan identified by `clan_id`.
    // The `entry` method is used to obtain a mutable reference to the value (Vec<String> of crab names) for the given `clan_id` key.
    // If `clan_id` does not already exist in the `clans` HashMap, the `entry` method automatically inserts a new Vec<String> and returns a reference to it.
    // The crab's name is then appended to this vector, effectively adding the crab to the clan.
    
    pub fn add_member(&mut self, clan_id: &str, crab_name: String) {
        self.clans.entry(clan_id.to_string())
            .or_insert_with(Vec::new)
            .push(crab_name);
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        self.clans.get(clan_id).map_or(0, |members| members.len())
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    // Determines the ID of the clan with the largest number of members by iterating over the `clans` HashMap.
    // The `iter` method creates an iterator over the map entries, where each entry is a tuple containing a clan ID and a vector of member names.
    // Subsequent chained operations (not fully shown here) will typically involve finding the entry with the maximum vector length, representing the largest clan.
    // The method is structured to return the ID of the largest clan as `Some(String)`. However, if the `clans` HashMap is empty or if no single clan is the largest due to a tie, `None` is returned.
    // This `None` case is handled implicitly by the nature of the iterator and chained operations, where an empty iterator or inability to determine a unique maximum would lead to `None`.
    pub fn get_largest_clan_id(&self) -> Option<String> {
        self.clans.iter()
            .max_by_key(|(_, members)| members.len())
            .map(|(id, _)| id.clone())
    }

    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        self.clans.entry(clan_id.to_string())
            .or_insert_with(Vec::new)
            .push(crab_name.to_string());
    }
}
