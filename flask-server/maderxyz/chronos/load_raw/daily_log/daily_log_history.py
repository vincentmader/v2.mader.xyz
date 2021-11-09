# TODO: rename file? org journal?

from datetime import datetime as dt
from datetime import timedelta as td
import os

from tqdm import tqdm

from FlaskApp import config
from FlaskApp.config import PATH_TO_DAILY_LOGS
from FlaskApp.chronos.io import save_to_database


def main():

    # loop over log files on disk
    daily_log_files = sorted(os.listdir(PATH_TO_DAILY_LOGS))
    for file_name in tqdm(daily_log_files):
        file_path = os.path.join(PATH_TO_DAILY_LOGS, file_name)
        if os.path.isdir(file_path):
            continue  # skip directories
        # get log content
        with open(file_path) as fp:
            content = fp.readlines()

        # get date for each file
        date = dt.strptime(file_name.split('.')[0], '%Y-%m-%d')

        # create document & save to database
        document = {
            'date': date,
            'content': content,
        }
        path = ['raw data', 'daily log']
        save_to_database(document, path=path)
