from datetime import datetime as dt


def main(x, y):

    new_x, new_y = [], []

    for idx, item in enumerate(x):
        xi = item  # dt obj
        yi = y[idx]

        year_dt = dt.strptime(xi.strftime('%Y'), '%Y')

        if year_dt in new_x:
            jdx = new_x.index(year_dt)
            new_y[jdx] += yi
        else:
            new_x.append(year_dt)
            new_y.append(yi)

    return new_x, new_y
