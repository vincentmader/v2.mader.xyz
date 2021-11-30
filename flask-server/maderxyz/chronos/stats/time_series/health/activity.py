from datetime import datetime as dt
from datetime import timedelta as td

import matplotlib.pyplot as plt  # TODO: remove
import pandas as pd

from maderxyz import config
from maderxyz.chronos.io import save_to_database


def prepare_dict(start_date, end_date):  # TODO: move elsewhere (empty time series)
    dic = {}
    while start_date < end_date:
        dic[start_date] = None
        start_date += td(days=1)
    return dic


def main():

    titles = [
        'active calories', 'cycling distance', 'distance',
        'heart rate', 'heart rate at rest',
        # 'heart rate variability',
        'steps', 'flights climbed',
    ]
    datasets_using_avg = [
        'heart rate', 'heart rate at rest', 'heart rate variability'
    ]

    # prepare empty time series
    # TODO: remove/rewrite, no Nones in db! carry no info
    foo = {}
    for title in titles:
        foo[title] = prepare_dict(config.START_DATE, config.END_DATE)

    # transform high-precision dataset into daily (sum or avg)
    for title in titles:
        collection = config.MDB['raw data']['qs export'][title]
        daily_counter = {}
        for entry in collection.find():

            date = dt.strptime(entry['date'].strftime('%Y-%m-%d'), '%Y-%m-%d')
            value = entry[title]

            if foo[title][date] is not None:
                foo[title][date] += value
                daily_counter[date] += 1.
            else:
                foo[title][date] = value
                daily_counter[date] = 1.

        # for some datasets, we need to take the average (e.g. heart rate)
        if title in datasets_using_avg:
            for date in foo[title].keys():
                if date in daily_counter.keys():
                    foo[title][date] /= daily_counter[date]

    # save to database
    for title in foo.keys():
        dates = sorted(foo[title].keys())

        # df = pd.DataFrame({
        #     'date': dates,
        #     title: [foo[title][d] for d in dates]
        # })
        # print(df.head())
        # if title == 'heart rate':
        #     plt.plot(df['date'], df[title])
        #     plt.show()

        document = {
            'dates': dates,
            'values': [foo[title][d] for d in dates],
            # 'resolution': ?
        }
        path = [
            'stats', 'time series', 'health', 'activity', title
        ]
        save_to_database(document, path=path)
