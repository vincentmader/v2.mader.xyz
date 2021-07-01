import os
from FlaskApp.chronos import load_raw, stats


# TODO: remove functions?

MDB_HIERARCHY = {
    'raw data': {
        'daily log': {  # one text document per day
            'db_entry_keys': [
                'date', 'content', 'nr of characters', 'nr of words'
            ],
            # 'f': load_raw.daily_log.daily_log_history
        }, 'facebook}': {
            # chat_history
        }, 'qs_export': {  # precision of seconds/minutes
            'active calories': {
                'db_entry_keys': [
                    'date', 'start', 'end', 'active calories'
                ],  # 'f': load_raw.qs_export.energy_active
            }, 'cycling distance': {
                'db_entry_keys': [
                    'date', 'start', 'end', 'cycling distance'
                ], 'f': load_raw.qs_export.cycling_distance
            }, 'distance': {
                'db_entry_keys': [
                    'date', 'start', 'end', 'distance'
                ], 'f': load_raw.qs_export.distance
            }, 'heart rate': {
                'db_entry_keys': [
                    'date', 'start', 'end', 'heart rate'
                ], 'f': load_raw.qs_export.heart_rate
                # }, 'heart rate at rest': {
                #     'db_entry_keys': ['date', 'start', 'end', 'heart rate at rest'],
                #     # 'f': load_raw.qs_export.heart_rate_at_rest
                # }, 'heart rate variability': {
                #     'db_entry_keys': ['date', 'start', 'end', 'heart rate variability'],
                #     # 'f': load_raw.qs_export.heart_rate_variability
            }, 'sleep analysis': {
                'db_entry_keys': [
                    'date', 'start', 'end', 'sleep analysis'
                ],  # 'f': load_raw.qs_export.sleep_analysis
            }, 'steps': {
                'db_entry_keys': [
                    'date', 'start', 'end', 'steps'
                ],  # 'f': load_raw.qs_export.steps
                # }, 'exercise minutes': {
                #     'db_entry_keys': ['date', 'start', 'end', 'exercise minutes'],
                #     # 'f': load_raw.qs_export.exercise_minutes
                # }, 'stand hours': {
                #     'db_entry_keys': ['date', 'start', 'end', 'stand hours'],
                #     # 'f': load_raw.qs_export.stand_hours
                # }, 'mindful minutes': {
                #     'db_entry_keys': ['date', 'start', 'end', 'mindful minutes'],
                #     # 'f': load_raw.qs_export.mindful_minutes
            },
        }, 'sleep_cycle': {  # daily
            'db_entry_keys': [
                'date', 'start', 'end', 'sleep quality',
                'hours in bed', 'wake-up mood'
            ], 'f': load_raw.sleep_cycle.sleep_history,
        }
    }, 'stats': {
        'time series': {
            'daily': {
                'health': {
                    'activity': {
                        'db_entry_keys': {
                            # 'active calories': ['date', 'active calories'],
                            # 'cycling distance': ['date', 'cycling distance'],
                            # 'distance': ['date', 'distance'],
                            # 'heart rate': ['date', 'heart rate'],
                            # 'whether I worked out': ['date', 'whether I worked out'],
                            # 'at least one pull-up/push-up': ['date', 'at least one pull-up/push-up'],
                            'whether I played ping pong': ['date', 'whether I played ping pong'],
                            'whether I took a walk': ['date', 'whether I took a walk'],
                            # 'small bike tour': ['date', 'small bike tour'],
                            # 'large bike tour': ['date', 'large bike tour'],
                            # ...
                        }, 'f': stats.time_series.activity
                    }, 'diet': {
                        'db_entry_keys': {
                            # 'whether I ate meat': ['date', 'whether I ate meat'],
                            # 'whether I ate fish': ['date', 'whether I ate fish'],
                            'whether I ate vegetarian': ['date', 'whether I ate vegetarian'],
                            'whether I ate vegan': ['date', 'whether I ate vegan'],
                            'whether I ate late': ['date', 'whether I ate late'],
                            'whether I cooked for myself': ['date', 'whether I cooked for myself'],
                            'whether I went to bed hungry': ['date', 'whether I went to bed hungry'],
                            'whether I ate nothing all day': ['date', 'whether I ate nothing all day'],
                            'whether I drank tea': ['date', 'whether I drank tea'],
                            'whether dinner was great': ['date', 'whether dinner was great'],
                        }, 'f': stats.time_series.diet  # TODO: rename
                    }, 'drug consumption': {
                        'db_entry_keys': {
                            'whether I consumed alcohol': ['date', 'whether I consumed alcohol'],
                            'whether I consumed caffeine': ['date', 'whether I consumed alcohol'],
                            'whether I consumed mdma': ['date', 'whether I consumed mushrooms'],
                            # 'whether I consumed mushrooms': ['date', 'whether I consumed mushrooms'],
                            # 'whether I consumed N2O': ['date', 'whether I consumed mushrooms'],
                            # 'whether I consumed poppers': ['date', 'whether I consumed alcohol'],
                            'whether I consumed tabacco': ['date', 'whether I consumed tabacco'],
                            'whether I consumed weed': ['date', 'whether I consumed weed'],
                            'whether I drank a bit of schnaps': ['date', 'whether I drank a bit of schnaps'],
                            # 'whether I drank a lot of schnaps': ['date', 'whether I drank a lot of schnaps'],
                            'whether I drank a bit of beer': ['date', 'whether I drank a bit of beer'],
                            'whether I drank a lot of beer': ['date', 'whether I drank a lot of beer'],
                            'whether I smoked cigarettes': ['date', 'whether I smoked cigarettes'],
                            # amount of weed (1, 2-3, 4+)
                        }
                    }, 'hygiene': {
                        'db_entry_keys': {
                            'whether I took a shower': ['date', 'whether I took a shower'],
                            'whether I took a bath': ['date', 'whether I took a bath'],
                            'whether I wore braces at night': ['date', 'whether I wore braces at night'],
                        }
                        # ...
                    }, 'sickness': {
                        'db_entry_keys': {
                            'whether I had back pain': ['date', 'whether I had back pain'],
                            'whether I was sick': ['date', 'whether I was sick'],
                            'whether I puked': ['date', 'whether puked'],
                            'whether I had a headache': ['date', 'whether I had a headache'],
                            'whether I had dry hands': ['date', 'whether I had dry hands'],
                            # fnail?
                        }
                    }, 'sleep analysis': {
                        'db_entry_keys': {
                            'hours in bed': ['date', 'hours in bed'],
                            #     'hours asleep': ['date', 'hours asleep'],
                            # 'minutes to fall asleep': ['date', 'minutes to fall asleep'],
                            'sleep quality': ['date', 'sleep quality'],
                            'wake-up mood': ['date', 'wake-up mood'],
                            # 'get-up time': ['date', 'get-up time'],
                            # 'go-to-bed time': ['date', 'go-to-bed time'],
                            # 'nr of sleep cycles': ['date', 'nr. of sleep cycles'],
                            'whether I went to bed tired': ['date', 'whether I went to bed tired'],
                            'sleep regularity': ['date', 'sleep regularity'],
                            'whether I listened to music before sleep': ['date', 'whether I listened to music before sleep'],
                            # 'bright screen before sleep': ['date', 'bright screen before sleep'],
                            # 'meditation before sleep': ['date', 'meditation before sleep'],
                            # 'on reddit/youtube before sleep': ['date', 'on reddit/youtube before sleep'],
                            # 'read a few pages before sleep': ['date', 'read a few pages before sleep'],
                            'whether I slept in a tent': ['date', 'whether I slept in a tent'],
                            # 'sleeping under clear sky': ['date', 'sleeping under clear sky'],
                            # nap?
                        }, 'f': stats.time_series.health.sleep_analysis
                    },
                    # }, 'learning': {
                    # 'various': {
                    # 'db_entry_keys': {
                    # 'wrote code': ['date', 'wrote code'],
                    # 'anki/duolingo': ['date', 'anki/duolingo'],
                    # }
                    # }
                    # }, 'social': {
                    # 'instant messaging': {
                    # 'db_entry_keys': {
                    # 'messages sent': ['date', 'contact name', 'messages sent', 'messenger'],
                    # 'messages received': ['date', 'contact name', 'messages received', 'messenger'],
                    # 'messages total': ['date', 'contact name', 'messages total', 'messenger'],
                    # 'nr. of characters sent': ['date', 'contact name', 'nr. of characters sent', 'messenger'],
                    # 'nr. of characters received': ['date', 'contact name', 'nr. of characters received', 'messenger'],
                    # 'nr. of characters total': ['date', 'contact name', 'nr. of characters total', 'messenger'],
                    # },
                    # }
                }, 'personal': {
                    'mood': {
                        'db_entry_keys': {
                            # 'great mood': ['date', 'great mood'],
                            # 'happy mood': ['date', 'happy mood'],
                            # 'weird mood': ['date', 'weird mood'],
                            # 'days with sad mood': ['date', 'days with sad mood'],
                            'good day': ['date', 'good day'],
                            'bad day': ['date', 'bad day'],
                            'sad day': ['date', 'sad day'],
                            'stressful day': ['date', 'stressful day'],
                            'whether I think life is great': ['date', 'whether I think life is great'],
                        }
                    }, 'sexual': {
                        'db_entry_keys': {
                            'whether I had sex': ['date', 'whether I had sex'],
                            'whether I fapped': ['date', 'whether I fapped'],
                            'whether I slept in bed with Selina': ['date', 'whether I slept in bed with Selina'],
                        }
                    }, 'location': {
                        'db_entry_keys': {
                            'whether I was in Ulm': ['date', 'whether I was in Ulm'],
                            'whether I was in Heidelberg': ['date', 'whether I was in Heidelberg'],
                            # 'whether I was in Berlin': ['date', 'whether I was in Berlin'],
                        }
                    }
                    # }, 'various': {
                    #     'finances': {
                    #         'db_entry_keys': {
                    # 'income': ['date', 'income'],
                    # 'expenses': ['date', 'expenses'],
                    # 'net worth': ['date', 'net worth'],
                }, 'various': {
                    'various': {
                        'db_entry_keys': {
                            'whether it was weekend': ['date', 'whether it was weekend'],
                            'whether it was spring': ['date', 'whether it was spring'],
                            'whether it was summer': ['date', 'whether it was summer'],
                            'whether it was fall': ['date', 'whether it was fall'],
                            'whether it was winter': ['date', 'whether it was winter'],
                            # semester / school / vacation
                        }, 'f': stats.time_series.various.main
                    }
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
}

MDB_TS_CATEGORIES = []
count = 0
for k, v in MDB_HIERARCHY['stats']['time series']['daily'].items():
    for k2, v2 in v.items():
        for k3, v3 in v2['db_entry_keys'].items():
            MDB_TS_CATEGORIES.append(k3)
