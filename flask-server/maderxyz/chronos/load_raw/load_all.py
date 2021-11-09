from tqdm import tqdm

from .daily_log import daily_log_history  # {date: content}
from .facebook import chat_history as facebook_chat_history
from .facebook import comment_history as facebook_comment_history
from .facebook import friends_history as facebook_friends_history
from .facebook import like_history as facebook_like_history
from .facebook import login_history as facebook_login_history
from .facebook import poke_history as facebook_poke_history
from .facebook import profile_update_history as facebook_profile_update_history
from .facebook import search_history as facebook_search_history
from .google_sheets import daily_goal_history, time_spent_history
from .google_sheets import expenditure_history, income_history  # TODO: reading
from .google_takeout import google_maps_review_history, chrome_browser_history
from .google_takeout import google_maps_saved_places_history
from .google_takeout import location_history as google_maps_location_history
from .google_takeout import youtube_comment_history, youtube_playlist_history
from .google_takeout import youtube_search_history, youtube_watch_history
from .icloud import mail_history as icloud_mail_history
from .life_cycle import life_cycle_history
from .photos import photos_taken_history
from .qs_export import cycling_distance as qs_cycling_distance
from .qs_export import distance as qs_distance
from .qs_export import swimming_distance as qs_swimming_distance
from .qs_export import steps as qs_steps
from .qs_export import active_calories as qs_active_calories
from .qs_export import exercise_minutes as qs_exercise_minutes
from .qs_export import stand_hours as qs_stand_hours
from .qs_export import flights_climbed as qs_flights_climbed
from .qs_export import mindful_minutes as qs_mindful_minutes
from .qs_export import sleep_analysis as qs_sleep_analysis
from .qs_export import heart_rate as qs_heart_rate
from .qs_export import heart_rate_at_rest as qs_heart_rate_at_rest
from .qs_export import heart_rate_variability as qs_heart_rate_variability
from .qutebrowser import browser_history as qute_browser_history
from .safari import browser_history as safari_browser_history
from .school import grades as school_grades
from .skype import chat_history as skype_chat_history
from .sleep_cycle import sleep_history as sc_sleep_history
from .sms import chat_history as sms_chat_history
from .spotify import library_history as spotify_library_history
from .spotify import play_history as spotify_play_history
from .spotify import search_history as spotify_search_history
from .uni import grades as uni_grades
from .whatsapp import chat_history as whatsapp_chat_history


funcs = {
    'daily_log': daily_log_history,
    # 'facebook_chat_history': facebook_chat_history,
    # 'facebook_comment_history': facebook_comment_history,
    # 'facebook_friends_history': facebook_friends_history,
    # 'facebook_like_history': facebook_like_history,
    # 'facebook_login_history': facebook_login_history,
    # 'facebook_poke_history': facebook_poke_history,
    # 'daily_goal_history': daily_goal_history,
    # 'time_spent_history': time_spent_history,
    # 'expenditure_history': expenditure_history,
    # 'income_history': income_history,
    # 'chrome_browser_history': chrome_browser_history,
    # 'google_maps_location_history': google_maps_location_history,
    # 'google_maps_review_history': google_maps_review_history,
    # 'google_maps_saved_places_history': google_maps_saved_places_history,
    # 'youtube_comment_history': youtube_comment_history,
    # 'youtube_playlist_history': youtube_playlist_history,
    # 'youtube_search_history': youtube_search_history,
    # 'youtube_watch_history': youtube_watch_history,
    # 'icloud_mail_history': icloud_mail_history,
    # 'life_cycle_history': life_cycle_history,
    # 'photos_taken_history': photos_taken_history,
    # 'qs active calories': qs_active_calories,
    # 'qs cycling distance': qs_cycling_distance,
    # 'qs distance': qs_distance,
    # 'qs flights climbed': qs_flights_climbed,
    # 'qs heart rate': qs_heart_rate,
    # 'qs steps': qs_steps,
    # 'qs heart rate_variability': qs_heart_rate_variability,
    # 'qs heart rate_at_rest': qs_heart_rate_at_rest,
    # 'qs mindful minutes': qs_mindful_minutes,
    # 'qs sleep analysis': qs_sleep_analysis,
    # 'qs swimming distance': qs_swimming_distance,
    # 'qs exercise minutes': qs_exercise_minutes,
    # 'qs stand hours': qs_stand_hours,
    # 'qute_browser_history': qute_browser_history,
    # 'safari_browser_history': safari_browser_history,
    # 'school_grades': school_grades,
    # 'skype_chat_history': skype_chat_history,
    'sc_sleep_history': sc_sleep_history,
    # 'sms_chat_history': sms_chat_history,
    # 'spotify_library_history': spotify_library_history,
    # 'spotify_play_history': spotify_play_history,
    # 'spotify_search_history': spotify_search_history,
    # 'uni_grades': uni_grades,
    # 'whatsapp_chat_history': whatsapp_chat_history,
}


def main():
    for f_name, f in funcs.items():
        # print(f_name, ' ', end='')
        print(f_name)
        a = f()
        # print(len(a) if a is not None else 0)
