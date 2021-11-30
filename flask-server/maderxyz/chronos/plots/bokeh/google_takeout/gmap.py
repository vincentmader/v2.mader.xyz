from datetime import datetime as dt
import json
import math
import os
from pprint import pprint

from bokeh.embed import components
from bokeh.plotting import figure
from bokeh.tile_providers import CARTODBPOSITRON_RETINA, get_provider
from bokeh.themes import built_in_themes
from bokeh.io import curdoc
from matplotlib import pyplot as plt
import numpy as np

import maderxyz.config as config


def main():

    path_to_location_history = os.path.join(
        config.PATH_TO_RAW_DATA,
        'google_takeout/Vincent/Takeout_2020-07-06',
        'Location History/Location History.json'
    )
    with open(path_to_location_history) as fp:
        gps_data = json.load(fp)['locations']

    # set up
    N = len(gps_data)  # N = 14097
    m, n = 0, N-1

    # unix_timestamps, datetimes, latitudes, longitudes, accuracy = [], [], [], [], []
    # for i in range(m, n):
    #     unix_timestamps.append(float(gps_data[i]['timestampMs']) / 1e3)
    #     datetimes.append(
    #         dt.fromtimestamp(float(gps_data[i]['timestampMs']) / 1e3)
    #     )
    #     latitudes.append(gps_data[i]['latitudeE7'] / 1e7)
    #     longitudes.append(gps_data[i]['longitudeE7'] / 1e7)
    #     accuracy.append(gps_data[i]['accuracy'] / 1e7)
    # unix_timestamps = np.array(unix_timestamps)
    # datetimes = np.array(datetimes)
    # longitudes = np.array(longitudes)
    # latitudes = np.array(latitudes)
    # accuracy = np.array(accuracy)

    unix_timestamps = np.array(
        [float(gps_data[i]['timestampMs']) for i in range(N)][m:n]
    ) / 1e3
    datetimes = np.array(
        [dt.fromtimestamp(float(gps_data[i]['timestampMs']) / 1e3)
         for i in range(N)][m:n]
    )
    latitudes = np.array(
        [gps_data[i]['latitudeE7'] for i in range(m, n)]
    ) / 1e7
    longitudes = np.array(
        [gps_data[i]['longitudeE7'] for i in range(m, n)]
    ) / 1e7
    accuracy = np.array(
        [gps_data[i]['accuracy'] for i in range(m, n)]
    ) / 1e7

    # convert from angles lambda and phi to Web Mercator Coordinates
    L = 20037508.34

    def longitude_to_mercator_x(longitude):
        return longitude*(L/180)

    def latitude_to_mercator_y(latitude):
        return (np.log(np.tan((90 + latitude) * np.pi/360)) * 180/np.pi) * (L/180)

    web_mercator_x = longitude_to_mercator_x(longitudes)
    web_mercator_y = latitude_to_mercator_y(latitudes)

    # print('{}, {}'.format(web_mercator_x[i], web_mercator_y[i]))

    # for i in gps_data:
    #    if 'altitude' not in i.keys():
    #        print(i)
    # altitude = np.array([gps_data[i]['altitude'] for i in range(m, n)][m:n])
    # verticalAccuracy = np.array([gps_data[i]['verticalAccuracy'] for i in range(m, n)][m:n])
    # velocity = np.array([gps_data[i]['velocity'] for i in range(m, n)][m:n])

    start_date = dt.fromtimestamp(
        float(gps_data[m]['timestampMs']) / 1e3
    ).strftime('%Y-%m-%d')
    end_date = dt.fromtimestamp(
        float(gps_data[n]['timestampMs']) / 1e3
    ).strftime('%Y-%m-%d')

    # pprint(gps_data[0])
    # print(dt.fromtimestamp(float(gps_data[200]['timestampMs'])/1e3))

    # =============================================================================

    # i = 0
    # print('{}, {}'.format(longitudes[i], latitudes[i]))

    # =============================================================================

    TOOLTIPS = [
        ("index", "$index"),
        ("(x,y)", "($x, $y)"),
        ("accuracy", "@accuracy"),
        # ("date", "@datetimes"),
    ]

    fig = figure(
        height=550, width=900,
        # x_range=(longitude_to_mercator_x(-20), longitude_to_mercator_x(60)),
        # y_range=(latitude_to_mercator_y(35), latitude_to_mercator_y(55)),
        # x_range=(longitude_to_mercator_x(5), longitude_to_mercator_x(20)),
        x_range=(longitude_to_mercator_x(6), longitude_to_mercator_x(18)),
        y_range=(latitude_to_mercator_y(45), latitude_to_mercator_y(55)),
        x_axis_type="mercator", y_axis_type="mercator",
        title='Google Takeout: GPS Data from {} to {}'.format(
            start_date, end_date),
        x_axis_label='Longitude', y_axis_label='Latitude',
        # tooltips=TOOLTIPS
    )

    # fig.background_fill_color = "beige"
    # fig.background_fill_alpha = 0.5

    fig.add_tile(get_provider(CARTODBPOSITRON_RETINA))
    # fig.line(web_mercator_x, web_mercator_y)
    fig.circle(web_mercator_x, web_mercator_y, fill_color='white')

    curdoc().theme = 'dark_minimal'  # not working, due to map tile provider?

    script, div = components(fig)
    return script, div
