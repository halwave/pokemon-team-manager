//! Core models and domain rules.

pub const TEAM_SIZE: usize = 6;

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TeamError {
    Full,
    InvalidSlotId(usize),
    EmptySlot(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Team {
    slots: Vec<Option<Pokemon>>,
}

impl Team {
    pub fn new() -> Self {
        Self {
            slots: vec![None; TEAM_SIZE],
        }
    }

    pub fn from_slots(slots: [Option<Pokemon>; TEAM_SIZE]) -> Self {
        Self {
            slots: Vec::from(slots),
        }
    }

    pub fn add_pokemon(&mut self, pokemon: Pokemon) -> Result<usize, TeamError> {
        for index in 0..TEAM_SIZE {
            if self.slots[index].is_none() {
                self.slots[index] = Some(pokemon);
                return Ok(index + 1);
            }
        }

        Err(TeamError::Full)
    }

    pub fn remove_pokemon(&mut self, slot_id: usize) -> Result<Pokemon, TeamError> {
        let index = Self::slot_index(slot_id)?;

        match self.slots[index].take() {
            Some(pokemon) => Ok(pokemon),
            None => Err(TeamError::EmptySlot(slot_id)),
        }
    }

    pub fn replace_pokemon(&mut self, slot_id: usize, pokemon: Pokemon) -> Result<(), TeamError> {
        let index = Self::slot_index(slot_id)?;

        match self.slots[index].as_mut() {
            Some(existing_pokemon) => {
                *existing_pokemon = pokemon;
                Ok(())
            }
            None => Err(TeamError::EmptySlot(slot_id)),
        }
    }

    pub fn slot(&self, slot_id: usize) -> Option<&Pokemon> {
        if slot_id == 0 || slot_id > TEAM_SIZE {
            return None;
        }

        self.slots[slot_id - 1].as_ref()
    }

    pub fn slots(&self) -> &[Option<Pokemon>] {
        &self.slots
    }

    fn slot_index(slot_id: usize) -> Result<usize, TeamError> {
        if slot_id == 0 || slot_id > TEAM_SIZE {
            return Err(TeamError::InvalidSlotId(slot_id));
        }

        Ok(slot_id - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_duplicate_pokemon_and_caps_team_at_six() {
        let mut team = Team::new();
        let pikachu = Pokemon {
            id: 25,
            name: "Pikachu".to_owned(),
        };

        let first_slot = team.add_pokemon(pikachu.clone()).unwrap();
        let second_slot = team.add_pokemon(pikachu.clone()).unwrap();
        assert_ne!(first_slot, second_slot);

        for _ in 2..TEAM_SIZE {
            team.add_pokemon(pikachu.clone()).unwrap();
        }

        assert_eq!(team.add_pokemon(pikachu), Err(TeamError::Full));
    }

    #[test]
    fn removes_and_replaces_by_slot_id() {
        let mut team = Team::new();
        let slot_id = team
            .add_pokemon(Pokemon {
                id: 1,
                name: "Bulbasaur".to_owned(),
            })
            .unwrap();
        let replacement = Pokemon {
            id: 4,
            name: "Charmander".to_owned(),
        };

        assert_eq!(team.replace_pokemon(slot_id, replacement.clone()), Ok(()));
        assert_eq!(team.slot(slot_id), Some(&replacement));
        assert_eq!(team.remove_pokemon(slot_id), Ok(replacement));
        assert_eq!(
            team.remove_pokemon(slot_id),
            Err(TeamError::EmptySlot(slot_id))
        );
    }

    #[test]
    fn rejects_slot_ids_outside_the_team() {
        let mut team = Team::new();

        assert_eq!(team.remove_pokemon(0), Err(TeamError::InvalidSlotId(0)));
        assert_eq!(
            team.replace_pokemon(
                7,
                Pokemon {
                    id: 4,
                    name: "Charmander".to_owned(),
                }
            ),
            Err(TeamError::InvalidSlotId(7))
        );
    }
}
