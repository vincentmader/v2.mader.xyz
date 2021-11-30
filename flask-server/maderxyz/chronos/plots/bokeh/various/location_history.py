from datetime import datetime as dt

from bokeh.embed import components
from bokeh.models import ColumnDataSource, GMapOptions, Circle
from bokeh.plotting import gmap, figure
from bokeh.tile_providers import get_provider

from maderxyz.chronos import load_raw
from maderxyz.config import PLOT_WIDTH, PLOT_HEIGHT


GOOGLE_API_KEY = 'AIzaSyDs5S84EPeL7bk_FA4rdtnI35VgKXsbDyY'


def setup_source():

    latitude, longitude = [], []

    location_history = load_raw.google_takeout.location_history()
    for i in location_history:
        latitude.append(i['latitudeE7'] / 1e7)
        longitude.append(i['longitudeE7'] / 1e7)

    source = ColumnDataSource(data=dict(
        lat=latitude, lon=longitude
    ))

    return source


def main():

    map_options = GMapOptions(
        lat=49.2081, lng=10.7394,
        map_type='roadmap', zoom=11
    )
    fig = gmap(
        GOOGLE_API_KEY, map_options, title='',
    )

    # tile_provider = get_provider('CARTODBPOSITRON')
    # fig = figure(
    #     x_axis_type='mercator', y_axis_type='mercator',
    # )
    # fig.add_tile(tile_provider)

    fig.width = PLOT_WIDTH
    fig.height = 700

    source = setup_source()

    circle = Circle(
        x='lon', y='lat',
        size=15, fill_color='blue', fill_alpha=0.8
    )
    fig.add_glyph(source, circle)

    return components(fig)
