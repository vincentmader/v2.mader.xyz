import os
from pprint import pprint

from datetime import datetime as dt
from datetime import timedelta as td
import matplotlib.pyplot as plt
import numpy as np

import FlaskApp.chronos
from FlaskApp.chronos.stats.time_series import reshape
from FlaskApp.config import PATH_TO_STATIC
from FlaskApp.chronos.utils import apply_pyplot_darkmode


def main():

    # barrr = chronos.stats.avg_grades()
    # dates, playtimes = chronos.stats.time_series.spotify.total_playtime()
    # # print(dates)
    # dates, playtimes = reshape.into_monthly_time_series(dates, playtimes)
    # print(dates)

    # w = td(days=22)
    # plt.bar(dates, playtimes, width=w)

    # [dt(2018, 1, 1)+td(year=i) for i in range(2)]
    # )

    N = 25
    x, y = chronos.stats.time_series.spotify.total_playtime()
    x1, y1 = reshape.into_daily_time_series(x, y)
    x1, y1 = reshape.into_N_day_moving_average(x1, y1, N)
    x2, y2 = reshape.into_monthly_time_series(x, y)

    plt.figure(figsize=(10, 5))
    plt.title('spotify playtime in hours')

    plt.bar(x2, height=np.array(y2) / 3.6e6, width=td(days=24))
    plt.plot(x1[:-int(N/2)], np.array(y1[int(N/2):]) / 3.6e6, color='white')

    # plt.ylabel('playtime in hours')
    # plt.xlabel('time')
    plt.xticks([dt(2018, 1, 1), dt(2019, 1, 1), dt(2020, 1, 1)])
    plt.xlim(x[0], dt.now())

    apply_pyplot_darkmode()

    plt.savefig(os.path.join(PATH_TO_STATIC, 'media/pyplots/test_spotify.pdf'))
    plt.savefig(os.path.join(PATH_TO_STATIC, 'media/pyplots/test_spotify.png'))
    plt.close()

    # dates = sorted(barrr.keys())
    # # pprint(dates)
    # grades = [barrr[i] for i in dates]
    # # grades = 17 - 3 * np.array(grades)

    # w = td(days=170)
    # plt.bar(dates[:4], grades[:4], width=w, color='green', label='JS-GS')
    # plt.bar(dates[4:20], grades[4:20], width=w,
    #         color='red', label='HSS-G')
    # plt.bar(dates[20:], grades[20:], width=w, color='blue', label='UHD')

    # # plt.bar(dates, grades, width=w, color=colors)
    # # plt.plot(dates, grades)
    # # plt.scatter(dates, grades)
    # # plt.xlim(dates[0], dates[-1])
    # plt.xlabel('year')
    # plt.ylabel('avg. semester grade')
    # plt.ylim(1, 3.5)
    # plt.legend()
    # plt.savefig('grades.pdf')
