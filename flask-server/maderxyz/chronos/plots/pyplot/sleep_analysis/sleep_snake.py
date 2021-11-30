from datetime import datetime as dt
from datetime import timedelta as td
import math

from bokeh.models import ColumnDataSource
from colour import Color
import matplotlib.pyplot as plt
import numpy as np

from maderxyz.chronos import load_raw
from maderxyz.config import PLOT_WIDTH, PLOT_HEIGHT


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
    fig, ax = plt.subplots(
        figsize=(12, 7)
    )

    # plt.gca().spines['right'].set_visible(False)
    # for item in [fig, plt.gca()]:
    #     item.patch.set_visible(False)
    # ax.spines['bottom'].set_color('black')
    # ax.spines['top'].set_color('black')
    # ax.spines['left'].set_color('black')
    # ax.spines['right'].set_color('black')
    ax.set_facecolor('black')

    plt.style.use('dark_background')

    return fig


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

    gradient = list(Color('orange').range_to(Color('white'), 100))

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
        # if date < dt(2014, 1, 1):
        #     print(date)
        #     continue
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

    N = 50
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

        # if not count:
        #     count = 1
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
    # rect = Rect(
    #     x='dates', y='y', width='w', height='h',
    #     line_width=0, fill_color='colors', fill_alpha=1
    # )
    # fig.add_glyph(source, rect)

    # line = Line(x='dates', y='avg_to_bed_times')
    # fig.add_glyph(source, line)
    # # circ = Circle(x='dates', y='avg_to_bed_times', size=3)
    # # fig.add_glyph(source, circ)
    # line = Line(x='dates', y='avg_get_up_times')
    # fig.add_glyph(source, line)
    # # circ = Circle(x='dates', y='avg_get_up_times', size=3)
    # # fig.add_glyph(source, circ)

    # # plot select
    # select = figure(
    #     # background_fill_color='#333333'
    #     title="", plot_width=PLOT_WIDTH, plot_height=80,
    #     toolbar_location=None,  # tools="xwheel_pan",
    #     x_axis_type="datetime",  # y_axis_type=None,
    #     x_axis_location=None, y_axis_location=None,
    #     y_range=fig.y_range,
    # )
    # range_tool = RangeTool(x_range=fig.x_range)
    # select.add_tools(range_tool)

    # select.line(x=[dt(2013, 5, 1), dt.now()], y=[0, 0], color=None)
    # select.add_glyph(source, rect)
    # select.ygrid.grid_line_color = None
    # select.toolbar.active_multi = range_tool
    # select.yaxis.ticker = []

    dates = source.data['dates']
    to_bed_times = np.array(source.data['to_bed_times'])
    get_up_times = np.array(source.data['get_up_times'])
    sleep_durations = np.array(source.data['h'])
    avg_to_bed_times = np.array(source.data['avg_to_bed_times'])
    avg_get_up_times = np.array(source.data['avg_get_up_times'])

    get_up_times_int = [
        int(i.split(':')[0])+int(i.split(':')[1])/60. for i in get_up_times
    ]
    to_bed_times_int = [
        int(i.split(':')[0])+int(i.split(':')[1])/60. for i in to_bed_times
    ]
    to_bed_times_int = [
        i if i < 20 else i - 24 for i in to_bed_times_int
    ]

    plt.bar(
        dates, sleep_durations,
        width=0.8 * td(days=1), bottom=to_bed_times_int,
        color='gray', label='daily data'
    )
    plt.plot(
        dates[80:], avg_get_up_times[80:], color='orange'
    )
    plt.plot(
        dates[80:], avg_to_bed_times[80:], color='orange',
        label='50 day running average'
    )
    plt.xticks(
        [dt(i, 1, 1) for i in range(2014, dt.now().year+2)],
        [dt(i, 1, 1).strftime('%Y') for i in range(2014, dt.now().year+2)],
        color='gray'
    )
    # plt.grid(True, color='black')
    plt.yticks(
        range(-2, 12+1, 2), [
            '22:00', '00:00', '02:00', '04:00',
            '06:00', '08:00', '10:00', '12:00',
        ], color='gray'
    )
    plt.ylim(-3, 15)
    plt.xlim(dt(2013, 12, 15), dt(dt.now().year+1, 1, 1))
    plt.legend(loc='upper left')
    # plt.xlabel('date', color='white')
    # plt.ylabel('time of day', color='white')
    plt.text(
        dt(2014, 9, 1), -5.25,
        'sleep snek: times at which I went to bed & got up ' +
        'over the last 6 years',
        color='orange', fontsize=15
    )
    plt.text(
        dt(2012, 12, 1), -5.2,
        '/u/TheRealZoidberg',
        color='gray'
    )
    plt.text(
        dt(2012, 12, 1), 17,
        'data was gathered using the Sleep Cycle iOS app',
        color='white'
    )
    plt.text(
        dt(2012, 12, 1), 16.3,
        f'nr. of recorded nights: {len(dates)}',
        color='white'
    )
    plt.text(
        dt(2014, 1, 1), -3,
        r'$\leftarrow$ high school, starts each day at 07:45 $\rightarrow$',
        color='white',
    )
    plt.text(
        dt(2016, 4, 20), -1.7,
        r'$\uparrow$ final high school exams',
        color='white',
    )
    plt.text(
        dt(2016, 9, 20), -1,
        r'$\uparrow$ start of uni',
        color='white',
    )
    plt.text(
        dt(2016, 10, 15), -3,
        r'$\leftarrow$ 1st sem. parties $\rightarrow$',
        color='white',
    )
    # plt.text(
    #     dt(2018, 2, 1), -3,
    #     # r'$\uparrow$ watch a video on sleep',
    #     r'$\leftarrow$ focus more on uni $\rightarrow$',
    #     color='white',
    # )
    # plt.text(
    #     dt(2019, 3, 1), -3,
    #     r'$\leftarrow$ more parties $\rightarrow$',
    #     color='white',
    # )
    plt.text(
        dt(2019, 11, 10), -2.4,
        r'$\uparrow$ start of B.Sc. thesis (home office)',
        color='white',
    )
    plt.text(
        dt(2020, 3, 1), -1.7,
        r'$\uparrow$ start of corona lockdown',
        color='white',
    )

    def foo(num):
        bar = str(num).split('.')
        h = round(float(bar[0]))
        m = round(float('0.' + bar[1]) * 60)
        h = h if h > 9 else f'0{h}'
        m = m if m > 9 else f'0{m}'
        return f'{h}:{m}'

    plt.text(
        dt(2020, 5, 24), 17,
        f'avg. go-to-bed time: {foo(np.mean(to_bed_times_int))}',
        color='white',
    )
    plt.text(
        dt(2020, 7, 28), 16.3,
        f'avg. get-up time: {foo(np.mean(get_up_times_int))}',
        color='white',
    )
    plt.text(
        dt(2020, 8, 2), 15.6,
        f'avg. time in bed: {foo(np.mean(sleep_durations))}',
        color='white',
    )

    return fig


def main():
    source = setup_source()
    fig = plot(source)
    plt.savefig('test.png')
