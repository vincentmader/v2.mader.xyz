import os
from maderxyz.chronos import load_raw, stats


# TODO: remove functions?

MDB_HIERARCHY = {
    # 'raw data': {
    #     'daily log': {  # one text document per day
    #         'db_entry_keys': [
    #             'date', 'content', 'nr of characters', 'nr of words'
    #         ],
    #         # 'f': load_raw.daily_log.daily_log_history
    #     }, 'facebook}': {
    #         # chat_history
    #     }, 'qs export': {  # precision of seconds/minutes
    #         'active calories': {
    #             'db_entry_keys': [
    #                 'date', 'start', 'end', 'active calories'
    #             ],  # 'f': load_raw.qs_export.energy_active
    #         }, 'cycling distance': {
    #             'db_entry_keys': [
    #                 'date', 'start', 'end', 'cycling distance'
    #             ],  # 'f': load_raw.qs_export.cycling_distance
    #         }, 'distance': {
    #             'db_entry_keys': [
    #                 'date', 'start', 'end', 'distance'
    #             ],  # 'f': load_raw.qs_export.distance
    #         }, 'heart rate': {
    #             'db_entry_keys': [
    #                 'date', 'start', 'end', 'heart rate'
    #             ],  # 'f': load_raw.qs_export.heart_rate
    #             # }, 'heart rate at rest': {
    #             #     'db_entry_keys': ['date', 'start', 'end', 'heart rate at rest'],
    #             #     # 'f': load_raw.qs_export.heart_rate_at_rest
    #             # }, 'heart rate variability': {
    #             #     'db_entry_keys': ['date', 'start', 'end', 'heart rate variability'],
    #             #     # 'f': load_raw.qs_export.heart_rate_variability
    #         }, 'sleep analysis': {
    #             'db_entry_keys': [
    #                 'date', 'start', 'end', 'sleep analysis'
    #             ],  # 'f': load_raw.qs_export.sleep_analysis
    #         }, 'steps': {
    #             'db_entry_keys': [
    #                 'date', 'start', 'end', 'steps'
    #             ],  # 'f': load_raw.qs_export.steps
    #             # }, 'exercise minutes': {
    #             #     'db_entry_keys': ['date', 'start', 'end', 'exercise minutes'],
    #             #     # 'f': load_raw.qs_export.exercise_minutes
    #             # }, 'stand hours': {
    #             #     'db_entry_keys': ['date', 'start', 'end', 'stand hours'],
    #             #     # 'f': load_raw.qs_export.stand_hours
    #             # }, 'mindful minutes': {
    #             #     'db_entry_keys': ['date', 'start', 'end', 'mindful minutes'],
    #             #     # 'f': load_raw.qs_export.mindful_minutes
    #         },
    #     }, 'sleep_cycle': {  # daily
    #         'db_entry_keys': [
    #             'date', 'start', 'end', 'sleep quality',
    #             'hours in bed', 'wake-up mood'
    #         ],  # 'f': load_raw.sleep_cycle.sleep_history,
    #     }
    # },
    'stats': {
        'time series': {
            'health': {
                'activity': [
                    # 'active calories',
                    # 'cycling distance',
                    # 'distance',
                    # 'flights climbed',
                    # 'heart rate',
                    # 'heart rate at rest',
                    # 'steps',
                    # 'at least one pull-up/push-up',
                    'whether I exercised physically',
                    'whether I played ping pong',
                    'whether I took a walk',
                    # 'small bike tour',
                    # 'large bike tour',
                    # ...
                ], 'diet': [
                    'whether dinner was great',
                    # 'whether I ate fish',
                    'whether I ate late',
                    # 'whether I ate nothing all day',
                    # 'whether I ate meat',
                    'whether I ate vegetarian',
                    'whether I ate vegan',
                    'whether I cooked for myself',
                    'whether I drank tea',
                    'whether I went to bed hungry',
                ], 'drug consumption': [
                    'whether I consumed alcohol',
                    'whether I consumed caffeine',
                    'whether I consumed mdma',
                    # 'whether I consumed mushrooms',
                    # 'whether I consumed N2O',
                    # 'whether I consumed poppers',
                    'whether I consumed tabacco',
                    'whether I consumed weed',
                    'whether I drank a bit of schnaps',
                    # 'whether I drank a lot of schnaps',
                    'whether I drank a bit of beer',
                    'whether I drank a lot of beer',
                    'whether I smoked cigarettes',
                    # amount of weed (1, 2-3, 4+)
                ], 'hygiene': [
                    'whether I took a shower',
                    'whether I took a bath',
                    'whether I wore braces at night',
                ], 'sickness': [
                    'whether I had back pain',
                    'whether I felt sick',
                    'whether I puked',
                    'whether I had a headache',
                    'whether I had dry hands',
                    'whether I had broken bones',
                    # fnail?
                ], 'sleep analysis': [
                    'hours in bed',
                    # 'hours asleep',
                    # 'minutes to fall asleep',
                    'sleep quality',
                    'wake-up mood',
                    # 'get-up time',
                    # 'bed time',
                    # 'nr of sleep cycles',
                    # 'whether I went to bed drunk',
                    # 'whether I went to bed stoned',
                    'whether I went to bed tired',
                    'sleep regularity',
                    'whether I listened to music before sleep',
                    # 'bright screen before sleep',
                    # 'meditation before sleep',
                    # 'on reddit/youtube before sleep',
                    # 'read a few pages before sleep',
                    'whether I slept in a tent',
                    'whether I slept under clear sky',
                    # nap?
                ],
            }, 'learning': {
                'various': [
                    'wrote code',
                    'anki/duolingo',
                ]
            }, 'social': {
                'instant messaging': [
                    # 'messages sent',
                    # 'messages received',
                    # 'messages total',
                    # 'nr. of characters sent',
                    # 'nr. of characters received',
                    # 'nr. of characters total',
                ]
            }, 'personal': {
                'mood': [
                    'great mood',
                    'happy mood',
                    'weird mood',
                    'sad mood',
                    'good day',
                    'bad day',
                    'sad day',
                    'stressful day',
                    'whether I think life is great',
                ], 'sexual': [
                    'whether I had sex',
                    'whether I fapped',
                    'whether I slept in bed with Selina',
                ], 'location': [
                    'whether I was in Ulm',
                    'whether I was in Heidelberg',
                    'whether I was in Berlin',
                ]
                # }, 'various': {
                #     'finances': {
                #         'db_entry_keys': {
                # 'income': ['date', 'income'],
                # 'expenses': ['date', 'expenses'],
                # 'net worth': ['date', 'net worth'],
            }, 'various': {
                'seasons etc.': [
                    'whether it was spring',
                    'whether it was weekend',
                    'whether it was summer',
                    'whether it was fall',
                    'whether it was winter',
                    # semester / school / vacation
                ],
            }
            # }, 'location history': {
            #     'db_entry_keys': {
            # 'latitude': ['date', 'latitude'],
            # 'longitude': ['date', 'longitude'],
            # 'altitude': ['date', 'altitude'],
            # 'velocity': {},
            # },
            # }, 'various': {
            #     'db_entry_keys': {
            # 'on vacation': ['date', 'on vacation'],
            # 'stayed inside all day': ['date', 'stayed inside all day'],
            # 'lots of gaming': ['date', 'lots of gaming'],
            # writing exam tomorrow
            # # wrote code
            # }
            # }
            # }, 'external comparison data': {
            # 'semesterferien'
            # 'seasons'
            # 'weather': {
            #     'db_entry_keys': {
            #         'peak temperature': ['date', 'peak temperature']  # from geo data
            # }
            # }
            # }
        }
        # }, 'correlations': {
        #     'db_entry_keys': {
        #         'correlation matrix': []
        #     },  # 'f': stats.correlation_finder.main
    },
}

MDB_TS_CATEGORIES = []
count = 0
for k, v in MDB_HIERARCHY['stats']['time series'].items():
    for k2, v2 in v.items():
        for k3 in v2:
            MDB_TS_CATEGORIES.append(k3)
