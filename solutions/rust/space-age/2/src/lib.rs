#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self {
            earth_years: s as f64 / 31_557_600.0,
        }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::ORBITAL_PERIOD
    }
}

macro_rules! planets {
    ($($name:ident => $period:expr),+ $(,)?) => {
        $(
            pub struct $name;

            impl Planet for $name {
                const ORBITAL_PERIOD: f64 = $period;
            }
        )+
    };
}

planets!(
    Mercury => 0.2408467,
    Venus => 0.61519724,
    Earth => 1.0,
    Mars => 1.8808158,
    Jupiter => 11.862615,
    Saturn => 29.447498,
    Uranus => 84.016846,
    Neptune => 164.81415,
);
