from datetime import datetime as dt


def main(x, y, N):
    new_x, new_y = [], []

    for idx, xi in enumerate(x):

        sigma = 0
        for i in range(N):
            try:
                sigma += y[idx - i]
            except KeyError:
                pass
            except TypeError:
                pass

        new_x.append(xi)
        new_y.append(sigma)

    return new_x, new_y
