import os


PATH_TO_PROJECT = '/home/vinc/code/mader.xyz/FlaskApp/chronos/'
PATH_TO_STATIC = '/home/vinc/code/mader.xyz/FlaskApp/static/'
PATH_TO_RAW_DATA = '/home/vinc/docs/chronos_data/'
PATH_TO_DAILY_LOGS = '/home/vinc/org/journal'

GSPREAD_CREDS_SETUP = os.path.exists(
    os.path.join(PATH_TO_RAW_DATA, 'creds', 'gspread_creds.json')
)
