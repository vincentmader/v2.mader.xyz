import csv
from datetime import datetime as dt
from datetime import timedelta as td
import os

import pandas as pd

import maderxyz.config as config
from maderxyz import chronos
from maderxyz.chronos.io import save_to_database


def main():

    # loop over export files on disk
    PATH_TO_SLEEP_CYCLE_EXPORTS = os.path.join(
        config.PATH_TO_RAW_DATA, 'sleep_cycle'
    )
    for export in os.listdir(PATH_TO_SLEEP_CYCLE_EXPORTS):
        if not export.startswith('sleepdata'):
            continue

        # open export csv file
        path_to_export = os.path.join(PATH_TO_SLEEP_CYCLE_EXPORTS, export)
        with open(path_to_export) as fp:
            reader = csv.reader(fp, delimiter=';')
            # skip empty lines
            next(reader, None)
            for row in reader:
                if not row:
                    continue  # TODO: this and next() both needed?

                # pull general datapoint info
                start = dt.strptime(row[0], '%Y-%m-%d %H:%M:%S')
                end = dt.strptime(row[1], '%Y-%m-%d %H:%M:%S')
                date = dt(start.year, start.month, start.day)
                index = (date - config.START_DATE).days

                # handle older export files (different format)
                if export == 'sleepdata_5.csv':

                    regularity = None
                    sleep_quality = float(row[2][:-1]) / 100
                    wake_up_mood = row[4]
                    if wake_up_mood:
                        wake_up_mood = {
                            ':)': 1, ':|': 0, ':(': -1
                        }[wake_up_mood]
                    else:
                        wake_up_mood = None
                    # heart_rate = None
                    # steps = None
                    # alarm_mode = None
                    air_pressure = None
                    city = None
                    movements_per_hour = None
                    hours_in_bed = (end - start).seconds / 3600.
                    time_asleep = None
                    time_before_sleep = None
                    window_start = None
                    window_stop = None
                    did_snore = None
                    snore_time = None
                    weather_temperature = None
                    weather_type = None
                    sleep_notes = row[5].split(
                        ':') if row[5] and row[5] != '0' else []

                # handle newer export file
                elif export in ['sleepdata.csv', 'sleepdata_6S.csv']:

                    sleep_quality = float(row[2][:-1]) / 100
                    regularity = float(row[3][:-1]) / 100
                    wake_up_mood = row[4]
                    if wake_up_mood:
                        wake_up_mood = {
                            'Good': 1, 'OK': 0, 'Bad': -1
                        }[wake_up_mood]
                    else:
                        wake_up_mood = None
                    # heart_rate = None
                    # steps
                    # /alarm mode
                    air_pressure = row[8]
                    city = row[9]
                    movements_per_hour = row[10]
                    hours_in_bed = (end - start).seconds / 3600.
                    # time asleep
                    # time before sleep
                    # window start
                    # window Stop
                    # did snore
                    # snore time
                    # weather_temperature =
                    # weather type
                    sleep_notes = row[20].split(
                        ':') if row[20] and row[20] != '0' else []

                else:
                    pass  # TODO: continue?

                # create document & save to database
                document = {
                    'date': date,
                    'start': start,
                    'end': end,
                    'sleep quality': sleep_quality,
                    'sleep regularity': regularity,
                    'wake-up mood': wake_up_mood,
                    # /heart rate
                    # /steps
                    # /alarm mode
                    'air pressure': air_pressure,
                    'city': city,
                    'movements per hour': movements_per_hour,
                    'hours in bed': hours_in_bed,
                    # time asleep
                    # time before sleep
                    # window start
                    # window Stop
                    # did snore
                    # snore time
                    # weather temperature
                    # weather type
                    'sleep notes': sleep_notes,
                }
                path = ['raw data', 'sleep cycle']
                save_to_database(document, path=path)
