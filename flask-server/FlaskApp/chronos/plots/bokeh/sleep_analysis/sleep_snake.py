from datetime import datetime as dt
from datetime import timedelta as td
import math

import bokeh
from bokeh.embed import components
from bokeh.layouts import column
from bokeh.models import ColumnDataSource, DatetimeTickFormatter, RangeTool
from bokeh.models import Rect, Line, Circle
from bokeh.models import Plot
from bokeh.plotting import figure
from colour import Color
import numpy as np

from FlaskApp.chronos import load_raw
from FlaskApp.config import PLOT_WIDTH, PLOT_HEIGHT


def convert_time_str_to_float(time_str):
    f = int(time_str.split(':')[0])
    f += int(time_str.split(':')[1]) / 60.
    return f


def to_midnight(dt_obj):
    midnight = dt.strptime(dt_obj.strftime('%Y-%m-%d'), '%Y-%m-%d')
    return midnight


def to_time(dt_obj):
    time = dt_obj.hour + dt_obj.minute / 60. + dt_obj.second / 2600.
    return time


def setup_figure():
    fig = figure(
        title='sleep snek',
        plot_width=PLOT_WIDTH, plot_height=PLOT_HEIGHT,
        x_range=(dt.now() - td(days=5 * 365), dt.now()), y_range=(-2, 16),
        x_axis_location="above",
        x_axis_type="datetime",  # y_axis_type=None,
        tooltips=[
            ('date', '@date_strs'),
            ('sleep quality', '@sleep_quality'),
            ('went to bed at', '@to_bed_times'),
            ('got up at', '@get_up_times'),
            ('was in bed for', '@durations'),
            ('sleep notes', '@sleep_notes'),
            ('avg. bed time', '@avg_to_bed_times'),
            ('avg. get up time', '@avg_get_up_times'),
        ],
        toolbar_location=None,
        match_aspect=True,
        tools="xpan",
        # tools="zoom_in,zoom_out,xwheel_pan,box_zoom,reset,hover,save",
        # background_fill_color='#333333'
    )
    fig.xaxis.formatter = DatetimeTickFormatter(days='%Y-%m-%d')
    fig.yaxis.ticker = [-2, 0, 2, 4, 6, 8, 10, 12, 14, 16]
    fig.yaxis.major_label_overrides = {
        -2: '22:00', 0: '00:00', 2: '02:00', 4: '04:00', 6: '06:00',
        8: '08:00', 10: '10:00', 12: '12:00', 14: '14:00', 16: '16:00',
    }
    return fig


# def get_running_average(iterable, idx, N=14):

#     if not iterable:
#         return None

#     avg = 0
#     count = 1
#     for jdx in range(int(-N/2), int(N/2-1)):
#         if idx <= N:
#             return None

#         try:
#             foo = iterable[-jdx]
#             if foo > 20:
#                 foo -= 24.
#             avg += foo

#             count += 1
#         except IndexError:
#             break

#     avg /= count
#     # if avg < 0:
#     #     avg += 24

#     return avg


def get_running_average(iterable, idx, N=14):

    if not iterable:
        return None

    avg = 0
    count = 1
    for jdx in range(int(-N/2), int(N/2-1)):
        if idx <= N:
            return None

        try:
            foo = iterable[-jdx]
            if foo > 20:
                foo -= 24.
            avg += foo

            count += 1
        except IndexError:
            break

    avg /= count
    # if avg < 0:
    #     avg += 24

    return avg


def setup_source():

    gradient = list(Color('red').range_to(Color('green'), 100))

    # load data
    sleep_history = load_raw.sleep_cycle.sleep_history()
    # daily_log_history = load_raw.daily_log.daily_log_history()

    dates, date_strs = [], []
    to_bed_times, get_up_times = [], []
    y, w, h = [], [], []
    durations = []
    sleep_notes, sleep_quality = [], []
    colors = []

    # daily_log_dates = []
    # daily_logs = []

    # for idx, i in daily_log_history:
    #     daily_log_dates.append(i['date'])
    #     daily_logs.append(i['daily_log'])

    for idx, i in enumerate(sleep_history):
        date = to_midnight(
            dt.fromtimestamp(i['start_timestamp'])
        )
        if date == dt(2017, 7, 16):
            continue
        to_bed_time = dt.fromtimestamp(i['start_timestamp'])
        get_up_time = dt.fromtimestamp(i['end_timestamp'])
        duration = get_up_time - to_bed_time
        sn = i['notes']
        sq = i['quality']
        c = gradient[math.floor((sq * (len(gradient)-1)))].get_hex()

        # daily_log_date = date.strftime('%Y-%m-%d')
        # if daily_log_date in daily_log_dates:
        #     date_idx = daily_log_dates.index(daily_log_date)
        #     daily_log = daily_log_history[date_idx]
        # else:
        #     daily_log = ''

        def append():
            dates.append(date)
            date_strs.append(date.strftime('%A, %d. %b %Y'))

            to_bed_times.append(to_bed_time.strftime('%H:%M'))
            get_up_times.append(get_up_time.strftime('%H:%S'))
            durations.append(
                f'{duration.seconds // 3600} h {(duration.seconds %3600) // 60} min'
            )
            sleep_notes.append(', '.join(sn))
            sleep_quality.append(f'{int(sq * 100)} %')
            colors.append(c)

            yi = to_time(to_bed_time) + duration.seconds / 3600 / 2
            if to_time(to_bed_time) > 20:
                yi -= 24
            y.append(yi)
            w.append(td(hours=16))
            h.append((get_up_time - to_bed_time).seconds / 3600)

        append()

    N = 28

    avg_to_bed_times, avg_get_up_times = [], []
    for idx, i in enumerate(sleep_history):

        avg_to_bed_time, avg_get_up_time, count = 0, 0, 0
        for jdx in range(int(idx-N/2), int(idx+N/2)):
            try:
                to_bed_time = convert_time_str_to_float(to_bed_times[jdx])
                if to_bed_time > 20:
                    to_bed_time -= 24.
                get_up_time = convert_time_str_to_float(get_up_times[jdx])

                if to_bed_time and get_up_time:
                    avg_to_bed_time += to_bed_time
                    avg_get_up_time += get_up_time
                    count += 1
            except IndexError:
                pass

        avg_to_bed_time /= count
        avg_get_up_time /= count

        avg_get_up_times.append(avg_get_up_time)
        avg_to_bed_times.append(avg_to_bed_time)

    source = ColumnDataSource(dict(
        dates=dates, date_strs=date_strs,
        get_up_times=get_up_times, to_bed_times=to_bed_times,
        y=y, w=w, h=h, durations=durations,
        sleep_notes=sleep_notes, sleep_quality=sleep_quality,
        colors=colors,
        avg_get_up_times=avg_get_up_times[:-1],
        avg_to_bed_times=avg_to_bed_times[:-1]
    ))
    return source


def plot(source):
    fig = setup_figure()

    # plot data
    rect = Rect(
        x='dates', y='y', width='w', height='h',
        line_width=0, fill_color='colors', fill_alpha=1
    )
    fig.add_glyph(source, rect)

    line = Line(x='dates', y='avg_to_bed_times')
    fig.add_glyph(source, line)
    # circ = Circle(x='dates', y='avg_to_bed_times', size=3)
    # fig.add_glyph(source, circ)
    line = Line(x='dates', y='avg_get_up_times')
    fig.add_glyph(source, line)
    # circ = Circle(x='dates', y='avg_get_up_times', size=3)
    # fig.add_glyph(source, circ)

    # plot select
    select = figure(
        # background_fill_color='#333333'
        title="", plot_width=PLOT_WIDTH, plot_height=80,
        toolbar_location=None,  # tools="xwheel_pan",
        x_axis_type="datetime",  # y_axis_type=None,
        x_axis_location=None, y_axis_location=None,
        y_range=fig.y_range,
    )
    range_tool = RangeTool(x_range=fig.x_range)
    select.add_tools(range_tool)

    select.line(x=[dt(2013, 5, 1), dt.now()], y=[0, 0], color=None)
    select.add_glyph(source, rect)
    select.ygrid.grid_line_color = None
    select.toolbar.active_multi = range_tool
    select.yaxis.ticker = []

    return fig, select


def main():

    source = setup_source()
    fig, select = plot(source)

    # return as html components
    script, div = components(
        column(
            fig,
            select
        )
    )
    return script, div
