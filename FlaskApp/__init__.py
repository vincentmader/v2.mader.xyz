import json
import os
import sys

import flask
from flask import Flask
from flask import render_template, send_from_directory, request
import numpy as np

# import .config
from .config import PATH_TO_STATIC


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
                {'id': 'n_body_flowers', 'link': '/comp_phys/n_body/flowers'},
                # {'id': 'n_body_sym', 'link': '/comp_phys/n_body/sym'},
                # {'id': 'n_body', 'link': ''},
                # {'id': 'n_body', 'link': '/comp_phys/n_body/3body'},
                {'id': '3body_fig8', 'link': '/comp_phys/n_body/3body_fig8'},
                {'id': 'n_body', 'link': '/comp_phys/n_body/3body_moon'},
                # ^ TODO: change to 3body_moon
            ]
        }, {
            'title': 'harmonical oscillators',
            'pages': [
                {'id': 'double_pendulum', 'link': '/comp_phys/pendulum'},
                {'id': 'lissajous', 'link': '/comp_phys/lissajous'},
            ]
        }, {
            'title': 'cellular automata',
            'pages': [
                {'id': 'game_of_life', 'link': '/comp_phys/game_of_life'},
                {'id': 'rock_paper_scissors', 'link': '/comp_phys/rock_paper_scissors'},
                {'id': 'forest_fire', 'link': '/comp_phys/forest_fire'},
            ]
        }, {
            'title': 'statistical physics',
            'pages': [
                {'id': 'ising', 'link': '/comp_phys/ising'},
                {'id': 'gas_in_a_box', 'link': '/comp_phys/gas_in_a_box/thermal_motion'},
                {'id': 'brownian_motion', 'link': '/comp_phys/gas_in_a_box/brownian_motion'},
            ]
        }, {
            'title': 'Monte Carlo',
            'pages': [
                {'id': 'mc_pi_darts', 'link': '/comp_phys/mc_pi_darts'},
            ]
        }, {
            'title': 'stuff',
            'pages': [
                {'id': 'tatooine', 'link': '/tatooine'},
                {'id': 'lorentz', 'link': '/lorentz'},
            ]
        }, {
            'title': 'unfinished',
            'pages': [
                {'id': 'testing_bokeh', 'link': '/chronos/testing/bokeh'},
                {'id': 'testing_chartjs', 'link': '/chronos/testing/chartjs'},
                {'id': 'testing_pyplot', 'link': '/chronos/testing/pyplots'},
            ]
            # {'id': 'orbit'},
            # {'id': 'solar'},
            # {'id': 'pyplot'},
            # {'id': 'factfulness'},
            # {'id': 'bachelor_thesis'},
            # {'id': 'spotify'},
            # {'id': 'boltzmann', 'link': '/comp_phys/boltzmann'},
        }
    ]

    return render_template(
        'index.html',
        nav_grid_sections=nav_grid_sections,
        nr_of_cols=4
    )


@app.route('/comp_phys/gas_in_a_box/thermal_motion')
def comp_phys_gas_in_a_box_thermal_motion():

    system_states = np.loadtxt('./FlaskApp/comp_phys/gas_in_a_box/out/ys.txt')
    system_states = [list(i) for i in system_states]

    props = {
        'ys': json.dumps(system_states),
    }
    template = 'comp_phys/gas_in_a_box/thermal_motion.html'
    return render_template(template, props=props)


@app.route('/comp_phys/gas_in_a_box/brownian_motion')
def comp_phys_gas_in_a_box_brownian_motion():
    props = {}
    template = 'comp_phys/gas_in_a_box/brownian_motion.html'
    return render_template(template, props=props)


@app.route('/comp_phys/n_body/flowers')
def comp_phys_n_body_flowers():

    props = {}
    return render_template('comp_phys/n_body_flowers.html', props=props)


@app.route('/comp_phys/n_body/3body_fig8')
def comp_phys_3body_fig8():

    # load output data
    path_to_output_file = './FlaskApp/comp_phys/n_body/out/3body_fig8/system_states.txt'
    system_states = [list(i) for i in np.loadtxt(path_to_output_file)]
    # append dict containing simulation info to list
    simulations = [{
        'system_states': json.dumps(system_states),
        'sim_id': '3body_fig8',
        'sim_idx': 0,
    }]
    # return props directory
    props = {
        'simulations': simulations,
    }
    return render_template('comp_phys/n_body.html', props=props)


@app.route('/comp_phys/n_body/3body_moon')
def comp_phys_3body_moon():

    props = {}
    return render_template('comp_phys/n_body/3body_moon.html', props=props)


@app.route('/comp_phys/lissajous')
def comp_phys_lissajous():
    props = {}
    return render_template('comp_phys/lissajous.html', props=props)


@app.route('/comp_phys/game_of_life')
def comp_phys_game_of_life():
    props = {}
    return render_template('comp_phys/game_of_life.html', props=props)


@app.route('/comp_phys/forest_fire')
def comp_phys_forest_fire():
    props = {}
    return render_template('comp_phys/forest_fire.html', props=props)


@app.route('/comp_phys/rock_paper_scissors')
def comp_phys_rock_paper_scissors():
    props = {}
    return render_template('comp_phys/rock_paper_scissors.html', props=props)


@app.route('/comp_phys/mc_pi_darts')
def comp_phys_mc_pi_darts():
    props = {}
    return render_template('comp_phys/mc_pi_darts.html', props=props)


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


@ app.route('/comp_phys/ising')
def comp_phys_ising():
    props = {}
    return render_template('comp_phys/ising.html', props=props)


@ app.route('/comp_phys/pendulum')
def comp_phys_pendulum():

    system_states = np.loadtxt('./FlaskApp/comp_phys/double_pendulum/out/ys.txt')
    system_states = [list(i) for i in system_states]

    props = {
        'title': 'double pendulum',
        'description': 'theory, latex...',
        'ys': json.dumps(system_states),
    }
    return render_template('comp_phys/pendulum.html', props=props)


# ===================================

@ app.route('/chronos/testing/chartjs')
def testing_chartjs():

    return render_template('chronos/testing/chartjs.html')


@ app.route('/chronos/testing/pyplot')
def pyplot():

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


@app.route('/chronos/testing/bokeh')
def testing_bokeh(lolol='Test'):

    plot_functions = [
        plots.bokeh.google_takeout.gmap,
    ]

    scripts, divs = [], []
    for f in plot_functions:
        script, div = f()
        scripts.append(script)
        divs.append(div)

    script, div = plots.bokeh.google_takeout.bar_plot_search_hits('youtube')
    scripts.append(script)
    divs.append(div)

    x, y = stats.time_series.avg_grades()
    # x, y = np.array(list(foo.keys())), np.array(list(foo.values()))
    # x, y = stats.time_series.avg_grades()
    script, div = plots.bokeh.basic.time_series(x, y)
    scripts.append(script)
    divs.append(div)

    # x, y = stats.time_series.chars_written_in_daily_log()
    # x, y = np.array(list(foo.keys())), np.array(list(foo.values()))
    # x, y = stats.time_series.avg_grades()
    # script, div = plots.bokeh.basic.time_series(x, y)
    # scripts.append(script)
    # divs.append(div)

    return render_template('chronos/testing/bokeh.html', scripts=scripts, divs=divs)


@app.route('/chronos/testing/bokeh', methods=['POST'])
def testing_bokeh_post():
    textfield_1 = request.form['textfield_1']
    return test(textfield_1)


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
    return render_template('old/tatooine.html')


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
    return render_template('old/lorentz.html')


@app.route('/chart_test/')
def chart_test():
    return render_template('chart_test.html')


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
        script, div = plots.bokeh.sleep_analysis.sleep_snake()
        scripts.append(script)
        divs.append(div)
    elif plot_name == 'map':
        script, div = plots.bokeh.various.location_history()
        scripts.append(script)
        divs.append(div)
    elif plot_name == 'grades':
        script, div = plots.bokeh.various.grades()
        scripts.append(script)
        divs.append(div)
    elif plot_name == 'heatmap':
        # data = chronos.stats.sample_data()
        data = stats.time_series.daily_log.chars_written_in_daily_log()
        script, div = plots.bokeh.basic.heatmap(data, 'daily')
        scripts.append(script)
        divs.append(div)
    elif plot_name == 'altitude':
        script, div = plots.bokeh.various.altitude_history()
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

    script, div = plots.bokeh.various.messages(query)
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
