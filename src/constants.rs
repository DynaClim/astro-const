//! Physical constants

/// Number pi
pub use core::f64::consts::PI;
/// Number 2 * [`PI`].
pub const TWO_PI: f64 = 2. * PI;
/// Number of seconds in a year (s)
pub const SECONDS_IN_YEAR: f64 = 3.15576e7;
/// Number of seconds in a day (s)
pub const SECONDS_IN_DAY: f64 = 86400.;

// ------------- ------------- -------------
// IAU Division I Working Group
// Numerical Standards for Fundamental Astronomy Astronomical Constants
// https:///iau-a3.gitlab.io/NSFA/NSFA_cbe.html

/// [Astronomical unit (m)](https:///www.iau.org/static/resolutions/IAU2012_English.pdf)
pub const AU: f64 = 1.495_978_707_00e11;
/// [Gravitational constant (USI) (m3kg−1s−2)](https:///iau-a3.gitlab.io/NSFA/NSFA_cbe.html#ConstGrav2009)
pub const GRAVITATIONAL: f64 = 6.674_28e-11;
/// [Equatorial radius of the Earth (m)](https:///iau-a3.gitlab.io/NSFA/NSFA_cbe.html#EarthRadius2009)
pub const EARTH_RADIUS: f64 = 6.378_136_6e6;
/// [Solar mass parameter](https:///iau-a3.gitlab.io/NSFA/NSFA_cbe.html#GMS2012)
pub const GRAVITATIONAL_SOLAR_MASS: f64 = 1.327_124_420_99e20;
/// Solar mass (kg)
/// [`GRAVITATIONAL_SOLAR_MASS`] / [`GRAVITATIONAL`]
pub const SOLAR_MASS: f64 = GRAVITATIONAL_SOLAR_MASS / GRAVITATIONAL;
/// [Geocentric gravitational constant, GME (TT-compatible)](https:///iau-a3.gitlab.io/NSFA/NSFA_cbe.html#GME2009)
pub const GRAVITATIONAL_EARTH_MASS: f64 = 3.986_004_415e14;
/// Earth mass (kg)
/// [`GRAVITATIONAL_EARTH_MASS`] / [`GRAVITATIONAL`]
pub const EARTH_MASS: f64 = GRAVITATIONAL_EARTH_MASS / GRAVITATIONAL;
/// [Ratio of the mass of the moon over the mass of the Earth](https:///iau-a3.gitlab.io/NSFA/NSFA_cbe.html#MMME2009)
pub const MOON_MASS_OVER_EARTH_MASS: f64 = 1.230_003_71e-2;
/// Moon mass (kg)
/// [`MOON_MASS_OVER_EARTH_MASS`] * [`EARTH_MASS`]
pub const MOON_MASS: f64 = MOON_MASS_OVER_EARTH_MASS * EARTH_MASS;
/// [Ratio mass sun over mass Jupiter](https:///iau-a3.gitlab.io/NSFA/NSFA_cbe.html#MSMJ2009)
pub const SOLAR_MASS_OVER_JUPITER_MASS: f64 = 1.047_348_644e3;
/// Mass of Jupiter (kg)
/// [`SOLAR_MASS`] / [`SOLAR_MASS_OVER_JUPITER_MASS`]
pub const JUPITER_MASS: f64 = SOLAR_MASS / SOLAR_MASS_OVER_JUPITER_MASS;

// ------------- ------------- -------------
// From Nasa fact sheets

/// Moon volumetric mean radius (m)
/// [NASA fact sheet](https:///nssdc.gsfc.nasa.gov/planetary/factsheet/moonfact.html)
pub const MOON_RADIUS: f64 = 1737.4E3;
/// Solar Volumetric mean radius (m)
/// [NASA fact sheet](https:///nssdc.gsfc.nasa.gov/planetary/factsheet/sunfact.html)
pub const SOLAR_RADIUS: f64 = 6.957e8;
/// Solar luminosity (10^24 J/s-1)
/// [NASA fact sheet](https:///nssdc.gsfc.nasa.gov/planetary/factsheet/sunfact.html)
pub const SOLAR_LUMINOSITY: f64 = 382.8;
/// Solar Sidereal rotation period (hrs)
/// [NASA fact sheet](https:///nssdc.gsfc.nasa.gov/planetary/factsheet/sunfact.html)
pub const SOLAR_PERIOD_HR: f64 = 609.12;
/// Solar rotation period (days)
/// [`SOLAR_PERIOD_HR`] / 24
pub const SOLAR_PERIOD: f64 = SOLAR_PERIOD_HR / 24.;
/// Solar angular velocity (rad.s-1)
/// [`TWO_PI`] / ([`SOLAR_PERIOD`] * [`SECONDS_IN_DAY`])
pub const SOLAR_ANGULAR_VELOCITY: f64 = TWO_PI / (SOLAR_PERIOD * SECONDS_IN_DAY);

// ------------- ------------- -------------
// Sourced from NIST

/// Perfect Gas Constant
/// [2022 CODATA recommended value](https:///physics.nist.gov/cgi-bin/cuu/Value?r|search_for=physchem_in!)
pub const GAS_CONSTANT: f64 = 8.314_462_618;
/// Mass of a proton (kg)
/// [2022 CODATA recommended value](https:///physics.nist.gov/cgi-bin/cuu/Value?mp|search_for=proton)
pub const PROTON_MASS: f64 = 1.672_621_925_95e-27;

// ------------- ------------- -------------
// Externally sourced

/// Boltzmann constant (J.K-1) from CODATA 2017
/// [Table 3 of Newell et al. 2018](https:///iopscience.iop.org/article/10.1088/1681-7575/aa950a/pdf)
pub const BOLTZMANN_CONST: f64 = 1.380_649e-23;
/// Solar Rossby number
/// [Ardestani et al. 2017](https:///doi.org/10.1093/mnras/stx2039)
pub const ROSSBY_SUN: f64 = 1.113;
/// Rossby number at saturation
/// [Ardestani et al. 2017](https:///doi.org/10.1093/mnras/stx2039)
pub const ROSSBY_SATURATION: f64 = 0.09;
/// Magnetic field at the surface of the Sun (T)
/// [Vidotto et al. 2014](https:///doi.org/10.1093/mnras/stu728)
pub const SOLAR_SURFACE_MAGNETIC_FIELD: f64 = 2e-4;
/// Temperature of the solar corona (K)
/// [Ahuir et al. 2021, Eq 20.](https:///doi.org/10.1051/0004-6361/202040173)
pub const SOLAR_CORONA_TEMPERATURE: f64 = 1.5e06;
/// Density of the solar corona (m-3)
/// [Ahuir et al. 2021, Eq 21.](https:///doi.org/10.1051/0004-6361/202040173)
pub const SOLAR_CORONA_DENSITY: f64 = 7.25e13;
/// Solar mass-loss rate
/// [Ahuir et al. 2020, Eq 70.](https:///doi.org/10.1051/0004-6361/201936974)
pub const SOLAR_MASS_LOSS_RATE: f64 = 2.3e-14;
/// Magnetic permeability of vacuum (USI)
/// [Wikipedia](https:///en.wikipedia.org/wiki/Vacuum_permeability)
pub const MAGNETIC_PERMEABILITY_OF_VACUUM: f64 = 4. * PI * 1e-07;
