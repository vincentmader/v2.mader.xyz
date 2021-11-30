from datetime import datetime as dt
from datetime import timedelta as td
import csv
import os

import pandas as pd
from tqdm import tqdm

import maderxyz.config as config
from maderxyz import chronos
from maderxyz.chronos.io import save_to_database


PATH_TO_QS_EXPORT_FILE = os.path.join(
    config.PATH_TO_RAW_DATA, 'qs_export/Heart Rate.csv'
)


def main():

    # loop over entries in export file
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
            heart_rate = float(row[2])

            # save to database
            document = {
                'date': date,
                'start': start,
                'end': end,
                'heart rate': heart_rate
            }
            path = ['raw data', 'qs export', 'heart rate']
            save_to_database(document, path=path)
