import sys

import matplotlib.pyplot as plt
from matplotlib.patches import Polygon, Rectangle

# -------- Parse arguments --------
if len(sys.argv) != 6:
    print("Usage: python plot.py points.txt x1 y1 x2 y2")
    sys.exit(1)

filename = sys.argv[1]
x1, y1, x2, y2 = map(float, sys.argv[2:])

# -------- Read polygon points --------
points = []
with open(filename) as f:
    for line in f:
        if line.strip():
            x, y = map(float, line.split(","))
            points.append((x, y))

# -------- Plot --------
fig, ax = plt.subplots(figsize=(6, 6))

# Polygon
polygon = Polygon(points, closed=True, alpha=0.3)
ax.add_patch(polygon)

# Polygon outline
px, py = zip(*points)
ax.plot(px + (px[0],), py + (py[0],), marker="o", markersize=3)

# Rectangle (from opposite corners)
rx = min(x1, x2)
ry = min(y1, y2)
width = abs(x2 - x1)
height = abs(y2 - y1)

rect = Rectangle((rx, ry), width, height, fill=False, edgecolor="red", linewidth=2)

ax.add_patch(rect)

# Formatting
ax.set_aspect("equal")
ax.grid(True)
ax.set_xlabel("X")
ax.set_ylabel("Y")
ax.set_title("Polygon with Rectangle")

plt.show()
