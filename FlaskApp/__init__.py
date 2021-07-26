import json
import os
import sys

from flask import Flask, render_template, request
import numpy as np

from .config import running_on_server
from .config import INDEX_NAVGRID_SECTIONS
from .config import PATH_TO_PROJECT, PATH_TO_STATIC
if not running_on_server:
    # from .chronos import plots, stats
    # TODO: make dependent on runtime environment (e.g. don't load db stuff on server (yet))
    from .config import MDB
    from .db_config import MDB_HIERARCHY, MDB_TS_CATEGORIES


# initialize app
app = Flask(__name__)


# view functions
# =============================================================================

# index
@app.route('/')
def index():
    props = {'nav_grid_sections': INDEX_NAVGRID_SECTIONS}
    return render_template('index.html', props=props)


# chronos
# -----------------------------------------------------------------------------
@app.route('/chronos/stats/<subdir>')
def chronos_stats_correlation_finder(subdir):

    if subdir == 'correlation_finder':
        template = 'chronos/correlation_finder.html'
        props = {
            # TODO: sort out unused props (here & @stats)
            'section_hierarchy': MDB_HIERARCHY['stats']['time series'],
            'categories': MDB_TS_CATEGORIES,
            'nr of categories': len(MDB_TS_CATEGORIES),  # TODO: remove
            'MDB': MDB['stats']['time series'],
            'correlations': MDB['stats']['correlations'],
            'zip': zip,  # TODO: move to funcs
            'funcs': {'len': len}
        }

    elif subdir == 'test':
        template = 'chronos/test.html'
        props = json.dumps({
            'mdb_hierarchy': MDB_HIERARCHY,
        })

    elif subdir == 'stats':
        template = 'chronos/stats.html'
        props = {
            'section_hierarchy': MDB_HIERARCHY['stats']['time series'],
            'categories': MDB_TS_CATEGORIES,
            'MDB': MDB['stats']['time series'],
            'correlations': MDB['stats']['correlations'],
            'zip': zip,
            'funcs': {'len': len}
        }

    return render_template(template, props=props)


# @app.route('/chronos/stats/<subdir>', methods=['POST'])
# def chronos_stats_post(subdir):
#     if subdir == 'correlation_finder':
#         return chronos_stats(subdir, )
#     textfield_1 = request.form['textfield_1']
#     return chronos_testing(subdir, textfield_1)


# comp phys
# -----------------------------------------------------------------------------
@app.route('/comp_phys/n_body/<subdir>')
def comp_phys_n_body(subdir):

    if subdir == '3body_fig8':
        # load output data
        path_to_output_file = os.path.join(
            PATH_TO_PROJECT,
            'comp_phys/n_body/out/3body_fig8/system_states.txt'
        )
        system_states = [list(i) for i in np.loadtxt(path_to_output_file)]
        # append dict containing simulation info to list
        simulations = [{
            'system_states': json.dumps(system_states[:1000]),
            'sim_id': '3body_fig8',
            'sim_idx': 0,
            'sim_title': 'stable 3-body orbit - figure eight'
        }]
        # return props directory
        template = 'comp_phys/n_body/n_body.html'
        props = {'simulations': simulations}
        return render_template(template, props=props)

    elif subdir in ['3body_moon', 'flowers', 'cloud', 'asteroids']:
        template = f'comp_phys/n_body/{subdir}.html'
        props = {}
        return render_template(template, props=props)


@app.route('/comp_phys/harmonical_oscillators/<subdir>')
def comp_phys_pendulum(subdir):

    if subdir == 'single_pendulum':
        template = 'comp_phys/oscillators/single_pendulum.html'
        props = {
            'title': 'pendulum',
        }
        return render_template(template, props=props)

    elif subdir == 'double_pendulum':
        path_to_output_file = os.path.join(
            PATH_TO_PROJECT, 'comp_phys/double_pendulum/out/ys.txt'
        )
        system_states = [list(i) for i in np.loadtxt(path_to_output_file)]
        template = 'comp_phys/oscillators/double_pendulum.html'
        props = {
            'title': 'double pendulum',
            'description': 'theory, latex...',  # TODO: remove? is this used anywhere?
            'ys': json.dumps(system_states),
        }
        return render_template(template, props=props)

    elif subdir == 'lissajous':
        template = 'comp_phys/oscillators/lissajous.html'
        props = {}
        return render_template(template, props=props)


@app.route('/comp_phys/cellular_automata/<subdir>')
def comp_phys_cellular_automata(subdir):

    if subdir in ['game_of_life', 'forest_fire', 'rock_paper_scissors']:
        template = f'comp_phys/cellular_automata/{subdir}.html'
        props = {}
        return render_template(template, props=props)


@app.route('/comp_phys/stat_phys/<subdir>')
def comp_phys_stat_phys(subdir):

    if subdir in ['brownian_motion', 'ising']:
        template = f'comp_phys/stat_phys/{subdir}.html'
        props = {}
        return render_template(template, props=props)

    elif subdir == 'thermal_motion':
        system_states = np.loadtxt(os.path.join(
            PATH_TO_PROJECT, 'comp_phys/gas_in_a_box/out/ys.txt'
        ))
        system_states = [list(i) for i in system_states]
        template = 'comp_phys/stat_phys/thermal_motion.html'
        props = {
            'ys': json.dumps(system_states),
        }
        return render_template(template, props=props)


@app.route('/comp_phys/monte_carlo/<subdir>')
def comp_phys_monte_carlo(subdir):

    if subdir == 'pi_darts':
        template = 'comp_phys/monte_carlo/pi_darts.html'
        props = {}
        return render_template(template, props=props)

    if subdir == 'ants':
        template = 'comp_phys/monte_carlo/ants.html'
        props = {}
        return render_template(template, props=props)

    if subdir == 'boids':
        template = 'comp_phys/monte_carlo/boids.html'
        props = {}
        return render_template(template, props=props)


@app.route('/comp_phys/electro_magnetism/<subdir>')
def comp_phys_electro_magnetism(subdir):
    if subdir == 'charge_interaction':
        template = 'comp_phys/electro_magnetism/charge_interaction.html'
        props = {}
        return render_template(template, props=props)


@app.route('/comp_phys/various/<subdir>')
def comp_phys_various(subdir):

    if subdir == 'quadtree':
        template = 'comp_phys/various/quadtree.html'
        props = {}
        return render_template(template, props=props)


# debugging
# =============================================================================


@app.route('/debugging/print_py_exec')
def print_python_executable():
    return sys.executable


# old
# =============================================================================


@app.route('/chronos/testing/<subdir>')
def chronos_testing(subdir, lolol=0):

    if subdir in ['chartjs']:
        return render_template(f'chronos/testing/{subdir}.html')

    if subdir == 'pyplot':
        images = []

        path_to_pyplots = os.path.join(PATH_TO_STATIC, 'media/pyplots')
        for filename in os.listdir(path_to_pyplots):
            image = {
                'filepath': os.path.join('media/pyplots', filename)
            }
            images.append(image)
        print(images)

        return render_template(
            'chronos/testing/pyplot.html',
            images=images
        )

    # if subdir == 'bokeh':
    #     plot_functions = [
    #         plots.bokeh.google_takeout.gmap,
    #     ]

    #     scripts, divs = [], []
    #     for f in plot_functions:
    #         script, div = f()
    #         scripts.append(script)
    #         divs.append(div)

    #     script, div = plots.bokeh.google_takeout.bar_plot_search_hits(
    #         'youtube')
    #     scripts.append(script)
    #     divs.append(div)

    #     x, y = stats.time_series.avg_grades()
    #     # x, y = np.array(list(foo.keys())), np.array(list(foo.values()))
    #     # x, y = stats.time_series.avg_grades()
    #     script, div = plots.bokeh.basic.time_series(x, y)
    #     scripts.append(script)
    #     divs.append(div)

    #     # x, y = stats.time_series.chars_written_in_daily_log()
    #     # x, y = np.array(list(foo.keys())), np.array(list(foo.values()))
    #     # x, y = stats.time_series.avg_grades()
    #     # script, div = plots.bokeh.basic.time_series(x, y)
    #     # scripts.append(script)
    #     # divs.append(div)

    #     template = 'chronos/testing/bokeh.html'
    #     return render_template(template, scripts=scripts, divs=divs)


@app.route('/chronos/testing/<subdir>', methods=['POST'])
def testing_bokeh_post(subdir):
    textfield_1 = request.form['textfield_1']
    return chronos_testing(subdir, textfield_1)


@app.route('/old/<subdir>')
def old(subdir):
    return render_template(f'old/{subdir}.html')


# @app.route('/comp_phys/n_body/<subdir_name>')
# def comp_phys_n_body(subdir_name):

#     def get_n_body_simulations(subdir_name):

#         # get output file names
#         path_to_output_files = os.path.join(
#             f'./comp_phys/n_body/out/{subdir_name}'
#         )
#         sorted_output_files = sorted(os.listdir(path_to_output_files))

#         simulations = []
#         for sim_idx, output_file in enumerate(sorted_output_files):
#             # skip directories
#             path_to_output_file = os.path.join(
#                 path_to_output_files, output_file)
#             if os.path.isdir(path_to_output_file):
#                 continue
#             # load output data
#             system_states = np.loadtxt(path_to_output_file)
#             system_states = [list(i) for i in system_states]
#             # append dict containing simulation info to list
#             simulations.append({
#                 'system_states': json.dumps(system_states),
#                 'sim_id': output_file[:-4],
#                 'sim_idx': sim_idx,
#                 'outfile_name': output_file,
#             })

#         return simulations

#     simulations = get_n_body_simulations(subdir_name)
#     # return props directory
#     props = {
#         'simulations': simulations,
#     }
#     return render_template('comp_phys/n_body.html', props=props)


# @app.route('/bokeh/')
# def bokeh():
#     scripts, divs = [], []
#     kwargs = {'scripts': scripts, 'divs': divs}
#     return render_template('bokeh.html', **kwargs)


# @app.route('/bokeh/', methods=['POST'])
# def bokeh_post():
#     plot_name = request.form['text']

#     scripts, divs = [], []
#     if plot_name == 'sleep':
#         script, div = plots.bokeh.sleep_analysis.sleep_snake()
#         scripts.append(script)
#         divs.append(div)
#     elif plot_name == 'map':
#         script, div = plots.bokeh.various.location_history()
#         scripts.append(script)
#         divs.append(div)
#     elif plot_name == 'grades':
#         script, div = plots.bokeh.various.grades()
#         scripts.append(script)
#         divs.append(div)
#     elif plot_name == 'heatmap':
#         # data = chronos.stats.sample_data()
#         data = stats.time_series.daily_log.chars_written_in_daily_log()
#         script, div = plots.bokeh.basic.heatmap(data, 'daily')
#         scripts.append(script)
#         divs.append(div)
#     elif plot_name == 'altitude':
#         script, div = plots.bokeh.various.altitude_history()
#         scripts.append(script)
#         divs.append(div)

#     kwargs = {'scripts': scripts, 'divs': divs}
#     return render_template('bokeh.html', **kwargs)


# @app.route('/messages/')
# def messages():
#     scripts, divs = [], []
#     kwargs = {'scripts': scripts, 'divs': divs}
#     return render_template('bokeh.html', **kwargs)


# @app.route('/messages/', methods=['POST'])
# def messages_post():
#     scripts, divs = [], []
#     query = request.form['text']

#     script, div = plots.bokeh.various.messages(query)
#     scripts.append(script)
#     divs.append(div)

#     kwargs = {'scripts': scripts, 'divs': divs}
#     return render_template('bokeh.html', **kwargs)


# @app.route('/heatmap/')
# def heatmap():

#     values = range(64)
#     NR_OF_ROWS = 6
#     REST = len(values) % NR_OF_ROWS
#     NR_OF_COLS = int((len(values) - REST) / NR_OF_ROWS)
#     kwargs = {
#         # 'NR_OF_ROWS': NR_OF_ROWS,
#         'NR_OF_COLS': NR_OF_COLS,
#         'REST': REST,
#         'values': values,
#         'values_len': len(values),
#     }

#     return render_template('heatmap.html', **kwargs)


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
