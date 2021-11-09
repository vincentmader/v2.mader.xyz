from datetime import datetime as dt
from datetime import timedelta as td
import csv
import os

import pandas as pd
from tqdm import tqdm

import FlaskApp.config as config
from FlaskApp import chronos
from FlaskApp.chronos.io import save_to_database


PATH_TO_QS_EXPORT_FILE = os.path.join(
    config.PATH_TO_RAW_DATA, 'qs_export/Steps.csv'
)


def main():

    # loop over lines in export file
    path_to_export = os.path.join(PATH_TO_QS_EXPORT_FILE)
    with open(path_to_export) as fp:
        reader = csv.reader(fp, delimiter=';')
        next(reader, None)
        for row in tqdm(reader):
            if not row:
                continue

            # get info from entry
            row = row[0].split(',')
            start = dt.strptime(row[0], '%d-%b-%Y %H:%M')
            end = dt.strptime(row[1], '%d-%b-%Y %H:%M')
            date = dt.strptime(start.strftime('%Y-%m-%d'), '%Y-%m-%d')
            index = (date - config.START_DATE).days
            steps = float(row[2])

            # save to database
            document = {
                'date': date,
                'start': start,
                'end': end,
                'steps': steps
            }
            path = ['raw data', 'qs export', 'steps']
            save_to_database(document, path=path)