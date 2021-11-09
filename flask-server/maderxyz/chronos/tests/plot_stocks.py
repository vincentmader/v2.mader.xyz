import matplotlib.pyplot as plt

from FlaskApp.chronos.tests.load_stocks import main as load_stocks


def main():

    tickerSymbols = [
        # 'TSLA', 'NKLA', 'AMZN',
        # 'BB',
        'GME'
    ]

    for t in tickerSymbols:
        dates, close_prices = load_stocks(t)
        plt.plot(dates, close_prices)

    plt.savefig('stocks.pdf')
