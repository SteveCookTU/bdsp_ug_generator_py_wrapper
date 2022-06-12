use bdsp_ug_generator::{run_results, Advance, Pokemon, RoomType, Version, Filter};
use bdsp_ug_generator::xorshift::XorShift;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
struct AdvancePy {
    #[pyo3(get)]
    advance: u32,
    #[pyo3(get)]
    regular_pokemon: Vec<PokemonPy>,
    #[pyo3(get)]
    rare_pokemon: Option<PokemonPy>,
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
    ability: u8,
    #[pyo3(get)]
    gender: u8,
    #[pyo3(get)]
    nature: u8,
    #[pyo3(get)]
    item: u16,
    #[pyo3(get)]
    egg_move: Option<u16>,
}

#[pyclass]
#[derive(Clone)]
struct FilterPy {
    pub shiny: bool,
    pub species: Option<u16>,
    pub min_ivs: [u8; 6],
    pub max_ivs: [u8; 6],
    pub ability: Option<u8>,
    pub nature: Option<Vec<u8>>,
    pub item: Option<u16>,
    pub egg_move: Option<u16>,
    pub gender: Option<u8>,
}

#[pymethods]
impl FilterPy {
    #[new]
    fn new(shiny: bool, species: Option<u16>, min_ivs: [u8; 6], max_ivs: [u8; 6], ability: Option<u8>, nature: Option<Vec<u8>>, item: Option<u16>, egg_move: Option<u16>, gender: Option<u8>) -> Self {
        Self {
            shiny,
            species,
            min_ivs,
            max_ivs,
            ability,
            nature,
            item,
            egg_move,
            gender
        }
    }
}

impl Into<Filter> for FilterPy {
    fn into(self) -> Filter {
        Filter {
            shiny: self.shiny,
            species: self.species,
            min_ivs: self.min_ivs,
            max_ivs: self.max_ivs,
            ability: self.ability,
            nature: self.nature,
            item: self.item,
            egg_move: self.egg_move,
            gender: self.gender
        }
    }
}

impl From<Advance> for AdvancePy {
    fn from(a: Advance) -> Self {
        Self {
            advance: a.advance,
            regular_pokemon: a
                .regular_pokemon
                .into_iter()
                .map(|p| p.into())
                .collect::<Vec<PokemonPy>>(),
            rare_pokemon: a.rare_pokemon.map(|p| p.into()),
        }
    }
}

impl From<Pokemon> for PokemonPy {
    fn from(p: Pokemon) -> Self {
        Self {
            species: p.species,
            ec: p.ec,
            pid: p.pid,
            shiny: p.shiny,
            ivs: p.ivs,
            ability: p.ability,
            gender: p.gender,
            nature: p.nature,
            item: p.item,
            egg_move: p.egg_move,
        }
    }
}

#[pyfunction]
fn generate_results(
    advances: u32,
    state: [u32; 4],
    version: u8,
    story_flag: u8,
    room: u8,
    filter: FilterPy,
    diglett: bool,
) -> PyResult<Vec<AdvancePy>> {
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
        _ => RoomType::TyphloCavern,
    };

    let rng = XorShift::from_state(state);

    let results = run_results(
        advances, rng, version, story_flag, room, filter.into(), diglett,
    );

    Ok(results
        .into_iter()
        .map(|a| a.into())
        .collect::<Vec<AdvancePy>>())
}

#[pymodule]
fn bdsp_ug_generator_py(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<AdvancePy>()?;
    m.add_class::<PokemonPy>()?;
    m.add_class::<FilterPy>()?;
    m.add_function(wrap_pyfunction!(generate_results, m)?)?;
    Ok(())
}
