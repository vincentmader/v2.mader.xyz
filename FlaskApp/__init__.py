import os
import sys

import flask
from flask import Flask, render_template, send_from_directory

import chronos


app = Flask(__name__)


# @app.route('/')
# def chart():
#     return render_template('chart.html', **{'ayy': 'ayy'})

@app.route('/')
def index():
    #return sys.executable
    return render_template('index.html')


@app.route('/tatooine/')
def tatooine():
    return render_template('tatooine.html')


@app.route('/orbit/')
def orbit():
    return render_template('orbit.html')


@app.route('/balls/')
def balls():
    return render_template('balls.html')


@app.route('/boltzmann/')
def boltzmann():
    return render_template('boltzmann.html')


@app.route('/spaceship/')
def spaceship():
    return render_template('spaceship.html')


@app.route('/solar/')
def solar():
    return render_template('solar.html')


@app.route('/lorentz/')
def lorentz():
    return render_template('lorentz.html')


@app.route('/chart_test/')
def chart_test():
    return render_template('chart_test.html')


# @app.route('/bokeh/')
# def bokeh():
#    data = chronos.stats.time_series.chars_written_in_daily_log()
#
#    script, div = chronos.plots.basic.heatmap(data)
#    kwargs = {'script': script, 'div': div}
#
#    return render_template('bokeh.html', **kwargs)


@app.route('/heatmap/')
def heatmap():

    values = range(64)
    NR_OF_ROWS = 6
    REST = len(values) % NR_OF_ROWS
    NR_OF_COLS = int((len(values) - REST) / NR_OF_ROWS)
    kwargs = {
        # 'NR_OF_ROWS': NR_OF_ROWS,
        'NR_OF_COLS': NR_OF_COLS,
        'REST': REST,
        'values': values,
        'values_len': len(values),
    }

    return render_template('heatmap.html', **kwargs)


@app.route('/pendulum/')
def pendulum():
    return render_template('pendulum.html')


if __name__ == '__main__':
    app.run()

    # the code code should make flask server auto-restart on change of
    # templates, but doesn't apparently?
    # restart is need only for html & css? not for py & js I believe

    # extra_dirs = ['./', ]
    # extra_files = extra_dirs[:]
    # for extra_dir in extra_dirs:
    #     for dirname, dirs, files in os.walk(extra_dir):
    #         for filename in files:
    #             filename = os.path.join(dirname, filename)
    #             if os.path.isfile(filename):
    #                 extra_files.append(filename)
    # app.run(extra_files=extra_files)
