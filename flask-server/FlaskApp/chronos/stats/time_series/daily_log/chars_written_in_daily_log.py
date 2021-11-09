import os
import random
from datetime import datetime as dt
from datetime import timedelta as td

import FlaskApp.config


def main():

    PATH_TO_DAILY_LOGS = config.PATH_TO_DAILY_LOGS

    data = []
    # timestamps, values = [], []
    for log_file in os.listdir(PATH_TO_DAILY_LOGS):

        path_to_log_file = os.path.join(PATH_TO_DAILY_LOGS, log_file)
        if os.path.isdir(path_to_log_file):
            continue
        if log_file in ['.DS_Store', 'diary']:
            continue
        if 'conflicted' in log_file.split(' '):
            continue
        with open(path_to_log_file) as fp:
            content = fp.readlines()

        nr_of_chars = len(''.join(content))
        timestamp = dt.strptime(log_file.split('.')[0], '%Y-%m-%d').timestamp()

        # timestamps.append(timestamp)
        # values.append(nr_of_chars)

        data.append(dict(
            timestamp=timestamp,
            value=nr_of_chars, content='\n'.join(content)
        ))

    # data = dict(timestamp=timestamps, value=values)
    return data
