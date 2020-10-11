import random

from datetime import datetime as dt
from datetime import timedelta as td


def main(nr_of_datapoints=400):

    data = []
    for i in range(nr_of_datapoints):
        data.append({
            'timestamp': (dt.now() - td(days=i)).timestamp(),
            'value': random.randint(0, 10)
        })

    return data
