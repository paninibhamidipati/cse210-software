use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    crabs: Vec<Crab>,
    clan_system: ClanSystem,
    // TODO: Declare the fields of the Beach struct here.
}

impl Beach {
    pub fn new() -> Beach {
        Beach { 
            crabs: Vec::new(),
            clan_system: ClanSystem::new(),
        }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        self.crabs.iter().max_by_key(|crab| crab.speed())
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        self.crabs.iter().filter(|crab| crab.name() == name).collect()
    }
    

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        let parent1 = &self.crabs[i];
        let parent2 = &self.crabs[j];
        let new_color = Color::cross(parent1.color(), parent2.color());
        let new_diet = Diet::random_diet();
        let new_crab = Crab::new(name, 1, new_color, new_diet);
        self.crabs.push(new_crab);
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clan_system
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        self.clan_system.add_member2(clan_id, crab_name);
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let clan1_members = self.clan_system.get_clan_member_names(id1);
        let clan2_members = self.clan_system.get_clan_member_names(id2);

        // Calculate average speed for each clan
        let avg_speed_clan1 = self.calculate_average_speed(&clan1_members);
        let avg_speed_clan2 = self.calculate_average_speed(&clan2_members);

        // Decide the winner
        match (avg_speed_clan1, avg_speed_clan2) {
            (Some(speed1), Some(speed2)) => {
                if speed1 > speed2 {
                    Ok(Some(id1.to_string()))
                } else if speed2 > speed1 {
                    Ok(Some(id2.to_string()))
                } else {
                    Ok(None)
                }
            },
            _ => Err("Invalid clan IDs".to_string()),
        }
    }

    fn calculate_average_speed(&self, member_names: &[String]) -> Option<f32> {
        let total_speed: f32 = member_names.iter()
            .filter_map(|name| self.crabs.iter().find(|crab| crab.name() == name))
            .map(|crab| crab.speed() as f32)
            .sum();

        if !member_names.is_empty() {
            Some(total_speed / member_names.len() as f32)
        } else {
            None
        }
    }
}

impl ClanSystem {
    pub fn add_member2(&mut self, clan_id: &str, crab_name: &str) {
        self.clans.entry(clan_id.to_string())
            .or_insert_with(Vec::new)
            .push(crab_name.to_string());
    }

    // Other existing methods...
}
