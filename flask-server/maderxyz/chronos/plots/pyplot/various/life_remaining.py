from datetime import datetime as dt

from matplotlib import pyplot as plt
from matplotlib.collections import PatchCollection
from matplotlib.patches import Rectangle

from maderxyz.chronos.utils import apply_pyplot_darkmode


def main(life_expectancy_in_years=80):

    width = life_expectancy_in_years
    height = 52
    fig, ax = plt.subplots(figsize=(width/2, height/2))

    ax.set_frame_on(False)

    patches, colors = [], []
    for h in range(1, height + 1):
        for w in range(1, width + 1):
            if w < dt.now().year - 1998:
                colors.append('white')
            elif w == dt.now().year - 1998:
                if h < dt.now().isocalendar()[1]:
                    colors.append('white')
                else:
                    colors.append('green')
            else:
                colors.append('green')

            rectangle = Rectangle((w, h), width=0.75,
                                  height=0.75, linewidth=10)
            patches.append(rectangle)

    plt.xlim(1, width + 1)
    plt.ylim(1, height + 1)
    plt.xticks(size=40)
    plt.yticks([], [])

    p = PatchCollection(patches, alpha=1, edgecolor='black',
                        linewidth=3, facecolor=colors)
    ax.add_collection(p)

    apply_pyplot_darkmode()

    plt.savefig('./static/media/pyplots/life_remaining.png')
    plt.savefig('./static/media/pyplots/life_remaining.pdf')
    plt.close()
