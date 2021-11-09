from datetime import datetime as dt
from datetime import timedelta as td

import matplotlib.pyplot as plt
import yfinance as yf


def main(
    tickerSymbol,
    start=dt(1998, 7, 11), end=dt.now()
):
    # get the historical prices for ticker
    tickerData = yf.Ticker(tickerSymbol)
    tickerDf = tickerData.history(
        period='1d', start=start, end=end
    )
    # get prices at close
    close_prices = [i for i in tickerDf['Close']]

    dates = [start]
    for close_price in close_prices[1:]:
        dates.append(dates[-1] + td(days=1))

    return dates, close_prices
