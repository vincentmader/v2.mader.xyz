from datetime import datetime as dt
import os
import xml.etree.cElementTree as ET

from FlaskApp.config import PATH_TO_RAW_DATA as PRD
from FlaskApp import chronos
from FlaskApp.chronos.io import save_to_database


PATH_TO_SMS_EXPORTS = os.path.join(
    PRD, 'sms'
)
VALID_EXPORTS = [
    '03.10.12.xml', '04.11.12.xml', '09.09.12.xml', '24.7.2012.xml'
]


def main():

    # chat_history = []

    for export_file in os.listdir(PATH_TO_SMS_EXPORTS):
        if export_file not in VALID_EXPORTS:
            continue

        path_to_export_file = os.path.join(PATH_TO_SMS_EXPORTS, export_file)
        parser = ET.parse(path_to_export_file).getroot()

        for sms_tag in parser:
            # protocol = sms_tag.get('protocol')
            # address = sms_tag.get('address')
            date = sms_tag.get('date')
            timestamp = int(date) / 1000
            sms_type = int(sms_tag.get('type'))
            # subject = sms_tag.get('subject')
            body = sms_tag.get('body')
            # toa = sms_tag.get('toa')
            # sc_toa = sms_tag.get('sc_toa')
            # service_center = sms_tag.get('service_center')
            # read = sms_tag.get('read')
            # status = sms_tag.get('status')
            # locked = sms_tag.get('locked')
            # date_sent = sms_tag.get('date_sent')
            # readable_date = sms_tag.get('readable_date')
            contact_name = sms_tag.get('contact_name')

            sender = contact_name if sms_type == 1 else 'Vincent Mader'
            receiver = 'Vincent Mader' if sender == contact_name else contact_name
            # ^ TODO: change to address instead of name?

            document = {
                'message_type': 'sms',
                'chat_name': contact_name,
                'timestamp': timestamp,
                'sender': sender,
                'receiver': receiver,
                'message_content': body,
            }
            path = ['raw data', 'sms']
            save_to_database(document, path=path)

            # print(timestamp, sender, '->', receiver)
            # print(body)

    # return chat_history
