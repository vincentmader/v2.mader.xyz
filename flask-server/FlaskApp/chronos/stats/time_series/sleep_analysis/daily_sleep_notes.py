# TODO: rename, sleep cycle notes?

from datetime import datetime as dt
from datetime import timedelta as td

import pymongo

from FlaskApp import config, chronos, db_config
from FlaskApp.config import MDB, START_DATE, END_DATE
from FlaskApp.config import SLEEP_CYCLE_NOTE_TRANSLATION as translate_notes
from FlaskApp.chronos.io import save_to_database
from FlaskApp.chronos.stats.time_series.various.empty import main as prepare_dict


def main():

    mdb_hierarchy = db_config.MDB_HIERARCHY['stats']['time series']
    a = list(mdb_hierarchy.keys())
    for cat in mdb_hierarchy.keys():
        for subcat in mdb_hierarchy[cat].keys():

            # skip stuff unrelated to sleep analysis
            if cat not in translate_notes.keys():
                print(f'\twarn: \'{cat}\' translation set not found (SC)')
                continue  # TODO: print error? -> nope (?)
            if subcat not in translate_notes[cat].keys():
                print(f'\twarn: \'{subcat}\' translation not found (SC)')
                continue

            # get time-series titles to be used in db
            db_keys = translate_notes[cat][subcat].keys()

            # prepare empty time series (containing only None)
            foo = {}  # TODO: skip this?
            for db_key in db_keys:
                foo[db_key] = {}
                # foo[db_key] = prepare_dict(START_DATE, END_DATE)

            # load collection for deletion before write (TODO: remove?)
            # collection = MDB['stats']['time series']['daily'][cat][subcat]

            # fill time series object
            for db_key in db_keys:

                # collection[db_key].delete_many({})  # (TODO: remove?)

                # get entries & sort
                entries = config.MDB['raw data']['sleep cycle'].find({})
                entries = entries.sort([("date", pymongo.ASCENDING)])

                found_entry = False
                for entry in entries:
                    found_translation = False
                    for translation in translate_notes[cat][subcat][db_key]:
                        if translation in entry['sleep notes']:
                            foo[db_key][entry['date']] = 1
                            found_entry = True
                            found_translation = True
                            # print(translation)
                            break
                    if found_entry:
                        if not found_translation:
                            foo[db_key][entry['date']] = 0

                # create document & save to database
                dates = sorted(foo[db_key].keys())
                timestamps = [d.timestamp() for d in dates]
                values = [foo[db_key][d] for d in dates]
                # print(values)
                # print(translation)
                # input()
                document = {
                    'title': db_key,
                    'category': cat,
                    'subcategory': subcat,
                    'timestamps': timestamps,
                    # 'dates': dates,
                    'values': values,
                    'resolution': 'daily',
                }
                path = ['stats', 'time series']
                save_to_database(document, path=path)
