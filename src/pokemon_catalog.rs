use crate::domain::Pokemon;

const MAX_POKEMON_ID: u32 = 151;

#[derive(Clone, Debug)]
pub struct PokemonCatalog {
    entries: Vec<Pokemon>,
}

impl PokemonCatalog {
    pub fn load() -> Result<Self, serde_json::Error> {
        let entries = serde_json::from_str(include_str!("../data/pokemon.json"))?;
        Ok(Self { entries })
    }

    pub fn find(&self, id: u32) -> Option<&Pokemon> {
        if id == 0 || id > MAX_POKEMON_ID {
            return None;
        }

        self.entries.iter().find(|pokemon| pokemon.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_contains_ids_one_through_151() {
        let catalog = PokemonCatalog::load().unwrap();

        assert_eq!(catalog.entries.len(), 151);
        for id in 1..=MAX_POKEMON_ID {
            assert!(catalog.find(id).is_some());
        }
        assert!(catalog.find(0).is_none());
        assert!(catalog.find(152).is_none());
    }
}
