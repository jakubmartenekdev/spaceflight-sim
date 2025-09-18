from skyfield.api import load
from contextlib import closing

import json

ts = load.timescale()
t = ts.now()

eph = load("de421.bsp")
sun = eph["Sun"]
# print(planets)

bodies = [
    "Mercury",
    "Venus",
    "Earth",
    # "Moon"
    # "Sun",
    "Mars",
    "Jupiter BARYCENTER",
    "Saturn BARYCENTER",
    "Uranus BARYCENTER",
    "Neptune BARYCENTER",
]

with open("states.json", "w") as f:
    data = []
    for body in bodies:
        d = sun.at(t).observe(eph[body]).apparent()  # type: ignore

        data.append(
            {
                "name": body,
                "dist": list(d.xyz.km),
                "velocity": list(d.velocity.km_per_s),
            }
        )
        if body == "Earth":
            print(d.distance())

    json.dump(data, f, indent=2)

# What's the position of Mars, viewed from Earth?
# astrometric = earth.at(t).observe(moon)  # type: ignore
# ra, dec, distance = astrometric.radec()

# print(d.xyz.km)
# print(d.distance().km)
# print(d.velocity.km_per_s.distance)
# print(d.center)
# print(d.target)
