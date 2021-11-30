import pandas as pd

from maderxyz.chronos import load_raw


def main():
    play_history = load_raw.spotify.play_history()

    x, y = [], []
    for entry in play_history:
        play_end_dt = entry['play_end_dt']
        ms_played = entry['ms_played']

        x.append(play_end_dt)
        y.append(ms_played)

    foo = {y[idx]: x[idx] for idx in range(len(x))}
    bar = sorted(foo, key=foo.get, reverse=False)
    x = [foo[i] for i in bar]
    y = bar

    return x, y
    # return {
    #     'data': pd.DataFrame({
    #         'measurement time': x, 'playtime': y
    #     })
    # }
