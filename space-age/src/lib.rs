// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SEC: f64 = 31557600 as f64;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    fn period() -> f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 as f64 / Self::period()
    }
}

macro_rules! planet {
    ($i:ident, $year_as_fraction_of_earths: tt) => {
        pub struct $i;
        impl Planet for $i {
            fn period() -> f64 {
               $year_as_fraction_of_earths * EARTH_YEAR_IN_SEC
            }
        }
    }
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);