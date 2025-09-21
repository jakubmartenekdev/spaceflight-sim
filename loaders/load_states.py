import astropy
from skyfield.api import Time, load
from skyfield import relativity

import json

from argparse import ArgumentParser

# print(type(M_earth.value))
parser = ArgumentParser()
parser.add_argument(
    "-u",
    "--utc_date",
    type=int,
    nargs=3,
    metavar=("YEAR", "MONTH", "DAY"),
    help="Constructs utc date",
)

args = parser.parse_args()
# print(args)

ts = load.timescale()
time: Time
if args.utc_date is not None:
    year, month, day = args.utc_date
    time = ts.utc(year, month, day)
else:
    time = ts.now()

print(time.utc_datetime())
relativity.deflectors = ["jupiter", "saturn"]

planets_eph = load("de421.bsp")
# mars_moons_eph = load("mar099s.bsp")
mars_moons_eph = load("mar_excerpt.bsp")

# print(
#     f"Mars moons eph range: {mars_moons_eph.start_time.utc_datetime()} to {mars_moons_eph.end_time.utc_datetime()}"
# )
# print(eph)
# print(type(0.3829))
sun = planets_eph["sun"]

planets = [
    "mercury",
    "venus",
    "earth",
    "moon",
    "mars",
    "jupiter barycenter",
    "saturn barycenter",
    "uranus barycenter",
    "neptune barycenter",
]
mars_moons = [
    "phobos",
    "deimos",
]


def serialize_data(bodies, eph, data):
    for body in bodies:
        apparent = sun.at(time).observe(eph[body]).apparent()  # type: ignore

        data[body] = {
            "dist": list(apparent.xyz.km),
            "velocity": list(apparent.velocity.km_per_s),
            # "mass": M_earth.value,
        }


with open("states.json", "w") as f:
    data = {}
    data["time"] = time.utc_datetime().isoformat()

    serialize_data(planets, planets_eph, data)
    serialize_data(mars_moons, mars_moons_eph, data)
    json.dump(data, f, indent=2)
