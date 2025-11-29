import csv
import numpy as np

import matplotlib.animation as animation
import matplotlib.pyplot as plt

# This file produces animations and is not part of the main code

points_record = []

with open("./data/points_record.csv", "r") as csv_file:
    reader = csv.reader(csv_file)
    for row in reader:
        n = len(row) // 3
        row = list(map(float, row))
        points_record.append(np.asarray(row).reshape(n, 3))

time_n = len(points_record)
size = np.amax(points_record[0])

fig, ax = plt.subplots(figsize=(15, 15), subplot_kw={"projection": "3d"})
plt.tight_layout()

fig.patch.set_facecolor("black")
ax.set_facecolor("black")
ax.grid(False)

# Make background black
ax.xaxis.set_pane_color((0.0, 0.0, 0.0, 0.0))
ax.yaxis.set_pane_color((0.0, 0.0, 0.0, 0.0))
ax.zaxis.set_pane_color((0.0, 0.0, 0.0, 0.0))

ax.set_xlim3d([-size, size])
ax.set_ylim3d([-size, size])
ax.set_zlim3d([-size, size])

ax.set_xticks([])
ax.set_yticks([])
ax.set_zticks([])

ax.view_init(elev=90)
ax.set_aspect("equal")

points = points_record[0]
(graph,) = ax.plot(
    points[:, 0],
    points[:, 1],
    points[:, 2],
    c="w",
    markersize=0.1,
    marker="o",
    linestyle="",
)


def animate(frame):
    points = points_record[frame]
    graph.set_data(points[:, 0], points[:, 1])
    graph.set_3d_properties(points[:, 2])


anim = animation.FuncAnimation(fig, animate, frames=time_n, interval=10)

writer = animation.FFMpegWriter(
    fps=30,
    metadata={"title": f"{len(points)} particles, {len(points_record)} time steps"},
)

anim.save("galaxy.mp4", writer=writer)

plt.show()
