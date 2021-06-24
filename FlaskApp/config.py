from datetime import datetime as dt
import os
import pymongo


PATH_TO_PROJECT = '/home/vinc/code/mader.xyz/FlaskApp/'
PATH_TO_CHRONOS = '/home/vinc/code/mader.xyz/FlaskApp/chronos'
PATH_TO_STATIC = '/home/vinc/code/mader.xyz/FlaskApp/static/'
PATH_TO_RAW_DATA = '/home/vinc/docs/chronos_data/'
PATH_TO_DAILY_LOGS = '/home/vinc/org/journal'


START_DATE = dt(2010, 1, 1)
END_DATE = dt(dt.now().year, 12, 31)


FACEBOOK_USER_NAME = 'Vincent Mader'

PLOT_WIDTH, PLOT_HEIGHT = 400, 400

MDB = pymongo.MongoClient('localhost', 27017)['maderxyz']

# PARAMETERS FOR IMPORTING RAW DATA

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

# COMP PHYS

INDEX_NAVGRID_SECTIONS = [
    {
        'title': 'n-body dynamics',
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
        'title': 'statistical physics',
        'pages': [
            {
                'id': 'ising',
                'link': '/comp_phys/stat_phys/ising'
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
        'title': 'monte carlo',
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
                'id': 'boids', 'link': '/comp_phys/monte_carlo/boids'
            }, {
                'id': 'rock_paper_scissors',
                'link': '/comp_phys/cellular_automata/rock_paper_scissors'
            },
        ]
    }, {
        'title': 'electro-magnetism',
        'pages': [
            {'id': 'lorentz', 'link': '/old/lorentz'},
        ]
    }, {
        'title': 'stuff/unfinished',
        'pages': [
            {'id': 'tatooine', 'link': '/old/tatooine'},
            {'id': 'correlation_finder',
                'link': '/chronos/stats/correlation_finder'},
            {'id': 'test',
                'link': '/chronos/stats/test'
             }, {
                'id': 'gas_in_a_box',
                'link': '/comp_phys/stat_phys/thermal_motion'
            }, {
                'id': 'brownian_motion',
                'link': '/comp_phys/stat_phys/brownian_motion'
            }, {
                'id': 'forest_fire',
                'link': '/comp_phys/cellular_automata/forest_fire'
            }, {
                'id': 'quadtree',
                'link': '/comp_phys/various/quadtree'
            }, {
                'id': 'nbody_cloud',
                'link': '/comp_phys/n_body/cloud'
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
    }
]
