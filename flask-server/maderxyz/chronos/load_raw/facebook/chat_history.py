from datetime import datetime as dt
import json
import os

from tqdm import tqdm

from FlaskApp.config import PATH_TO_RAW_DATA as PRD
from FlaskApp.config import FACEBOOK_USER_NAME as ME
from FlaskApp.chronos.io import save_to_database


def main():

    # loop over files in inbox on disk
    path_to_inbox = os.path.join(PRD, 'facebook/json/messages/inbox')
    for dir_name in tqdm(sorted(os.listdir(path_to_inbox))):

        # get contents of chat file
        path_to_chat = os.path.join(path_to_inbox, dir_name, 'message.json')
        with open(path_to_chat) as fp:
            content = json.load(fp)

        # handle group chats (skip, for now)
        participants = content['participants']
        if len(participants) > 2:  # TODO: handle group chats
            continue

        # loop over messages
        for msg in content['messages']:
            timestamp = int(msg['timestamp_ms']) / 1e3
            date = dt.fromtimestamp(timestamp)

            # TODO: make dict to translate into real names
            chat_id = dir_name.split('_')[0]

            # get name (id?) of sender, infer receiver (TODO: groups)
            sender = msg['sender_name']
            receiver = ME if sender != ME else sender

            # TODO: ?
            try:
                message_content = msg['content']
            except KeyError:
                continue

            # create document and save to database
            document = {
                'message type': 'facebook',
                'chat name': chat_id,
                'timestamp': timestamp,
                'date': date,
                'sender': sender,
                'receiver': receiver,
                'message content': message_content,
                'message length': len(message_content),
            }
            path = ['raw data', 'facebook', 'chat_history']
            save_to_database(document, path=path)
