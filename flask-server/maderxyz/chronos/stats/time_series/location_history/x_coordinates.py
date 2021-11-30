import chronos
from chronos import load_raw


def main():

    location_history = load_raw.google_takeout.location_history()

    timestamps, x_coordinates = [], []

    for i in location_history:

        timestamp = i['timestampMs'] / 1e3
        x_coordinate = i['longitudeE7'] / 1e7

        timestamps.append(timestamp)
        x_coordinates.append(x_coordinate)

    return timestamps, x_coordinates
