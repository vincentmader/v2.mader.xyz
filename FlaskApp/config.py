from datetime import datetime as dt
import json
import os
import sys


running_on_server = True

if running_on_server:
    PATH_TO_PROJECT = '/var/www/maderxyz/FlaskApp/'
else:
    import pymongo
    MDB = pymongo.MongoClient('localhost', 27017)['maderxyz']

    PATH_TO_PROJECT = './FlaskApp'
    if sys.platform == 'darwin':
        PATH_TO_RAW_DATA = '/Users/vinc/Documents/chronos_data/'
        PATH_TO_DAILY_LOGS = '/Users/vinc/Library/Mobile Documents/com~apple~CloudDocs/org'
    elif sys.platform == 'linux':
        PATH_TO_RAW_DATA = '/home/vinc/docs/chronos_data/'
        PATH_TO_DAILY_LOGS = '/home/vinc/org/journal'

PATH_TO_CHRONOS = os.path.join(PATH_TO_PROJECT, 'chronos')
PATH_TO_STATIC = os.path.join(PATH_TO_PROJECT, 'static/')

# GENERAL
START_DATE = dt(2010, 1, 1)
END_DATE = dt(dt.now().year, 12, 31)
PLOT_WIDTH, PLOT_HEIGHT = 400, 400

# INDEX

INDEX_NAVGRID_SECTIONS = [{
    'title': 'gravitational n-body dynamics',
    'pages': [
        {
            'id': '3body_moon',
            'link': '/comp_phys/n_body/3body_moon'
        }, {
            'id': '3body_fig8',
            'link': '/comp_phys/n_body/3body_fig8'
        }, {
            'id': 'nbody_flowers',
            'link': '/comp_phys/n_body/flowers'
        }, {
            'id': 'nbody_asteroids',
            'link': '/comp_phys/n_body/asteroids'
        },
    ]
}, {
    'title': 'harmonical oscillations',
    'pages': [
        {
            'id': 'single_pendulum',
            'link': '/comp_phys/harmonical_oscillators/single_pendulum'
        }, {
            'id': 'double_pendulum',
            'link': '/comp_phys/harmonical_oscillators/double_pendulum'
        }, {
            'id': 'lissajous',
            'link': '/comp_phys/harmonical_oscillators/lissajous'
            # TODO: solve analytically (-> performance)
        }, {
            'id': 'mc_pi_darts',
            'link': '/comp_phys/monte_carlo/pi_darts'
        },
    ]
}, {
    'title': 'statistical physics & thermodynamics',
    'pages': [
        {
            'id': 'gas_in_a_box',
            'link': '/comp_phys/stat_phys/thermal_motion'
        }, {
            'id': 'brownian_motion',
            'link': '/comp_phys/stat_phys/brownian_motion'
        }, {
            'id': 'ising',
            'link': '/comp_phys/stat_phys/ising'
        },
    ]
}, {
    'title': 'emergent behavior & cellular automata',
    'pages': [
        {
            'id': 'boids',
            'link': '/comp_phys/monte_carlo/boids'
        }, {
            'id': 'ants',
            'link': '/comp_phys/monte_carlo/ants'
        }, {
            'id': 'game_of_life',
            'link': '/comp_phys/cellular_automata/game_of_life'
        }, {
            'id': 'rock_paper_scissors',
            'link': '/comp_phys/cellular_automata/rock_paper_scissors'
        },
    ]
}, {
    'title': 'electro-magnetism',
    'pages': [
        {
            'id': 'lorentz',
            'link': '/old/lorentz'
        }, {
            'id': 'charge_interaction',
            'link': '/comp_phys/electro_magnetism/charge_interaction'
        },
    ]
}, {
    'title': 'fluid dynamics',
    'pages': [
        {
            'id': 'incompressible_fluid',
            'link': '/comp_phys/fluid_dynamics/incompressible_fluid'
        }, {
            'id': 'diffusion',
            'link': '/comp_phys/fluid_dynamics/diffusion'
        },
    ]
}, {
    'title': 'chronos',
    'pages': [
        {
            'id': 'correlation_finder',
            'link': '/chronos/stats/correlation_finder'
        }, {
            'id': 'stats',
            'link': '/chronos/stats/stats'
        }, {
            'id': 'test',
            'link': '/chronos/stats/test'
        }
    ]
}, {
    'title': 'stuff/unfinished',
    'pages': [
        {
            'id': 'tatooine', 'link': '/old/tatooine'
        }, {
            'id': 'forest_fire',
            'link': '/comp_phys/cellular_automata/forest_fire'
        }, {
            'id': 'quadtree',
            'link': '/comp_phys/various/quadtree'
        }, {
            'id': 'nbody_cloud',
            'link': '/comp_phys/n_body/cloud'
        }, {
            'id': 'react_test',
            'link': '/react'
        },
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
}]

# handle privacy on server: don't display chronos in nav view
if running_on_server:
    INDEX_NAVGRID_SECTIONS = [
        i for i in INDEX_NAVGRID_SECTIONS if i['title'] != 'chronos'
    ]
else:
    # PARAMETERS FOR IMPORTING RAW DATA

    FACEBOOK_USER_NAME = 'Vincent Mader'

    GSPREAD_CREDS_SETUP = os.path.exists(
        os.path.join(PATH_TO_RAW_DATA, 'creds', 'gspread_creds.json')
    )

    GSHEETS_DAILY_REVIEW_LOCS = {
        2018: (range(58, 58+9), range(46, 46+365)),
        2019: (range(56, 56+9), range(25, 25+365)),
        2020: (range(56, 56+9), range(25, 25+365)),
    }

    GSHEETS_TIMETABLE_LOCS = {
        2017: (range(3, 3+48), range(45, 45+365)),
        2018: (range(3, 3+48), range(46, 46+365)),
        2019: (range(2, 2+48), range(25, 25+365)),
        2020: (range(2, 2+48), range(25, 25+365)),
    }

    SLEEP_CYCLE_NOTE_TRANSLATION = {
        'health': {
            'activity': {
                'whether I played ping pong': ['table tennis'],
                'whether I took a walk': ['took a walk'],
                'whether I exercised physically': ['Sport gemacht', 'Worked out'],
            }, 'diet': {
                'whether I ate late': ['Ate late', 'Spaet gegessen'],
                'whether I drank tea': ['Tea', 'Tee getrunken'],
                'whether I ate vegan': ['Ate Vegan'],
                'whether I ate vegetarian': ['Ate Vegetarian'],
                'whether I went to bed hungry': ['hungry'],
                'whether dinner was great': ['dinner was great'],
                'whether I cooked for myself': ['cooked for myself'],
                'whether I ate nothing all day': ['Ate nothing'],
            }, 'drug consumption': {
                'whether I consumed alcohol': ['Alcohol', 'Drank Alcohol'],
                'whether I consumed caffeine': ['Coffee', 'Kaffee getrunken'],
                'whether I consumed mdma': ['E'],
                'whether I consumed tabacco': ['Smoked Tabacco'],
                'whether I consumed weed': ['Blaze', 'Smoked Weed'],
                'whether I smoked cigarettes': ['Smoked Cigarettes'],
                'whether I drank a bit of beer': ['drank < 1l beer'],
                'whether I drank a lot of beer': ['drank > 1l beer'],
                'whether I drank a bit of schnaps': ['drank a bit of Schnaps'],
                # 'whether I drank a lot of schnaps': ['drank a lot of Schnaps'],
            }, 'hygiene': {
                'whether I took a shower': ['Shower'],
                'whether I took a bath': ['Bath'],
                'whether I wore braces at night': ['Zahnspange'],
            }, 'sickness': {
                'whether I puked': ['puked'],
                'whether I felt sick': ['Sick'],
                'whether I had a headache': ['Headache'],
                'whether I had back pain': ['back pain'],
                'whether I had broken bones': ['Broken bones'],
                'whether I had dry hands': ['dry hands'],
            }, 'sleep analysis': {
                'whether I listened to music before sleep': ['Listening to Music'],
                # 'whether I went to bed drunk': ['drunk'],
                # 'whether I went to bed stoned': ['stoned'],
                'whether I went to bed tired': ['Tired'],
                'whether I slept in a tent': ['Sleeping in a tent '],
                'whether I slept under clear sky': ['Sleeping under clean sky'],
            }
        }, 'personal': {
            'mood': {
                'good day': ['Good day'],
                'sad day': ['Sad Day'],
                'bad day': ['Bad Day'],
                'stressful day': ['Stressful day', 'Anstrengender Tag'],
                'whether I think life is great': ['life is great'],
            }, 'sexual': {
                'whether I had sex': ['Had Sex'],
                'whether I fapped': ['Fapped'],
                'whether I slept in bed with Selina': ['In Bed with Selina'],
            }, 'location': {
                'whether I was in Ulm': ['In Ulm'],
                'whether I was in Heidelberg': ['In Heidelberg'],
                'whether I was in Berlin': ['In Berlin'],
            }
        }, 'learning': {
            'various': {
                'wrote code': ['wrote code'],
            }
        }, 'personal': {
            'mood': {
                'great mood': [' great'],  # should be 'mood: great'
                'happy mood': [' happy'],  # but: split err at ':'
                'weird mood': [' weird'],  # same
                'sad mood': [' sad'],      # same
                'good day': ['Good day'],
                'bad day': ['Bad Day'],
                'sad day': ['Sad Day'],
                'stressful day': ['Stressful day'],
                'whether I think life is great': ['life is great'],
            }, 'sexual': {
                'whether I had sex': ['Had Sex'],
                'whether I fapped': ['Fapped'],
                'whether I slept in bed with Selina': ['In Bed with Selina'],
            }, 'location': {
                'whether I was in Ulm': ['In Ulm'],
                'whether I was in Heidelberg': ['In Heidelberg'],
                'whether I was in Berlin': ['In Berlin '],
            }
        }
    }
