import datetime
from datetime import datetime as dt
from datetime import timedelta as td
import math

import matplotlib.cm as cm
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd
import scipy
from scipy import stats
from tqdm import tqdm

from FlaskApp.chronos import load_raw, stats
from FlaskApp.chronos.io import save_to_database
from FlaskApp import config, db_config


def prepare_datasets(rules):

    # def reshape_df(df_resolution, out_resolution):
    # if df_resolution == 'high precision': # TODO: rename to 'continuous'?
    #     pass # TODO: reshape from higher precisions to daily
    # elif df_resolution == 'daily':
    #     pass # TODO: only do something if
    # elif df_resolution == 'monthly':
    #     pass  # TODO: think re: what to do about monthly data?
    # return df

    collection = config.MDB['stats']['time series']

    out_df = 0
    for ts in collection.find(rules):
        title = ts['title']
        df = pd.DataFrame({
            'timestamp': ts['timestamps'],
            title: ts['values']
        })
        # print(df.dropna())
        # print(df)
        # input()

        if type(out_df) == int:
            out_df = df
        else:
            # out_df = out_df.join(df, on='timestamp', how='left')
            # out_df = out_df.set_index('timestamp').join(
            #     df.set_index('timestamp'))
            # out_df = df.join(out_df, on='timestamp')
            # out_df = df.join(out_df.set_index('timestamp'), on='timestamp')
            # out_df = out_df.join(df, how='outer')
            out_df = pd.merge(
                out_df, df,
                # left_index=True, right_index=True,
                how='outer',
                on='timestamp'
            )

        # print(out_df)
        # input()

    return out_df


def calculate_correlation(df, col_id, col_jd):

    # print(col_id, col_jd)

    def time_to_float(time, rules=[]):
        h = time.hour
        m = time.minute
        s = time.second
        time = h + m/60. + s/3600.
        if 'bed time' in rules and time < 12:
            time += 24
        return time

    # get rid of all rows where either col1 or col2 has empty cell
    new_df = df.dropna()
    col_i, col_j = new_df[col_id], new_df[col_jd]
    if len(new_df.index) < 2:
        return 0, 0  # TODO: rather return None, None?

    cols = [col_i, col_j]
    col_ids = [col_id, col_jd]

    # check whether one of the columns is constant (-> correlation undef.)
    for col_id, col in zip(col_ids, cols):
        if all(x == col.iloc[0] for x in col):
            # if constant -> return
            return 0, 0  # TODO: rather return None, None?

    # convert all cells to numerical values
    for col_idx, col in enumerate(cols):
        # dates -> convert to UNIX timestamp
        if col.dtypes == np.dtype('datetime64[ns]'):
            cols[col_idx] = [i.timestamp() for i in col]
        # times of day -> convert to hours after midnight
        else:
            for i in col:
                if type(i) == datetime.time:
                    rules = ['bed time'] if col_ids[col_idx] == 'bed time' else []
                    cols[col_idx] = [
                        time_to_float(i, rules=rules) for i in col
                    ]
                    break
    # get correlation coefficients and return
    corr = scipy.stats.stats.pearsonr(*cols)
    if np.isnan(corr[0]):
        return 0, 0  # TODO
    else:
        return corr


def get_correlation_matrix(df):

    nr_of_cols = len(df.columns)
    correlation_matrix = np.zeros(shape=(nr_of_cols, nr_of_cols))
    error_matrix = np.zeros(shape=(nr_of_cols, nr_of_cols))
    col_ids = []

    # nr_of_ts = len(df.columns)
    # print(nr_of_ts, nr_of_ts**2)

    already_checked = []
    for idx, col_id in tqdm(enumerate(df)):
        col = df[col_id]
        col_ids.append(col_id)
        for jdx, col_jd in enumerate(df):
            # print(col_id, col_jd)
            # skip diagonal & lower triangle of correlation matrix
            if col_id == col_jd or col_jd in already_checked:
                continue
            other_col = df[col_jd]
            # calculate correlation
            try:
                # print(df[col_id])
                # print(f'{col_id} vs. {col_jd}')
                # print(df[col_id].shape[0])
                corr, err = calculate_correlation(
                    df[[col_id, col_jd]], col_id, col_jd
                )
                corr, err = round(corr, 3), round(err, 4)
                # print(f'{col_id} vs. {col_jd}: ', corr, err)
                # print(corr, err)
                # input()
                correlation_matrix[idx][jdx] = corr
                error_matrix[idx][jdx] = err
            # except PearsonRConstantInputWarning as w:
            #     pass
            except Exception as e:
                print('  ERROR: ', col_id, col_jd)
                # continue
                raise e

        already_checked.append(col_id)

    return correlation_matrix


def plot_correlation_matrix(df, correlation_matrix):
    cols = [i for i in df.columns]
    nr_of_cols = len(cols)

    plt.figure(figsize=(6, 6))
    plt.gca().set_aspect('equal', adjustable='box')
    plt.set_cmap('bwr')
    plt.pcolor(correlation_matrix[::-1], vmin=-1, vmax=1)
    plt.gca().xaxis.tick_top()
    plt.gca().yaxis.tick_right()
    plt.xticks(np.arange(.5, nr_of_cols+.5, 1), cols, rotation=90)
    plt.yticks(np.arange(.5, nr_of_cols+.5, 1), cols[::-1])
    # plt.xlim(0, len(correlation_matrix)-1)
    # plt.ylim(0, len(correlation_matrix)-1)
    plt.colorbar(orientation='horizontal')
    plt.savefig('correlations.png')
    plt.show()


def main():

    rules = dict(resolution='daily')
    df = prepare_datasets(rules)
    correlation_matrix = get_correlation_matrix(df)

    document = {
        'correlation matrix': [list(i) for i in correlation_matrix],
        'date': dt.now().strftime('%Y-%m-%d')
    }
    path = ['stats', 'correlations']
    save_to_database(document, path=path)

    plot_correlation_matrix(df, correlation_matrix)
