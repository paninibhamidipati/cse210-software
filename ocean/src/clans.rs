
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
    pub fn get_clan_count(&self) -> usize {
        self.clans.len()
    }
    

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