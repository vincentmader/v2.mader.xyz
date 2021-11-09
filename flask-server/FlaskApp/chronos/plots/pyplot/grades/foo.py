import os
from pprint import pprint

from datetime import timedelta as td
import matplotlib.pyplot as plt
import numpy as np

import FlaskApp.chronos
from FlaskApp.config import PATH_TO_STATIC
from FlaskApp.chronos.utils import apply_pyplot_darkmode


def main():

    semesters, grades = chronos.stats.time_series.grades.avg_grades()
    # grades = 17 - 3 * np.array(grades)

    plt.figure(figsize=(10, 5))

    plt.title('average grade per semester')
    # plt.xlabel('year')
    # plt.ylabel('avg. semester grade')
    plt.ylim(1, 3.5)

    # plot bars
    w = td(days=120)
    plt.bar(
        semesters[:4], grades[:4], width=w,
        color='green', label='JS-GS'
    )
    plt.bar(
        semesters[4:20], grades[4:20], width=w,
        color='red', label='HSS-G'
    )
    plt.bar(
        semesters[20:], grades[20:], width=w,
        color='blue', label='UHD'
    )

    apply_pyplot_darkmode()
    # save
    plt.savefig(os.path.join(PATH_TO_STATIC, 'media/pyplots/grades.pdf'))
    plt.savefig(os.path.join(PATH_TO_STATIC, 'media/pyplots/grades.png'))
    plt.close()
