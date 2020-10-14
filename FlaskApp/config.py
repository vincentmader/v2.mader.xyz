import os


PATH_TO_PROJECT = '/home/vinc/Documents/mader.xyz/FlaskApp/chronos/'
PATH_TO_RAW_DATA = '/home/vinc/Documents/chronos_data/'

GSPREAD_CREDS_SETUP = os.path.exists(
    os.path.join(PATH_TO_RAW_DATA, 'creds', 'gspread_creds.json')
)
