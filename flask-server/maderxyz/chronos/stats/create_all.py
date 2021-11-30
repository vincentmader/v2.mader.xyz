from maderxyz import config

from .time_series import health


# {
#     'health': {
#         'activity': {
#             {'active calories': {'date': [], 'active calories': []}},
#             {'steps': {'date': [], 'steps': []}},
#             ...
#         }, 'diet': {

#         }, 'drug consumption': {

#         }, 'sleep analysis': {

#         }
#     }, 'social': {

#     }, 'web history': {
#         'browser history': {'date': [], 'page': [], 'browser': []}
#     }
# }


# from .time_series import


def main():
    health.activity()

    # db = config.MDB['stats']['time series']['daily']

    # HEALTH

    # db['health']['sleep analysis'].insert_one({
    #    'time at which I got up': get_up_times,  # TODO: val is dataframe
    #    'time at which I went to bed': to_bed_times,
    #    'sleep quality': sleep_qualities,
    #    'sleep duration': sleep_durations,  # TODO: get from QS
    #    'nr of sleep cycles': nr_of_sleep_cycles,  # TODO: get from QS
    #    'time to fall asleep': nr_of_sleep_cycles,  # TODO: get from QS
    # })

    # db['health']['drug consumption'].insert_one({
    #    'days on which I consumed caffeine': days_on_which_I_consumed_caffeine,
    #    'days on which I consumed mdma': days_on_which_I_consumed_mdma,
    #    'days on which I consumed poppers': days_on_which_I_consumed_poppers,
    #    'days on which I consumed tabacco': days_on_which_I_consumed_tabacco,
    #    'days on which I consumed weed': days_on_which_I_consumed_weed,
    # })

    # db['health']['diet'].insert_one({
    #    'days on which I ate meat': days_on_which_I_ate_meat,
    #    'days on which I ate fish': days_on_which_I_ate_fish,
    #    'days on which I ate vegan': days_on_which_I_ate_vegan,
    #    'days on which I ate vegetarian': days_on_which_I_ate_vegetarian,
    # })

    # SOCIAL (TODO: rename to IM?)

    # db['social']['skype'].insert_one({
    #    'total messages': messages,  # dataframe with cols for contact
    #    'total messages sent': messages_sent,
    #    'total messages received': messages_received,
    #    'total characters': characters,
    #    'total characters sent': characters_sent,
    #    'total characters received': characters_received,
    # })
    # db['social']['whatsapp'].insert_one({
    #    'total messages': messages,
    #    'total messages sent': messages_sent,
    #    'total messages received': messages_received,
    #    'total characters': characters,
    #    'total characters sent': characters_sent,
    #    'total characters received': characters_received,
    # })
    # db['social']['facebook'].insert_one({
    #    'total messages': messages,
    #    'total messages sent': messages_sent,
    #    'total messages received': messages_received,
    #    'total characters': characters,
    #    'total characters sent': characters_sent,
    #    'total characters received': characters_received,
    # })
    # db['social']['sms'].insert_one({
    #    'total messages': messages,
    #    'total messages sent': messages_sent,
    #    'total messages received': messages_received,
    #    'total characters': characters,
    #    'total characters sent': characters_sent,
    #    'total characters received': characters_received,
    # })
    # db['social']['mail'].insert_one({
    #    'total messages': messages,
    #    'total messages sent': messages_sent,
    #    'total messages received': messages_received,
    #    'total characters': characters,
    #    'total characters sent': characters_sent,
    #    'total characters received': characters_received,
    # })

    # WEB HISTORY

    # db['web history']['browsing'].insert_one({
    #    # 'visits for {s}': visits(s) for s in top_sites_visited,
    # })

    # db['web history']['youtube'].insert_one({
    #    # 'searches for {p}': searches(p) for p in top_search_phrases,
    #    # 'watches for {p}': watches(p) for p in top_search_phrases,
    # })

    # db['web history']['google search'].insert_one({
    #    # 'searches for {p}': searches(p) for p in TOP_SEARCH_PHRASES,
    # })

    ##

    # db['spotify'].insert_one({
    #    # f'plays of {s}': plays(s) for s in top_songs,
    #    # f'plays of {a}': plays(a) for a in top_albums,
    #    # f'plays of {a}': plays(a) for a in top_artists,
    #    # f'plays of {g}': plays(g) for g in top_genres,
    #    'total playtime': total_playtime,
    #    # 'average listening duration': average_listening_duration,
    # })

    # db['daily log'].insert_one({
    #    'length of daily log (chars)': nr_of_characters_in_daily_log,
    #    'length of daily log (words)': nr_of_words_in_daily_log,
    #    # f'mentions of {c}': mentions(c) for c in CONTACTS,
    #    # f'mentions of {p}': mentions(p) for p in KEY_PHRASES,
    #    'mood (general)': mood_general,
    #    'mood (uni)': mood_un,
    #    'mood (social)': mood_so,
    #    'mood (relationship)': mood_gf,
    # })

    ##

    # db['time spent per category'].insert_one({
    #    'le': hours_per_tag_le,
    #    'un': hours_per_tag_un,
    #    'wo': hours_per_tag_wo,
    #    're': hours_per_tag_re,
    #    'pr': hours_per_tag_pr,
    #    'me': hours_per_tag_me,
    #    'pi': hours_per_tag_pi,
    #    'ex': hours_per_tag_ex,
    #    'tv': hours_per_tag_tv,
    #    'do': hours_per_tag_do,
    #    'gs': hours_per_tag_gs,
    #    'ga': hours_per_tag_ga,
    #    'wa': hours_per_tag_wa,
    #    'ma': hours_per_tag_ma,
    #    'fa': hours_per_tag_fa,
    #    'gf': hours_per_tag_gf,
    #    'sx': hours_per_tag_sx,
    #    'so': hours_per_tag_so,
    #    'sp': hours_per_tag_sp,
    #    'sl': hours_per_tag_sl,
    # })
