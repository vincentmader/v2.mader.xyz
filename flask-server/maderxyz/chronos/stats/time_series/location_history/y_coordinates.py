import chronos
from chronos import load_raw


def main():

    location_history = load_raw.google_takeout.location_history()

    timestamps, y_coordinates = [], []

    for i in location_history:

        timestamp = i['timestampMs'] / 1e3
        y_coordinate = i['latitudeE7'] / 1e7

        timestamps.append(timestamp)
        y_coordinates.append(y_coordinate)

    return timestamps, y_coordinates
