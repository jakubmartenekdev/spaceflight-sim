from skyfield.api import Time, load
from skyfield import relativity

import json

from argparse import ArgumentParser

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

relativity.deflectors = ["jupiter", "saturn"]

eph = load("de421.bsp")
sun = eph["sun"]

bodies = [
    "mercury",
    "venus",
    "earth",
    # "moon"
    "sun",
    "mars",
    "jupiter barycenter",
    "saturn barycenter",
    "uranus barycenter",
    "neptune barycenter",
]

with open("states.json", "w") as f:
    data = {}
    data["time"] = time.utc_datetime().isoformat()
    for body in bodies:
        d = sun.at(time).observe(eph[body]).apparent()  # type: ignore

        data[body] = {
            "dist": list(d.xyz.km),
            "velocity": list(d.velocity.km_per_s),
        }

    json.dump(data, f, indent=2)
