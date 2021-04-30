from datetime import datetime as dt
import os
import pymongo


PATH_TO_PROJECT = '/home/vinc/code/mader.xyz/FlaskApp/'
PATH_TO_CHRONOS = '/home/vinc/code/mader.xyz/FlaskApp/chronos'
PATH_TO_STATIC = '/home/vinc/code/mader.xyz/FlaskApp/static/'
PATH_TO_RAW_DATA = '/home/vinc/docs/chronos_data/'
PATH_TO_DAILY_LOGS = '/home/vinc/org/journal'


GSPREAD_CREDS_SETUP = os.path.exists(
    os.path.join(PATH_TO_RAW_DATA, 'creds', 'gspread_creds.json')
)

START_DATE = dt(2010, 1, 1)
END_DATE = dt(dt.now().year, 12, 31)


FACEBOOK_USER_NAME = 'Vincent Mader'

PLOT_WIDTH, PLOT_HEIGHT = 400, 400

MDB = pymongo.MongoClient('localhost', 31416)['maderxyz']

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
