import numpy as np
from skimage import io
import matplotlib.pyplot as plt
# from skyfield.api import load

# # Create a timescale and ask the current time.
# ts = load.timescale()
# t = ts.now()

# # Load the JPL ephemeris DE421 (covers 1900-2050).
# planets = load('de421.bsp')
# earth, mars = planets['earth'], planets['mars']

# # What's the position of Mars, viewed from Earth?
# astrometric = earth.at(t).observe(mars)
# ra, dec, distance = astrometric.radec()

# print(ra)
# print(dec)
# print(distance)

a = np.zeros(3);
b = np.array([10, 20])
c = [10, 20]
print(type(b))
print(type(c))

# b.shape

print(b.shape)

np.random.seed(0)

n1 = np.array([[0], [0], [1], [2]])
print(n1.shape)

img = io.imread("img.png")

plt.imshow(img[::-1])
plt.savefig('output.png', bbox_inches='tight')
plt.close()

print(img.shape)
# plt.imshow(img)
