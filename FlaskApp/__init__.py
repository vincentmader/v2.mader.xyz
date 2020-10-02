import flask
from flask import Flask, render_template, send_from_directory


app = Flask(__name__)


# @app.route('/')
# def chart():
#     return render_template('chart.html', **{'ayy': 'ayy'})

@app.route('/')
def index():
    return render_template('index.html')


@app.route('/desert_game/')
def desert_game():
    return render_template('desert_game.html')


@app.route('/solar_system/')
def solar_system():
    return render_template('solar_system.html')


@app.route('/balls/')
def balls():
    return render_template('balls.html')


@app.route('/molecules/')
def molecules():
    return render_template('molecules.html')


if __name__ == '__main__':
    app.run()
