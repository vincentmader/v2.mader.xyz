import matplotlib.pyplot as plt


def main():
    fig, ax = plt.gcf(), plt.gca()

    fig.patch.set_facecolor('#222222')
    ax.set_facecolor('#222222')

    ax.title.set_color('white')
    ax.xaxis.label.set_color('white')
    ax.yaxis.label.set_color('white')

    ax.spines['top'].set_color('white')
    ax.spines['bottom'].set_color('white')
    ax.spines['left'].set_color('white')
    ax.spines['right'].set_color('white')

    ax.tick_params(axis='x', colors='white')
    ax.tick_params(axis='y', colors='white')
    plt.xticks(color='white')
    plt.yticks(color='white')

    legend = plt.legend(frameon=True)
    for text in legend.get_texts():
        text.set_color('white')
    frame = legend.get_frame()
    frame.set_facecolor('#222222')
    frame.set_edgecolor('white')
