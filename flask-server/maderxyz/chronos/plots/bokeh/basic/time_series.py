from datetime import datetime as dt
from datetime import timedelta as td
import random
import os

import bokeh
from bokeh.embed import components
# from bokeh.io import curdoc
from bokeh.layouts import column
# from bokeh.models import ColumnDataSource, DatetimeTickFormatter, RangeTool, Square
from bokeh.plotting import figure
# from bokeh.themes import built_in_themes
from bokeh.themes import built_in_themes
from bokeh.io import curdoc
import numpy as np

from FlaskApp.config import PLOT_WIDTH, PLOT_HEIGHT


def main(x, y):

    p = figure(
        plot_width=PLOT_WIDTH, plot_height=PLOT_HEIGHT,
        title="log",
        # x_range=(dt.now() - td(days=220), dt.now()),
        # y_range=(0, NR_OF_ROWS + 1),
        x_axis_type="datetime"  # , y_axis_type=None,
        # x_axis_location="above",
        # tooltips=[
        #     ('date', '@date'), ('value', '@value'),
        #     ('content', '@content')
        # ],
        # toolbar_location=None,
        # match_aspect=True,
        # tools='zoom_in,zoom_out,pan,box_zoom,reset,hover,save",
        # background_fill_color='#222222'
    )
    curdoc().theme = 'dark_minimal'  # not working, due to map tile provider?

    if type(x[0]) is int:
        datetimes = [dt.fromtimestamp(timestamp) for timestamp in x]
    else:
        datetimes = x

    width = .9 * np.mean(np.diff(datetimes))

    p.vbar(
        x, top=y,
        color='blue',
        width=width,
        # fill_alpha=.5,
        # fill_color='salmon',
        # line_alpha=.5,
        # line_color='green',
        # line_dash='dashed'
    )
    # p.line(x, y, color='blue')

    script, div = components(p)
    return script, div
