from datetime import datetime as dt
from datetime import timedelta as td

import pandas as pd

from FlaskApp.chronos import load_raw


def main():

    sleep_qualities, timestamps = [], []

    sleep_history = load_raw.sleep_cycle.sleep_history()
    for datapoint in sleep_history:
        # get timestamp representing midnight of the day I went to bed
        foo = dt.fromtimestamp(datapoint['end_timestamp'])
        date = dt(foo.year, foo.month, foo.day) - td(days=1)
        timestamps.append(date)
        # get time at which I got out of bed
        sleep_quality = dt.fromtimestamp(datapoint['end_timestamp']).time()
        sleep_qualities.append(sleep_quality)

    df = pd.DataFrame.from_dict({
        'timestamp': timestamps,
        'sleep quality': sleep_qualities,
    })
    out = {'resolution': 'daily', 'data': df}

    return out
