import flask
from flask import Flask, render_template, send_from_directory


app = Flask(__name__)


# @app.route('/')
# def chart():
#     return render_template('chart.html', **{'ayy': 'ayy'})

@app.route('/')
def index():
    return render_template('index.html')


@app.route('/tatooine/')
def tatooine():
    return render_template('tatooine.html')


@app.route('/orbit/')
def orbit():
    return render_template('orbit.html')


@app.route('/balls/')
def balls():
    return render_template('balls.html')


@app.route('/boltzmann/')
def boltzmann():
    return render_template('boltzmann.html')


@app.route('/spaceship/')
def spaceship():
    return render_template('spaceship.html')


if __name__ == '__main__':
    app.run()
