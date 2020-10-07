import random

from datetime import datetime as dt
from datetime import timedelta as td


def main():
    data = []
    for i in range(400):
        data.append({
            'timestamp': (dt.now() - td(days=i)).timestamp(),
            'value': random.randint(0, 10)
        })
    return data
