import re

import maderxyz.config
from maderxyz.chronos import load_raw


def main(word, chat_name='all', msg_type='all'):

    chat_history = load_raw.chat_history()

    x, y = [], []

    for msg_obj in chat_history:
        # filter for sent/received messages
        if msg_type == 'received' and msg_obj['sender'] == 'Vincent Mader':
            continue
        elif msg_type == 'sent' and msg_obj['sender'] != 'Vincent Mader':
            continue
        # filter for chat name
        if chat_name != 'all' and msg_obj['chat_name'] != chat_name:
            continue

        msg_timestamp = msg_obj['timestamp']
        msg_content = msg_obj['message_content']
        words = [
            w.lower()[:-1] for w in msg_content.split(' ')
        ]

        matches = re.findall(word, msg_content)
        if matches:
            x.append(msg_timestamp)
            y.append(len(matches))

    return x, y
