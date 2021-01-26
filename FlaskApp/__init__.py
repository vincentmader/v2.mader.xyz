import os
import sys

import flask
from flask import Flask, render_template, send_from_directory, request

import chronos
import config
from config import PATH_TO_STATIC


# initialize app
app = Flask(__name__)


# helper methods (move!)
# =============================================
# def html_for_navigator_grid():

#     html = '<table>'

#     pages = ['/chronos/test']
#     for p in pages:

# html += """
#     <a class="stealthy_link" href="/chronos/test">
#         <img src="{{ url_for('static', filename='media/thumbnails/boltzmann.png') }}" width="200px" height="200px">
#     </a>
# """

# html += '</table>'
# return html

# views
# =============================================================================

# index


@app.route('/')
def index():

    nav_grid_items = [
        {'id': 'pyplots', 'link': '/chronos/test_pyplots'},
        {'id': 'test', 'link': '/chronos/test'},
        {'id': 'boltzmann', 'link': '/comp_phys/boltzmann'},
        {'id': 'orbit'},
        {'id': 'solar'},
        {'id': 'tatooine'},
        {'id': 'solar'},
        {'id': 'tatooine'},
    ]
    enumerated_nav_grid_items = enumerate(nav_grid_items)

    return render_template(
        'index.html',
        enumerated_nav_grid_items=enumerated_nav_grid_items,
        nr_of_cols=4
    )


@app.route('/comp_phys/boltzmann')
def boltzmann2():
    return render_template('comp_phys/layout1.html')


@app.route('/chronos/test_pyplots')
def pyplots():

    images = []

    path_to_pyplots = os.path.join(PATH_TO_STATIC, 'media/pyplots')
    for filename in os.listdir(path_to_pyplots):
        image = {
            'filepath': os.path.join('media/pyplots', filename)
        }
        images.append(image)
    print(images)

    return render_template(
        'chronos/test_pyplots.html',
        images=images
    )


@app.route('/chronos/test')
def test():

    script, div = chronos.plots.bokeh.google_takeout.gmap()
    # script, div = chronos.plots.bokeh.google_takeout.bar_plot_search_hits(
    #     'youtube')

    return render_template('chronos/test.html', script=script, div=div)


# old
# =============================================================================

# main sections


@app.route('/minipages/<page_name>')
def minipages(page_name):
    return page_name


# debugging
@app.route('/exec/')
def python_executable():
    return sys.executable


# minipages
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


@app.route('/solar_binary/')
def solar_binary():
    return render_template('solar_binary.html')


@app.route('/lorentz/')
def lorentz():
    return render_template('lorentz.html')


@app.route('/chart_test/')
def chart_test():
    return render_template('chart_test.html')


# bokeh
@app.route('/bokeh/')
def bokeh():
    scripts, divs = [], []
    kwargs = {'scripts': scripts, 'divs': divs}
    return render_template('bokeh.html', **kwargs)


@app.route('/bokeh/', methods=['POST'])
def bokeh_post():
    plot_name = request.form['text']

    scripts, divs = [], []
    if plot_name == 'sleep':
        script, div = chronos.plots.bokeh.sleep_analysis.sleep_snake()
        scripts.append(script)
        divs.append(div)
    elif plot_name == 'map':
        script, div = chronos.plots.bokeh.various.location_history()
        scripts.append(script)
        divs.append(div)
    elif plot_name == 'grades':
        script, div = chronos.plots.bokeh.various.grades()
        scripts.append(script)
        divs.append(div)
    elif plot_name == 'heatmap':
        # data = chronos.stats.sample_data()
        data = chronos.stats.time_series.daily_log.chars_written_in_daily_log()
        script, div = chronos.plots.bokeh.basic.heatmap(data, 'daily')
        scripts.append(script)
        divs.append(div)
    elif plot_name == 'altitude':
        script, div = chronos.plots.bokeh.various.altitude_history()
        scripts.append(script)
        divs.append(div)

    kwargs = {'scripts': scripts, 'divs': divs}
    return render_template('bokeh.html', **kwargs)


@app.route('/messages/')
def messages():
    scripts, divs = [], []
    kwargs = {'scripts': scripts, 'divs': divs}
    return render_template('bokeh.html', **kwargs)


@app.route('/messages/', methods=['POST'])
def messages_post():
    scripts, divs = [], []
    query = request.form['text']

    script, div = chronos.plots.bokeh.various.messages(query)
    scripts.append(script)
    divs.append(div)

    kwargs = {'scripts': scripts, 'divs': divs}
    return render_template('bokeh.html', **kwargs)


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


# @app.route('/gtakeout/', methods=['POST', 'GET'])
# def gtakeout():

#     query = request.form['text']
#     scripts, divs = [], []
#     script, div = chronos.plots.bokeh.google_takeout.bar_plot_search_hits(
#         query)
#     scripts.append(script)
#     divs.append(div)

#     kwargs = {'scripts': scripts, 'divs': divs}
#     return render_template('bokeh.html', **kwargs)


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
