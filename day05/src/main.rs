#[derive(Debug, Default)]
struct Input {
    seeds: Vec<usize>,
    seed_to_soils: Vec<std::ops::Range<usize>>,
    soil_to_ferts: Vec<std::ops::Range<usize>>,
    fert_to_waters: Vec<std::ops::Range<usize>>,
    water_to_lights: Vec<std::ops::Range<usize>>,
    light_to_temps: Vec<std::ops::Range<usize>>,
    temp_to_humidities: Vec<std::ops::Range<usize>>,
    humidity_to_locations: Vec<std::ops::Range<usize>>,
}

fn parse_input() -> Input {
    let file_contents = std::fs::read_to_string("./input.txt").unwrap();

    let mut input: Input = Default::default();

    input
}

fn main() {
    let input = parse_input();
    println!("input: {input:#?}");
}
