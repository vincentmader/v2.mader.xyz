from datetime import datetime as dt
import os

from FlaskApp.config import PATH_TO_RAW_DATA as PRD
from FlaskApp import chronos
from FlaskApp.chronos.io import save_to_database


PATH_TO_WHATSAPP_DATA = os.path.join(PRD, 'whatsapp')


def main():

    chat_history = []
    for chat_file in os.listdir(PATH_TO_WHATSAPP_DATA):
        if chat_file == '.DS_Store':
            continue
        chat_name = chat_file.split('.')[0]
        if chat_name[-1] in [str(i) for i in range(1, 10)]:
            chat_name = chat_name[:-2]

        path_to_chat_file = os.path.join(PATH_TO_WHATSAPP_DATA, chat_file)
        with open(path_to_chat_file) as fp:
            content = fp.readlines()

        # make sure one line corresponds to exactly one message
        tmp = []
        for line in content:
            if line.startswith('['):
                tmp.append(line)
            else:
                tmp[-1] += '\n' + line
        content = tmp

        # go through messages
        for line in content:
            tmp = line.split(']')  # using these tmp vars saves ~10% comp. time
            date_str = tmp[0][1:]
            timestamp = dt.strptime(date_str, '%d.%m.%y, %H:%M:%S').timestamp()
            tmp = tmp[1][1:].split(':')
            message_content = tmp[1][1:]

            sender = tmp[0] if tmp[0] != 'vinc' else 'Vincent Mader'
            receiver = chat_name if sender == 'Vincent Mader' else 'Vincent Mader'

            document = {
                'message_type': 'whatsapp',
                'chat_name': chat_name,
                'timestamp': timestamp,
                'sender': sender,
                'receiver': receiver,
                'message_content': message_content,
            }
            path = ['raw data', 'whatsapp']
            save_to_database(document, path=path)

    return chat_history
