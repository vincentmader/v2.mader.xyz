from datetime import datetime as dt

from bokeh.embed import components
from bokeh.models import ColumnDataSource
from bokeh.plotting import figure

from FlaskApp.chronos import load_raw
from FlaskApp.config import PLOT_HEIGHT, PLOT_WIDTH


def setup_source():

    date, altitude = [], []

    location_history = load_raw.google_takeout.location_history()
    for i in location_history:
        if not 'altitude' in i.keys():
            continue
        date.append(dt.fromtimestamp(
            int(i['timestampMs']) / 1000.)
        )
        altitude.append(i['altitude'])

    source = ColumnDataSource(dict(
        date=date, altitude=altitude
    ))

    return source


def main():

    fig = figure(
        plot_width=PLOT_WIDTH, plot_height=PLOT_HEIGHT,
        title="altitude",
        # x_range=(dt.now() - td(days=220), dt.now()),
        # y_range=(0, NR_OF_ROWS + 1),
        x_axis_type="datetime", y_axis_type=None,
        # x_axis_location="above",
        # tooltips=[
        #     ('date', '@date'), ('value', '@value'),
        #     ('content', '@content')
        # ],
        # toolbar_location=None,
        # match_aspect=True,
        tools='zoom_in, zoom_out, pan, box_zoom, reset, hover'
        # background_fill_color='#333333'
    )

    source = setup_source()

    fig.line(x='date', y='altitude', source=source)

    script, div = components(fig)
    return script, div
