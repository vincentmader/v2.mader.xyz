from datetime import datetime as dt

import gspread
import httplib2

import FlaskApp.config
from .generic_gsheet import main as load_gsheet


def main(month_str=None):
    """load data for finances from Google Sheets, save to database

    args:
        month_str (str): month identifier in format YYYY-MM
    """

# download spreadsheets and return
    year = month_str.split('-')[0]
    month = month_str.split('-')[1]

    try:
        gsheet_name = f'Finanzen {year}.{month}.'
        sheet = load_gsheet(gsheet_name).worksheet('Transaktionen')

    except gspread.exceptions.SpreadsheetNotFound:
        pass
    except httplib2.ServerNotFoundError:
        pass

    return sheet
