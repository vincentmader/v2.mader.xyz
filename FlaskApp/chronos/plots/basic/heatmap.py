from datetime import datetime as dt
from datetime import timedelta as td
import random
import os

# import bokeh
# from bokeh.embed import components
# # from bokeh.io import curdoc
# from bokeh.layouts import column
# from bokeh.models import ColumnDataSource, DatetimeTickFormatter, RangeTool, Square
# from bokeh.plotting import figure
# # from bokeh.themes import built_in_themes
# import numpy as np


# def main(data, itvl):

#     if itvl == 'weekly':
#         NR_OF_ROWS = 7
#     if itvl == 'monthly':
#         NR_OF_ROWS = 5
#     if itvl == 'yearly':
#         NR_OF_ROWS = 4
#     NR_OF_COLS = len(data) // NR_OF_ROWS + 1

#     # curdoc().theme = 'dark_minimal'

#     p = figure(
#         plot_width=600, plot_height=200,
#         title="squarezz",
#         x_range=(dt.now() - td(days=220), dt.now()),
#         y_range=(0, NR_OF_ROWS + 1),
#         x_axis_type="datetime", y_axis_type=None,
#         x_axis_location="above",
#         tooltips=[
#             ('date', '@date'), ('value', '@value'),
#             ('content', '@content')
#         ], toolbar_location=None,
#         match_aspect=True,
#         # tools='zoom_in,zoom_out,pan,box_zoom,reset,hover,save",
#         background_fill_color='#333333'
#     )
#     # p.axis.visible = False
#     p.grid.grid_line_color = None
#     # p.axis.axis_line_color = None
#     # p.axis.major_tick_line_color = None
#     # p.axis.major_label_text_font_size = "7px"
#     # p.axis.major_label_standoff = 0
#     # p.xaxis.major_label_orientation = np.pi/3
#     p.xaxis.formatter = DatetimeTickFormatter()

#     x, y, fill_alphas, date, values, content = [], [], [], [], [], []
#     max_value = max([i['value'] for i in data])
#     for entry in data:

#         c = entry['log_content']
#         content.append(c)

#         value = entry['value']
#         values.append(value)

#         fill_alpha = value / max_value
#         fill_alphas.append(fill_alpha)

#         timestamp = entry['timestamp']
#         d = dt.fromtimestamp(timestamp)
#         date.append(d.strftime('%Y-%m-%d'))

#         if itvl == 'weekly':
#             x.append(dt.strptime(
#                 (d - td(days=d.weekday())).strftime('%Y-%m-%d'), '%Y-%m-%d')
#             )
#         y.append(NR_OF_ROWS - d.weekday())

#     source = ColumnDataSource(dict(
#         x=x, y=y, fill_alpha=fill_alphas, date=date, value=values, content=content
#     ))

#     p.add_glyph(source, Square(
#         x='x', y='y', size=13, fill_color='#00FF00', fill_alpha='fill_alpha',
#     ))

#     select = figure(
#         title="",
#         plot_height=30, plot_width=600, y_range=p.y_range,
#         x_axis_location=None,
#         x_axis_type="datetime", y_axis_type=None,
#         tools="", toolbar_location=None,
#         background_fill_color='#333333'
#     )

#     range_tool = RangeTool(x_range=p.x_range)
#     range_tool.overlay.fill_color = "green"
#     range_tool.overlay.fill_alpha = 0.2

#     select.line(x=[dt(2016, 1, 1), dt.now()], y=[0, 0], color=None)
#     select.ygrid.grid_line_color = None
#     select.add_tools(range_tool)
#     select.toolbar.active_multi = range_tool

#     script, div = components(column(p, select))
#     return script, div

def main(data):
    pass
