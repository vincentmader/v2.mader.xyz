from datetime import datetime as dt
from datetime import timedelta as td

import matplotlib.pyplot as plt

import maderxyz.config
from maderxyz.chronos import load_raw


def translate_keys(key):

    translate = {
        'Smoked Weed': 'no weed',
        'Fapped': 'no fap',
        'Drank Alcohol': 'no alcohol',
        # 'reading',
        # 'exercise',
        # 'no youtube',
        # 'anki',
        # 'no meat',
        # 'no fap',
        # 'no weed',
    }

    if key not in translate.keys():
        return key

    return translate[key]


def main():

    KEYS = ['no weed', 'no fap', 'no alcohol']
    negative_keys = ['Smoked Weed', 'Fapped', 'Drank Alcohol']
    positive_keys = []
    SLEEP_NOTE_KEYS = positive_keys + negative_keys

    # load data
    daily_goals = load_raw.google_sheets.daily_goal_history()
    sleep_cycle = load_raw.sleep_cycle.sleep_history()

    # prepare output dictionary
    out = {'timestamp': []}
    start = dt.strptime("01-01-2012", "%d-%m-%Y")
    end = dt.now()
    time_span = [
        (start + td(days=x)).timestamp()
        for x in range((end-start).days)
    ]
    for day_timestamp in time_span:
        out['timestamp'].append(day_timestamp)
    for key in KEYS:
        out[key] = []
        for day_timestamp in time_span:
            out[key].append(None)

    # read sleep notes from sleep cycle and write to out
    for entry in sleep_cycle:
        start_dt = dt.fromtimestamp(entry['start_timestamp'])
        day_dt = dt.strptime(dt.strftime(start_dt, '%Y-%m'), '%Y-%m')
        day_timestamp = day_dt.timestamp()
        sleep_notes = entry['notes']

        for key in SLEEP_NOTE_KEYS:
            translated_key = translate_keys(key)
            idx = out['timestamp'].index(day_timestamp)
            if key in sleep_notes:
                if key in negative_keys:
                    out[translated_key][idx] = 0
                elif key in positive_keys:
                    out[translated_key][idx] = 1
            else:
                if key in negative_keys:
                    out[translated_key][idx] = 1
                elif key in positive_keys:
                    out[translated_key][idx] = 0

    # read daily goals and write to out
    for entry in daily_goals:
        timestamp = entry['timestamp']
        for key in entry.keys():
            if key == 'timestamp' or key not in KEYS:
                continue
            else:
                try:
                    idx = out['timestamp'].index(timestamp)
                    if out[key][idx] is not None:
                        continue  # only write if sleep cycle has not already
                except ValueError:  # skip when timestamp > now
                    continue
                if entry[key] == 'g':
                    out[key][idx] = 1
                elif entry[key] == 'b':
                    out[key][idx] = 0

    return out

    # bar = {}

    # for entry in gsheet_goal_history:
    #     for k in entry.keys():
    #         if k not in bar.keys():
    #             bar[k] = []
    #         baz = None
    #         if k == 'timestamp':
    #             baz = entry[k]
    #         else:
    #             if entry[k].lower() == 'b':
    #                 baz = 0
    #             elif entry[k].lower() == 'g':
    #                 baz = 1

    #         bar[k].append(baz)

    # for entry in sleep_cycle_data:
    #     sleep_notes = entry['notes']
    #     start_dt = dt.fromtimestamp(entry['start_timestamp'])
    #     day_timestamp = dt.strptime(
    #         start_dt.strftime('%Y-%m'), '%Y-%d').timestamp()
    #     try:
    #         day_idx = bar['timestamp'].find(day_timestamp)
    #     except AttributeError:
    #         day_idx = -1

    #     if 'Smoked Weed' not in sleep_notes:
    #         if day_idx == -1:
    #             # bar['timestamp'].append(day_timestamp)
    #             # bar['no weed'].append(1)
    #             pass
    #         # elif bar['no weed'][day_idx] is not None:
    #         #     bar['no weed'][day_idx] += 1
    #         else:
    #             bar['no weed'][day_idx] = 1

    # print(sleep_notes)

    # return bar
