from datetime import datetime as dt
from datetime import timedelta as td

import random


def main(N=500):

    sample_data = []

    for i in range(N):

        sample_data.append({
            'timestamp': (dt.now() - td(days=i)).timestamp(),
            'value': random.uniform(0, 1),
            'content': ''
        })

    return sample_data
