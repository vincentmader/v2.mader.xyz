from datetime import datetime as dt
import json
import os

from FlaskApp.config import PATH_TO_RAW_DATA as PRD
from FlaskApp import chronos
from FlaskApp.chronos.io import save_to_database


def main():

    PATH_TO_SPOTIFY_DATA = os.path.join(PRD, 'spotify')

    # play_history = []
    timestamps_of_already_added_datapoints = []

    for export_directory in sorted(os.listdir(PATH_TO_SPOTIFY_DATA)):
        if export_directory == '.DS_Store':
            continue
        path_to_export_directory = os.path.join(
            PATH_TO_SPOTIFY_DATA, export_directory
        )
        for export_file in os.listdir(path_to_export_directory):
            if not export_file.startswith('StreamingHistory'):
                continue
            path_to_export_file = os.path.join(
                PATH_TO_SPOTIFY_DATA, export_directory, export_file
            )

            with open(path_to_export_file) as fp:
                content = json.load(fp)

            for datapoint in content:

                play_end_dt = dt.strptime(
                    datapoint['endTime'], '%Y-%m-%d %H:%M'
                )
                if play_end_dt in timestamps_of_already_added_datapoints:
                    continue
                date = play_end_dt

                artist_name = datapoint['artistName']
                track_name = datapoint['trackName']
                ms_played = datapoint['msPlayed']

                document = {
                    'date': date,
                    'artist name': artist_name,
                    'track name': track_name,
                    'time played [s]': ms_played / 1000.

                }
                path = ['raw data', 'spotify', 'artist name']
                save_to_database(document, path=path)

                # play_history.append({
                #     'play_end_dt': play_end_dt,
                #     'artist_name': artist_name,
                #     'track_name': track_name,
                #     'ms_played': ms_played
                # })

    # return play_history
