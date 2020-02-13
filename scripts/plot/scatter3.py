#!/usr/bin/python3


from mpl_toolkits.mplot3d import Axes3D

import pandas
import matplotlib.pyplot as plt
import sys


def quit_figure(event):
    if event.key == 'escape':
        plt.close(event.canvas.figure)


cid = plt.gcf().canvas.mpl_connect('key_press_event', quit_figure)


#   Settings
TITLE_LABEL = "Title"
X_AXIS_LABEL = "X"
Y_AXIS_LABEL = "Y"
Z_AXIS_LABEL = "Y"

COLS_SMALL = ["r.",
              "m.",
              "b.",
              "c.",
              "g.",
              "y.",
              "k."]
COLS_LARGE = ["#FF0000",
              "#800000",
              "#FFFF00",
              "#808000",
              "#00FF00",
              "#008000",
              "#00FFFF",
              "#008080",
              "#0000FF",
              "#000080",
              "#FF00FF",
              "#800080",
              "#000000",
              "#808080",
              "#C0C0C0"]


#   Main
if len(sys.argv) != 2:
    print("Incorrect arguments: <filename>")
    quit()

filename = sys.argv[1]

# colnames = ['t', 'a', 'b', 'c', 'd', 'e', 'f', 'g']
data = pandas.read_csv(filename, header=None)

num_cols = len(data.columns) - 1
print("Total columns: ", num_cols)

if num_cols <= len(COLS_SMALL):
    cols = COLS_SMALL
else:
    cols = COLS_LARGE

xs = data[0]
ys = data[1]
zs = data[2]

fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')
ax.scatter(xs, ys, zs)

ax.set_xlabel(X_AXIS_LABEL)
ax.set_ylabel(Y_AXIS_LABEL)
ax.set_zlabel(Z_AXIS_LABEL)

plt.show()
