import json
import os
import sys

import flask
from flask import Flask
from flask import render_template, send_from_directory, request
import numpy as np

from .config import PATH_TO_STATIC
from .config import PATH_TO_PROJECT
# from .chronos import plots, stats


# initialize app
app = Flask(__name__)

# views
# =============================================================================


# index
@app.route('/')
def index():

    nav_grid_sections = [
        {
            'title': 'n-body dynamics',
            'pages': [
                {
                    'id': 'nbody_moon',
                    'link': '/comp_phys/n_body/3body_moon'
                }, {
                    'id': '3body_fig8',
                    'link': '/comp_phys/n_body/3body_fig8'
                    # }, {
                    #     'id': 'n_body',
                    #     'link': '/comp_phys/n_body/3body_moon'
                }, {
                    'id': 'n_body_flowers',
                    'link': '/comp_phys/n_body/flowers'
                }, {
                    'id': 'n_body_cloud',
                    'link': '/comp_phys/n_body/cloud'
                }, {
                    'id': 'n_body_asteriods',
                    'link': '/comp_phys/n_body/asteroids'
                },
                {'id': 'quadtree', 'link': '/comp_phys/various/quadtree'},
            ]
        }, {
            'title': 'statistical physics',
            'pages': [
                {
                    'id': 'ising',
                    'link': '/comp_phys/stat_phys/ising'
                }, {
                    'id': 'gas_in_a_box',
                    'link': '/comp_phys/stat_phys/thermal_motion'
                }, {
                    'id': 'brownian_motion',
                    'link': '/comp_phys/stat_phys/brownian_motion'
                },
            ]

        }, {
            'title': 'harmonical oscillators',
            'pages': [
                {
                    'id': 'double_pendulum',
                    'link': '/comp_phys/harmonical_oscillators/pendulum'
                }, {
                    'id': 'lissajous',
                    'link': '/comp_phys/harmonical_oscillators/lissajous'
                    # TODO: solve analytically (-> performance)
                },
            ]
        }, {
            'title': 'Monte Carlo',
            'pages': [
                {
                    'id': 'mc_pi_darts',
                    'link': '/comp_phys/monte_carlo/pi_darts'
                },
                {'id': 'ants', 'link': '/comp_phys/monte_carlo/ants'},
            ]
        }, {
            'title': 'cellular automata',
            'pages': [
                {
                    'id': 'game_of_life',
                    'link': '/comp_phys/cellular_automata/game_of_life'
                }, {
                    'id': 'rock_paper_scissors',
                    'link': '/comp_phys/cellular_automata/rock_paper_scissors'
                }, {
                    'id': 'forest_fire',
                    'link': '/comp_phys/cellular_automata/forest_fire'
                },
                {'id': 'boids', 'link': '/comp_phys/monte_carlo/boids'},
            ]
        }, {
            'title': 'electro-magnetism',
            'pages': [
                {'id': 'lorentz', 'link': '/old/lorentz'},
            ]
        }, {
            'title': 'stuff',
            'pages': [
                {'id': 'tatooine', 'link': '/old/tatooine'},
                # {'id': 'orbit'},
                # {'id': 'solar'},
                # {'id': 'factfulness'},
                # {'id': 'bachelor_thesis'},
                # {'id': 'spotify'},
                # {'id': 'boltzmann', 'link': '/comp_phys/boltzmann'},
            ]
            # }, {
            #     'title': 'unfinished',
            #     'pages': [
            #         {'id': 'testing_bokeh', 'link': '/chronos/testing/bokeh'},
            #         {'id': 'testing_chartjs', 'link': '/chronos/testing/chartjs'},
            #         {'id': 'testing_pyplot', 'link': '/chronos/testing/pyplot'},
            #     ]
        }
    ]

    props = {'nav_grid_sections': nav_grid_sections}
    return render_template('index.html', props=props)


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
            'sim_title': '3-body orbits - stable figure eight'
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

    if subdir == 'pendulum':
        path_to_output_file = os.path.join(
            PATH_TO_PROJECT, 'comp_phys/double_pendulum/out/ys.txt'
        )
        system_states = [list(i) for i in np.loadtxt(path_to_output_file)]
        template = 'comp_phys/oscillators/pendulum.html'
        props = {
            'title': 'double pendulum',
            'description': 'theory, latex...',
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
        system_states = np.loadtxt(
            PATH_TO_PROJECT + 'comp_phys/gas_in_a_box/out/ys.txt'
        )
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


@app.route('/comp_phys/various/<subdir>')
def comp_phys_various(subdir):

    if subdir == 'quadtree':
        template = 'comp_phys/various/quadtree.html'
        props = {}
        return render_template(template, props=props)


# @app.route('/chronos/testing/<subdir>')
# def chronos_testing(subdir, lolol=0):

#     if subdir in ['chartjs']:
#         return render_template(f'chronos/testing/{subdir}.html')

#     if subdir == 'pyplot':
#         images = []

#         path_to_pyplots = os.path.join(PATH_TO_STATIC, 'media/pyplots')
#         for filename in os.listdir(path_to_pyplots):
#             image = {
#                 'filepath': os.path.join('media/pyplots', filename)
#             }
#             images.append(image)
#         print(images)

#         return render_template(
#             'chronos/testing/pyplot.html',
#             images=images
#         )

#     if subdir == 'bokeh':
#         plot_functions = [
#             plots.bokeh.google_takeout.gmap,
#         ]

#         scripts, divs = [], []
#         for f in plot_functions:
#             script, div = f()
#             scripts.append(script)
#             divs.append(div)

#         script, div = plots.bokeh.google_takeout.bar_plot_search_hits(
#             'youtube')
#         scripts.append(script)
#         divs.append(div)

#         x, y = stats.time_series.avg_grades()
#         # x, y = np.array(list(foo.keys())), np.array(list(foo.values()))
#         # x, y = stats.time_series.avg_grades()
#         script, div = plots.bokeh.basic.time_series(x, y)
#         scripts.append(script)
#         divs.append(div)

#         # x, y = stats.time_series.chars_written_in_daily_log()
#         # x, y = np.array(list(foo.keys())), np.array(list(foo.values()))
#         # x, y = stats.time_series.avg_grades()
#         # script, div = plots.bokeh.basic.time_series(x, y)
#         # scripts.append(script)
#         # divs.append(div)

#         template = 'chronos/testing/bokeh.html'
#         return render_template(template, scripts=scripts, divs=divs)


# @app.route('/chronos/testing/<subdir>', methods=['POST'])
# def testing_bokeh_post(subdir):
#     textfield_1 = request.form['textfield_1']
#     return chronos_testing(subdir, textfield_1)


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


# old
# =============================================================================


# @app.route('/exec/')
# def python_executable():
#     return sys.executable


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
