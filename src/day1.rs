// stolen from https://github.com/mitsuhiko/aoc19

type Mass = u64;
type Fuel = u64;

#[derive(Debug, Copy, Clone)]
struct Module {
    pub mass: Mass,
}

fn mass_to_fuel(mass: Mass) -> Mass {
    (mass / 3).saturating_sub(2)
}

impl Module {
    pub fn launch_fuel(self) -> Fuel {
        mass_to_fuel(self.mass)
    }

    pub fn total_fuel_incl_additional(self) -> Fuel {
        std::iter::successors(Some(self.launch_fuel()), |&fuel| Some(mass_to_fuel(fuel)))
            .take_while(|&fuel| fuel != 0)
            .sum::<Fuel>()
    }
}

pub fn total_basic_fuel_of_modules(content: &'static str) -> Fuel {
    content
        .lines()
        .filter_map(|x| x.parse().ok())
        .map(|mass| Module { mass }.launch_fuel())
        .sum::<Fuel>()
}

pub fn total_fuel_of_modules_incl_additional(content: &'static str) -> Fuel {
    content
        .lines()
        .filter_map(|x| x.parse().ok())
        .map(|mass| Module { mass }.total_fuel_incl_additional())
        .sum::<Fuel>()
}
