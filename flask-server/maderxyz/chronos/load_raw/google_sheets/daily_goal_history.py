from datetime import datetime as dt
from datetime import timedelta as td

import maderxyz.config as config
from .generic_gsheet import main as generic_gsheet


def main():

    daily_goal_history = []

    for year in range(2018, 2020+1):

        gsheet = generic_gsheet(f'Time{year} (Vincent)').sheet1
        gsheet = gsheet.get_all_values()

        row_range = config.GSHEETS_DAILY_REVIEW_LOCS[year][1]
        col_range = config.GSHEETS_DAILY_REVIEW_LOCS[year][0]
        date_str_idx = config.GSHEETS_TIMETABLE_LOCS[year][0][0]
        categories = config.GSHEETS_DAILY_REVIEW_CATEGORIES

        for jdx in row_range:
            row = gsheet[jdx]
            date_str = row[date_str_idx]
            timestamp = (dt(year, 1, 1) + td(days=jdx)).timestamp()

            daily_goals = {'timestamp': timestamp}
            for idx in col_range:
                cell = row[idx]
                category_idx = idx - col_range[0]
                category = categories[category_idx]

                daily_goals[category] = cell

            daily_goal_history.append(daily_goals)

    return daily_goal_history
