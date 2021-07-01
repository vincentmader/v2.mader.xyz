from datetime import datetime as dt
from datetime import timedelta as td

import matplotlib.pyplot as plt
import numpy as np

from FlaskApp import chronos
from FlaskApp import config, db_config


# def load_all_raw():
#     print('load_raw')
#     for k, v in db_config.MDB_HIERARCHY['raw data'].items():
#         if 'f' in v.keys():
#             print('  ' + k)  # TODO: print all skipped
#             v['f']()
#         else:
#             print('  ' + k)  # TODO: print all skipped
#             for k2, v2 in db_config.MDB_HIERARCHY['raw data'][k].items():
#                 if type(v2) is not dict:
#                     continue
#                 if 'f' in v2.keys():
#                     print('    ' + k2)
#                     v2['f']()
#                 else:
#                     print('    (skipped) ' + k2)


# def create_all_stats():
#     print('stats')
#     for k, v in db_config.MDB_HIERARCHY['stats']['time series']['daily'].items():
#         print('  ' + k)
#         for k2, v2 in v.items():
#             if 'f' in v2.keys():
#                 print('    ' + k2)
#                 v2['f']()
#             else:
#                 print('    (skipped) ' + k2)


if __name__ == '__main__':

    # LOAD RAW DATA INTO DB
    # =====================

    chronos.load_raw.load_all()

    # chronos.load_raw.qs_export.active_calories()
    # chronos.load_raw.qs_export.cycling_distance()
    # chronos.load_raw.qs_export.distance()
    # chronos.load_raw.qs_export.flights_climbed()
    # chronos.load_raw.qs_export.heart_rate()
    # chronos.load_raw.qs_export.stand_hours()
    # chronos.load_raw.qs_export.steps()

    # ---

    # chronos.load_raw.daily_log.daily_log_history()
    # chronos.load_raw.facebook.chat_history()
    # ! chronos.load_raw.facebook.comment_history()
    # ! chronos.load_raw.facebook.friends_history()
    # ! chronos.load_raw.facebook.like_history()
    # ! chronos.load_raw.facebook.login_history()
    # ! chronos.load_raw.facebook.poke_history()
    # ! chronos.load_raw.facebook.profile_update_history()
    # ! chronos.load_raw.facebook.search_history()

    # chronos.load_raw.sleep_cycle.sleep_history()

    # CREATE STATISTICS
    # =================

    # - time series
    # mdb_hierarchy = db_config.MDB_HIERARCHY['stats']['time series']['daily']
    # for cat in mdb_hierarchy.keys():
    #     for subcat in mdb_hierarchy[cat].keys():
    #         chronos.stats.time_series.sleep_analysis.daily_sleep_notes(
    #             cat, subcat
    #         )
    # chronos.stats.time_series.health.sleep_analysis()
    # chronos.stats.time_series.various.main()

    # - correlations
    # chronos.stats.correlation_finder.main()

    # ======

    # load_all_raw()
    # create_all_stats()

    #

    #

    # LOAD RAW DATA

    # chronos.load_raw.load_all()

    # chronos.stats.create_all()

    # chronos.load_raw.facebook.chat_history()

    # chronos.load_raw.google_sheets.daily_goal_history()
    # chronos.load_raw.google_sheets.expenditure_history
    # chronos.load_raw.google_sheets.income_history()
    # chronos.load_raw.google_sheets.time_spent_history()
    # chronos.load_raw.google_takeout.location_history()          # ready

    # chronos.load_raw.skype.chat_history()white

    # chronos.load_raw.sleep_cycle.sleep_history()                # ready

    # chronos.load_raw.spotify.play_history()                     # ready
    # chronos.load_raw.spotify.library_history()
    # chronos.load_raw.spotify.search_history()

    # chronos.load_raw.whatsapp.chat_history()                    # ready

    # chronos.load_raw.sms.chat_history()                         # ready

    # CREATE STATISTICS

    # data = chronos.stats.sample_data()
    # time_series = chronos.stats.sleep_analysis.daily_time_went_to_bed()
    # time_series = chronos.stats.sleep_analysis.daily_time_got_up()
    # time_series = chronos.stats.social.word_occurences_in_IM(
    #     'pogli', chat_name='Selina Patent', msg_type='all'
    # )

    # ==========================================================================
    # N = 25
    # x, y = chronos.stats.time_series.spotify.total_playtime()
    # plt.figure(figsize=(10, 5))
    # print(len(y))
    # x2, y2 = chronos.stats.time_series.reshape.into_monthly_time_series(x, y)
    # plt.bar(x2, height=np.array(y2) / 3.6e6, width=td(days=28))
    # x1, y1 = chronos.stats.time_series.reshape.into_daily_time_series(x, y)
    # x1, y1 = chronos.stats.time_series.reshape.into_N_day_moving_average(x1, y1, N)
    # plt.plot(x1[:-int(N/2)], np.array(y1[int(N/2):]) / 3.6e6, color='black')
    # plt.ylabel('playtime in hours')
    # plt.xlabel('time')
    # plt.xlim(x[0], dt.now())
    # plt.ylim(0, )
    # x2, y2 = chronos.stats.time_series.reshape.into_daily_time_series(x, y)
    # plt.plot(x2, y2)
    # plt.savefig('./static/media/pyplots/test_spotify.pdf')
    # plt.savefig('./static/media/pyplots/test_spotify.png')
    # plt.close()
    # ==========================================================================

    # plot daily goals
    # plt.figure(figsize=(10, 5))
    # time_series = chronos.stats.time_series.daily_goals.from_gsheets_and_sleepcycle()
    # x = time_series['timestamp']
    # d = [dt.fromtimestamp(i) for i in x]

    # for k in time_series.keys():
    #     if k in ['timestamp'] + [
    #         'reading', 'exercise', 'meditation', 'no youtube', 'anki'
    #     ]:
    #         continue
    #     y = time_series[k]
    #     print(len(y))
    #     print(len(x))
    #     _, y = chronos.stats.time_series.reshape.into_N_day_moving_average(x, y, N)
    #     # x, y = chronos.stats.time_series.reshape.into_yearly_time_series(x, y)
    #     plt.plot(d, y, label=f'{k}')

    # plt.xlim(d[0], dt.now())
    # plt.ylim(0, N)
    # plt.legend()
    # plt.savefig('./test.pdf')

    # CREATE PLOTS

    # chronos.plots.bokeh.basic.bar_chart(data)
    # chronos.plots.bokeh.basic.heatmap(data, 'daily')
    # chronos.plots.bokeh.basic.line_chart(data)
    # chronos.plots.bokeh.basic.pie_chart(data)
    # chronos.plots.bokeh.sleep_analysis.sleep_snake()

    # chronos.plots.pyplot.sleep_analysis.sleep_snake()

    # chronos.tests.ayyyyy.test()

    # a = chronos.load_raw.daily_log.daily_log_history()
    # print(a)

    # chronos.tests.load_finances()

    # chronos.stats.time_series.location_history.coordinates()

    # chronos.plots.bokeh.google_takeout.gmap()

    # =========================================================

    # chronos.load_raw.daily_log.daily_log_history()
    # chronos.stats.time_series.reading.main()

    # chronos.plots.pyplot.various.life_remaining(life_expectancy_in_years=90)
    # chronos.plots.pyplot.grades.foo()
    # chronos.plots.pyplot.spotify.foo()

    # =========================================================
    # chronos.tests.load_finances()
    # chronos.tests.plot_stocks()
    # chronos.stats.correlation_finder.main()
