use bdsp_ug_generator::{Advance, Pokemon, RoomType, run_results, Version};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
struct AdvancePy {
    #[pyo3(get)]
    regular_pokemon: Vec<PokemonPy>,
    #[pyo3(get)]
    rare_pokemon: Option<PokemonPy>
}

#[pyclass]
#[derive(Clone)]
struct PokemonPy {
    #[pyo3(get)]
    species: u16,
    #[pyo3(get)]
    ec: u32,
    #[pyo3(get)]
    pid: u32,
    #[pyo3(get)]
    shiny: bool,
    #[pyo3(get)]
    ivs: [u8; 6],
    #[pyo3(get)]
    ability: u16,
    #[pyo3(get)]
    gender: u8,
    #[pyo3(get)]
    nature: u8,
    #[pyo3(get)]
    item: u16,
    #[pyo3(get)]
    egg_move: Option<u16>,
}

impl Into<AdvancePy> for Advance {
    fn into(self) -> AdvancePy {
        AdvancePy {
            regular_pokemon: self.regular_pokemon.into_iter().map(|p| p.into()).collect::<Vec<PokemonPy>>(),
            rare_pokemon: self.rare_pokemon.map(|p| p.into())
        }
    }
}

impl Into<PokemonPy> for Pokemon {
    fn into(self) -> PokemonPy {
        PokemonPy {
            species: self.species,
            ec: self.ec,
            pid: self.pid,
            shiny: self.shiny,
            ivs: self.ivs,
            ability: self.ability,
            gender: self.gender,
            nature: self.nature,
            item: self.item,
            egg_move: self.egg_move
        }
    }
}

#[pyfunction]
fn generate_results(advances: u32, s0: u32, s1: u32, s2: u32, s3: u32, version: u8, story_flag: u8, room: u8, shiny_only: bool) -> PyResult<Vec<AdvancePy>> {
    let version = match version {
        2 => Version::BD,
        _ => Version::SP,
    };

    let room = match room {
        2 => RoomType::SpaciousCave,
        3 => RoomType::GrasslandCave,
        4 => RoomType::FountainspringCave,
        5 => RoomType::RockyCave,
        6 => RoomType::VolcanicCave,
        7 => RoomType::SwampyCave,
        8 => RoomType::DazzlingCave,
        9 => RoomType::WhiteoutCave,
        10 => RoomType::IcyCave,
        11 => RoomType::RiverbankCave,
        12 => RoomType::SandsearCave,
        13 => RoomType::StillWaterCavern,
        14 => RoomType::SunlitCavern,
        15 => RoomType::BigBluffCavern,
        16 => RoomType::StargleamCavern,
        17 => RoomType::GlacialCavern,
        18 => RoomType::BogsunkCavern,
        _ => RoomType::TyphloCavern
    };

    let results = run_results(advances, s0, s1, s2, s3, version, story_flag, room, shiny_only);

    Ok(results.into_iter().map(|a| a.into()).collect::<Vec<AdvancePy>>())
}

#[pymodule]
fn bdsp_ug_generator_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_results, m)?)?;
    Ok(())
}