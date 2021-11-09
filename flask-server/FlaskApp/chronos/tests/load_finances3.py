from datetime import datetime as dt
import os

import PyPDF2
# from tika import parser  # pip install tika
import pdfreader
from pdfreader import PDFDocument, SimplePDFViewer

import config


path_to_sparkasse_docs = os.path.join(
    config.PATH_TO_RAW_DATA, 'finances/banking', 'ING-DIBA'
)


def get_networth_timeseries(transactions):

    sorted_transactions = sorted(
        transactions, key=lambda k: k['timestamp'], reverse=False
    )

    # print(sorted_transactions)
    # input()

    current_networth, networths = 600.95, []
    for transaction in sorted_transactions:

        timestamp = transaction['timestamp']
        value = transaction['transaction_value']

        print(dt.fromtimestamp(timestamp))
        print(current_networth, value)
        input()

        networths.append(current_networth + value)
        current_networth = networths[-1]


def load_ING_DiBa_Direkt_Depot(content):

    transactions = []

    return transactions


def load_ING_DiBa_Extra_Konto(content):

    transactions = []

    return transactions


def load_ING_DiBa_Girokonto(content):

    transactions = []

    return transactions


def load_Sparkasse_Girokonto(doc_name, content):

    transactions = []

    # timestamps, transaction_values = [], []
    # transaction_types = []

    # content = ''.join(content).split('Kontostand')[1:]

    kontostand_title_skipped = False

    for line_idx, line in enumerate(content):

        # skip all lines not starting with two dates
        try:
            date = dt.strptime(line.split(' ')[0], '%d.%m.%Y')
            dt.strptime(line.split(' ')[1], '%d.%m.%Y')
        except Exception as err:
            continue

        #    #
        transaction_type = line.split(' ')[2]
        # transaction_desc = ''

        value_found, search_idx = False, 0
        while not value_found:

            value_line = content[line_idx + search_idx]
            if value_line.endswith('-\n'):
                transaction_sign = -1
            elif value_line.endswith('+\n'):
                transaction_sign = +1
            else:
                # transaction_desc += value_line
                search_idx += 1
                continue
            # print(value_line)
            # input()

            if not kontostand_title_skipped:
                kontostand_title_skipped = True
                search_idx += 1
                continue

            value_found = True
            foo = value_line[:-2].split(',')
            transaction_value = transaction_sign * (
                float(foo[0]) + float(foo[1]) / 100.
            )
            # print(transaction_value)
            # input()

        # if transactions:
        #     if transaction_value == transactions[-1]['transaction_value']:
        #         continue

        # timestamps.append(date.timestamp())
        # transaction_values.append(transaction_value)
        # transaction_types.append(transaction_type)
        # # transaction_descs.append(transaction_desc)

        transactions.append({
            'timestamp': date.timestamp(),
            'transaction_value': transaction_value,
            'transaction_type': transaction_type,
            # 'transaction_desc': transaction_desc,
        })
        # timestamps.append(date.timestamp())
        # transaction_values.append(transaction_value)

        # transactions.append({
        # 'timestamp': date.timestamp(),
        # 'transaction_value': transaction_value,
        # 'transaction_type': transaction_type,
        # # 'transaction_desc': transaction_desc,
    # })

    # transactions['timestamps'] = timestamps
    # transactions['transaction_values'] = transaction_values

    return transactions


def load_Sparkasse_Girokonto2(doc_name, content):

    total_transaction = 0
    for line in content:
        if line.endswith('+'):
            transaction_sign = 1
        elif line.endswith('-'):
            transaction_sign = -1
        else:
            continue

        foo = line[:-2].split(',')
        transaction_value = transaction_sign * (
            float(foo[0]) + float(foo[1]) / 100.
        )
        total_transaction += transaction_value

    return total_transaction


def get_date_from_filename(bank, filename):

    if bank == 'Sparkasse':
        date = dt.strptime(
            filename.split('-')[1],
            'Auszug_%Y_0%m.txt'
        )

    elif bank == 'Ing-DIBA':
        date = dt.strptime(
            filename.split('_')[-1],
            '%Y%m%d.txt'
        )

        return date


def get_pdf_content(path_to_pdf):

    with open(path_to_pdf[:-3] + 'txt') as fp:
        content = fp.readlines()

    return content


def main():

    transactions = []

    # banks = ['Sparkasse']
    banks = ['Ing-DIBA', 'Sparkasse']
    for bank in banks:

        path_to_docs = os.path.join(
            config.PATH_TO_RAW_DATA, 'finances',
            'banking', bank, 'Auszuege'
        )

        docs = os.listdir(path_to_docs)
        for doc_name in sorted(docs):
            path_to_doc = os.path.join(path_to_docs, doc_name)

            # convert PDFs to txt with bash command pdftotext
            if doc_name.lower().endswith('.pdf'):
                os.system(f'pdftotext {path_to_doc}')
                continue

            # open txt file
            with open(path_to_doc) as fp:
                content = fp.readlines()

            # load txt files into list of event objects/dictionaries
            if doc_name.startswith('Direkt_Depot_8013719253'):
                transactions += load_ING_DiBa_Direkt_Depot(content)
            elif doc_name.startswith('Extra_Konto_5533147316'):
                transactions += load_ING_DiBa_Extra_Konto(content)
            elif doc_name.startswith('Girokonto_'):
                transactions += load_ING_DiBa_Girokonto(content)
            elif doc_name.startswith('Konto_1010628787'):
                # print(doc_name)
                transactions += load_Sparkasse_Girokonto(doc_name, content)
                load_Sparkasse_Girokonto2(doc_name, content)
                # print(len(transactions))
            else:
                print('Error: no loading method for this bank account')
                print(bank)
                print(doc_name)
                input('Press Enter to skip...')

    networth_timeseries = get_networth_timeseries(transactions)
