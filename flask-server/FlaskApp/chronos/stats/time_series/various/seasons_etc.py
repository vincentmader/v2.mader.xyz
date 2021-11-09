from FlaskApp import config, chronos
from FlaskApp.config import START_DATE, END_DATE, MDB
from FlaskApp.chronos.io import save_to_database
from FlaskApp.chronos.stats.time_series.various.empty import main as prepare_dict


db_keys = [
    'whether it was weekend',
    'whether it was spring',
    'whether it was summer',
    'whether it was fall',
    'whether it was winter',
]


def main():
    foo = {db_key: prepare_dict(START_DATE, END_DATE) for db_key in db_keys}
    for db_key in db_keys:
        for date in foo[db_key].keys():
            if db_key == 'whether it was weekend':
                if date.strftime('%a') in ['Sat', 'Sun']:
                    foo[db_key][date] = 1
                else:
                    foo[db_key][date] = 0
            elif db_key == 'whether it was spring':
                if date.strftime('%b') in ['Mar', 'Apr', 'May']:
                    foo[db_key][date] = 1
                else:
                    foo[db_key][date] = 0
            elif db_key == 'whether it was summer':
                if date.strftime('%b') in ['Jun', 'Jul', 'Aug']:
                    foo[db_key][date] = 1
                else:
                    foo[db_key][date] = 0
            elif db_key == 'whether it was fall':
                if date.strftime('%b') in ['Sep', 'Oct', 'Nov']:
                    foo[db_key][date] = 1
                else:
                    foo[db_key][date] = 0
            elif db_key == 'whether it was winter':
                if date.strftime('%b') in ['Dec', 'Jan', 'Feb']:
                    foo[db_key][date] = 1
                else:
                    foo[db_key][date] = 0

        # create document & save to database
        dates = sorted(foo[db_key].keys())
        timestamps = [d.timestamp() for d in dates]
        values = [foo[db_key][d] for d in dates]
        document = {
            # 'dates': dates,  # TODO: think re: both dates & timestamps in db?
            'timestamps': timestamps,
            'values': values,
            'category': 'various',
            'subcategory': 'seasons etc.',
            'title': db_key,
            'resolution': 'daily',
        }
        path = ['stats', 'time series']
        save_to_database(document, path=path)
