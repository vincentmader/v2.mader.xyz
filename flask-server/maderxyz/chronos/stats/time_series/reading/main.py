import json

import matplotlib.pyplot as plt

# from FlaskApp.chronos.utils import apply_pyplot_darkmode


def main():

    # get data
    books_read_per_year = {
        '2013': 1,  '2014': 5,
        '2015': 35, '2016': 16,
        '2017': 12, '2018': 17,
        '2019': 4
    }
    # with open('/Users/meister/PyCharmProjects/Chronos/data/extracted/reading/books_read_per_year.json') as fp:
    #     books_read_per_year = json.load(fp)

    x = [int(year) for year in books_read_per_year.keys()]
    y = [books_read_per_year[str(year)] for year in x]

    # plot
    fig = plt.figure(figsize=(9, 5))
    plt.title('books read per year')
    plt.bar(
        x, y,
        color='white',
        edgecolor='white', linewidth=4
    )
    plt.xticks()
    plt.yticks()
    # plt.xticks(size=35)
    # plt.yticks(size=35)

    # apply_pyplot_darkmode()

    plt.savefig('./static/media/pyplots/books_read_per_year.png')
    plt.savefig('./static/media/pyplots/books_read_per_year.pdf')
    plt.close()
