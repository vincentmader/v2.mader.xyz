from datetime import datetime as dt
import json
import os

import pandas as pd

from maderxyz import config


USER_NAME = 'Vincent'
PATH_TO_TAKEOUTS = os.path.join(
    config.PATH_TO_RAW_DATA, 'google_takeout', USER_NAME
)


def main():

    measurement_times, latitudes, longitudes = [], [], []
    velocities, headings, altitudes, verticalAccuracies = [], [], [], []

    for takeout in os.listdir(PATH_TO_TAKEOUTS):
        if not os.path.isdir(os.path.join(PATH_TO_TAKEOUTS, takeout)):
            continue

        path_to_location_hist = os.path.join(
            PATH_TO_TAKEOUTS, takeout, 'Location History/Location History.json'
        )
        with open(path_to_location_hist) as fp:
            content = json.load(fp)['locations']

        for datapoint in content:
            measurement_time = dt.fromtimestamp(
                int(datapoint['timestampMs']) / 1000
            )
            latitude = datapoint['latitudeE7'] / 1e7
            longitude = datapoint['longitudeE7'] / 1e7
            if 'velocity' in datapoint.keys():
                velocity = datapoint['velocity']
            else:
                velocity = None
            if 'heading' in datapoint.keys():
                heading = datapoint['heading']
            else:
                heading = None
            if 'altitude' in datapoint.keys():
                altitude = datapoint['altitude']
            else:
                altitude = None
            if 'verticalAccuracy' in datapoint.keys():
                verticalAccuracy = datapoint['verticalAccuracy']
            else:
                verticalAccuracy = None

            measurement_times.append(measurement_time)
            latitudes.append(latitude)
            longitudes.append(longitude)
            velocities.append(velocity)
            headings.append(heading)
            altitudes.append(altitudes)
            verticalAccuracies.append(verticalAccuracy)

        # not executing the code below saves ~20% execution time,
        # but the units might be a bit shitty...

        # for datapoint in content:
        #     timestamp = float(datapoint['timestampMs']) / 1000.
        #     latitude = datapoint['latitudeE7'] / 1e7
        #     longitude = datapoint['longitudeE7'] / 1e7
        #     accuracy = datapoint['accuracy'] / 1e7

    # TODO

    # return {
    #     'name': 'location history',
    #     'rules': ['high resolution'],
    #     'data': pd.DataFrame({
    #         'measurement time': measurement_times,
    #         'latitude': latitudes, 'longitude': longitudes,
    #         'velocity': velocities, 'heading': headings,
    #         'altitude': altitudes, 'verticalAccuracy': verticalAccuracies
    #     })
    # }
