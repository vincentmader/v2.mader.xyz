from datetime import datetime as dt
from datetime import timedelta as td


def main(start_date, end_date):
    dic = {}
    while start_date < end_date:
        dic[start_date] = None
        start_date += td(days=1)
    return dic
