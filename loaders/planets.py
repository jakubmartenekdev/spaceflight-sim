from astropy.constants import M_earth, R_earth
from dataclasses import dataclass, field


@dataclass
class PlanetaryData:
    name: str
    radi: float
    mass: float
    active: bool


# todo: masses are incorrect
planets = [
    PlanetaryData(name="mercury", radi=0.3829 * R_earth, mass=0.0553 * M_earth),
    PlanetaryData(name="venus", radi=0.9499 * R_earth, mass=0.815 * M_earth),
    PlanetaryData(name="earth", radi=1 * R_earth, mass=1 * M_earth),
    PlanetaryData(name="mars", radi=0.5320 * R_earth, mass=0.107 * M_earth),
    PlanetaryData(name="jupiter", radi=10.97 * R_earth, mass=317.83 * M_earth),
    PlanetaryData(name="saturn", radi=9.140 * R_earth, mass=95.162 * M_earth),
    PlanetaryData(name="uranus", radi=3.981 * R_earth, mass=14.536 * M_earth),
    PlanetaryData(name="neptune", radi=3.865 * R_earth, mass=17.147 * M_earth),
    PlanetaryData(name="phobos", radi=11.1, mass=10.659 * M_earth),
]
