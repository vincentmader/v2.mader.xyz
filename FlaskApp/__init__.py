from flask import Flask, render_template


app = Flask(__name__)


@app.route('/')
def chart():
    return render_template('chart.html')


if __name__ == '__main__':
    app.run()
