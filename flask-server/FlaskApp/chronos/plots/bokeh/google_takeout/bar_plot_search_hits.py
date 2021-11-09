from datetime import datetime as dt
from datetime import timedelta as td
import os
import sqlite3

from bokeh.plotting import figure
from bokeh.models import ColumnDataSource, Circle
from bokeh.embed import components
import matplotlib.pyplot as plt
import numpy as np


def get_monthly_counts(word):

    foo = 'youtube_search'

    path_to_sentdex_tut = '/home/vinc/code/tutorials/sentdex_google_takeout/'
    if foo == 'google_search':
        conn = sqlite3.connect(path_to_sentdex_tut + 'search_history.db')
    elif foo == 'youtube_search':
        conn = sqlite3.connect(path_to_sentdex_tut + 'yt_search_history.db')
    c = conn.cursor()
    c.execute(
        f'SELECT unix FROM words WHERE word = "{word}";'
    )
    timestamps = c.fetchall()

    monthly_counts = {}
    for t in timestamps:
        d = dt.fromtimestamp(int(t[0]))
        month = d.strftime('%Y-%m')
        if month not in monthly_counts.keys():
            monthly_counts[month] = 1
        else:
            monthly_counts[month] += 1

    return monthly_counts


def main(word):

    xs, ys = [], []
    monthly_counts = get_monthly_counts(word)
    for x, y in monthly_counts.items():
        xs.append(dt.strptime(x, '%Y-%m'))
        ys.append(y)

    fig = figure(
        title=word,
        # plot_width=PLOT_WIDTH, plot_height=PLOT_HEIGHT,
        x_range=(dt(2016, 1, 1), dt.now()),
        y_range=(1, 100),
        # x_axis_location="above",
        x_axis_type="datetime",  # y_axis_type=None,
        toolbar_location=None,
        # match_aspect=True,
    )

    source = ColumnDataSource(dict(xs=xs, ys=ys))
    # circ = Circle(x='xs', y='ys')
    # fig.add_glyph(source, circ)
    fig.vbar(x=xs, top=ys, width=td(days=25))

    print(xs)
    print(ys)

    script, div = components(fig)
    return script, div
