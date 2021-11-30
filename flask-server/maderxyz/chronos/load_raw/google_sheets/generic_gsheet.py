import os

import gspread
from oauth2client.service_account import ServiceAccountCredentials

from maderxyz.config import PATH_TO_CHRONOS


def main(gsheet_name):
    """open a spreadsheet from Google Sheets.

    args:
        gsheet_name (str): name of spreadsheet

    yields:
        gspread spreadsheet object
    """

    scope = [
        'https://spreadsheets.google.com/feeds',
        'https://www.googleapis.com/auth/drive'
    ]

    creds = ServiceAccountCredentials.from_json_keyfile_name(
        os.path.join(
            PATH_TO_CHRONOS, 'creds', 'gspread_creds.json'
        ), scope
    )

    sheet = gspread.authorize(creds).open(gsheet_name)

    return sheet
