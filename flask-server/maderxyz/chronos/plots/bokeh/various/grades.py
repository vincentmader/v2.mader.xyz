from datetime import datetime as dt

from bokeh.embed import components
from bokeh.plotting import figure
from bokeh.models import Circle, ColumnDataSource

from FlaskApp.chronos import stats
from FlaskApp.config import PLOT_WIDTH, PLOT_HEIGHT


def setup_source():
    semesters, grades = stats.time_series.grades.avg_grades()

    source = ColumnDataSource(dict(semesters=semesters, grades=grades))
    return source


def main():

    source = setup_source()

    fig = figure(
        title='grades',
        plot_width=PLOT_WIDTH, plot_height=PLOT_HEIGHT,
        x_range=(dt(2006, 1, 1), dt.now()),
        y_range=(1, 4),
        x_axis_location="above",
        x_axis_type="datetime",  # y_axis_type=None,
        toolbar_location=None,
        match_aspect=True,
    )
    fig.yaxis.ticker = [1, 2, 3, 4]
    fig.yaxis.major_label_overrides = {1: '1', 2: '2', 3: '3', 4: '4'}

    line = Line(x='semesters', y='grades')
    fig.add_glyph(source, line)

    script, div = components(fig)
    return script, div
