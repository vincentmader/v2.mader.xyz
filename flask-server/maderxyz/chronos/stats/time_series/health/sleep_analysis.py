from datetime import datetime as dt
from datetime import timedelta as td

from maderxyz import config, chronos
from maderxyz.config import MDB, START_DATE, END_DATE
from maderxyz.chronos.io import save_to_database
from maderxyz.chronos.stats.time_series.various.empty import main as prepare_dict


db_keys = [
    'sleep quality', 'hours in bed', 'sleep regularity',
    'wake-up mood',
    # 'start', 'end',  # TODO: include
    # 'nr of sleep cycles',
    # 'wake-up time', 'go-to-bed time'
]


def main():

    # remove entries from database collection
    coll = MDB['stats']['time series']['daily']['health']['sleep analysis']
    for db_key in db_keys:
        coll[db_key].delete_many({})  # TODO: comment out this section?

    # load raw data
    sleep_cycle_history = MDB['raw data']['sleep cycle'].find({})

    # setup temporary dictionary to hold time series values
    foo = {k: prepare_dict(START_DATE, END_DATE) for k in db_keys}
    for entry in sleep_cycle_history:
        for db_key in db_keys:
            date = entry['date']
            value = entry[db_key]
            foo[db_key][date] = value

    for db_key in db_keys:
        # adjust title (TODO: change this earlier? in load_raw?)
        title = db_key
        if db_key == 'start':
            title = 'bed time'
        if db_key == 'end':
            title = 'get-up time'
        # create document & save to database
        dates = sorted(foo[db_key].keys())
        timestamps = [d.timestamp() for d in dates]
        values = [foo[db_key][d] for d in dates]
        document = {
            # 'dates': dates,
            'timestamps': timestamps,
            'values': values,
            'category': 'health',
            'subcategory': 'sleep analysis',
            'title': title,
            'datatype': 'num',
            'resolution': 'daily',  # TODO: not really needed, already in path
        }
        path = ['stats', 'time series']
        save_to_database(document, path=path)

        # if db_key == 'wake-up mood':
        #     print([k for k in values if k is not None])
        #     input()
