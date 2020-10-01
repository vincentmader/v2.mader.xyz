import flask
from flask import Flask, render_template, send_from_directory


app = Flask(__name__)


@app.route('/')
def chart():
    return render_template('chart.html', **{'ayy': 'ayy'})


@app.route('/game/')
def game():
    # style_sheet = send_from_directory('templates', 'style.css')
    # game = send_from_directory('templates', 'script.js')
    # print(game)

    # return render_template('game.html', game=game, style_sheet=style_sheet)
    return render_template('game.html')


if __name__ == '__main__':
    app.run()
