from datetime import datetime as dt

from bokeh.plotting import figure
from bokeh.models import ColumnDataSource
from bokeh.embed import components

from maderxyz.chronos import load_raw


ALIASES = {
    'Selina Patent': [
        'Selina Patent',
        'Selina <33'
    ], 'Camillo Mizaikoff': [
        'Camillo Mizaikoff'
    ]
}


def setup_source(query, contact_name):

    whatsapp = load_raw.whatsapp.chat_history()
    facebook = load_raw.facebook.chat_history()
    # skype = load_raw.skype.chat_history()
    sms = load_raw.sms.chat_history()
    all_msgs = whatsapp + facebook + sms  # + skype,

    counts = {}

    for msg in all_msgs:
        month = dt.strptime(
            dt.fromtimestamp(
                msg['timestamp']
            ).strftime('%Y-%m-01'), '%Y-%m-01'
        )
        if dt.fromtimestamp(msg['timestamp']) < dt(2010, 1, 1):
            continue
        if msg['chat_name'] not in ALIASES[contact_name]:
            continue
        if query.lower() in msg['message_content'].lower() or query == '':
            try:
                counts[month] += 1
            except KeyError:
                counts[month] = 1

    return ColumnDataSource(dict(
        month=[i for i in sorted(counts.keys())],
        count=[counts[i] for i in sorted(counts.keys())]
    ))


def main(query):
    fig = figure(
        x_axis_type='datetime',
    )

    source = setup_source(query, 'Camillo Mizaikoff')

    fig.scatter(
        x='month', y='count', source=source
    )
    fig.line(
        x='month', y='count', source=source
    )

    div, script = components(fig)
    return div, script
