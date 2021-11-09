from datetime import datetime as dt
import os

import PyPDF2
# from tika import parser  # pip install tika
import pdfreader
from pdfreader import PDFDocument, SimplePDFViewer

import FlaskApp.config as config
from FlaskApp.config import PATH_TO_RAW_DATA as P2RD


path_to_sparkasse_docs = os.path.join(
    config.PATH_TO_RAW_DATA, 'finances/banking', 'ING-DIBA'
)


# 600.95
def remove_leading_char(char, string):
    if string.startswith(char):
        string = string[1:]
        string = remove_leading_char(char, string)
        return string
    else:
        return string


def plot_networth(account_balances):
    for bank in account_balances.keys():
        dates = list(sorted(account_balances[bank].keys()))
        balances = [account_balances[bank][d] for d in dates]
        plt.bar(dates, balances)
    plt.savefig('net_worth.pdf')


def load_Sparkasse_Gitokonto(content):
    content = ''.join(content).split('Kontostand')[1:]
    content = ''.join(content).split('\n')

    for line in content:
        try:
            date = dt.strptime(line.split(' ')[0], '%d.%m.%Y')
            dt.strptime(line.split(' ')[1], '%d.%m.%Y')
            break
        except Exception:
            continue

    for line in content:
        if not line.endswith('+'):
            continue
        line = remove_leading_char('-', line)
        foo = line[:-1].split(',')
        balance = float(foo[0]) + float(foo[1]) / 100.

    return date, balance


def load_ING_DiBa_Extra_Konto(content):
    for line in content:

        try:
            date = dt.strptime(line, '%d.%m.%Y\n')
        except Exception:
            continue

    skipped_alter_saldo = False
    for line in content:
        if line.endswith(' Euro\n'):
            if not skipped_alter_saldo:
                skipped_alter_saldo = True
                continue
            else:
                foo = line.split(' ')[0]
                balance = float(foo.split(',')[0])
                balance += float(foo.split(',')[1]) / 100
                break

    return date, balance


def get_balance_details(bank, doc_name, content):
    # load txt files into list of event objects/dictionaries
    # if doc_name.startswith('Direkt_Depot_8013719253'):
    #     transactions += load_ING_DiBa_Direkt_Depot(content)
    if doc_name.startswith('Extra_Konto_5533147316'):
        date, balance = load_ING_DiBa_Extra_Konto(content)
    # elif doc_name.startswith('Girokonto_'):
    #     date, balance = load_ING_DiBa_Extra_Konto(content)
        # date, balance = load_ING_DiBa_Girokonto(content)
    elif doc_name.startswith('Konto_1010628787'):
        date, balance = load_Sparkasse_Gitokonto(content)
    else:
        # print('Error: no loading method for this bank account')
        # print(bank)
        # print(doc_name)
        # input('Press Enter to skip...')
        return None, None

    return date, balance


def main():

    account_balances = {}

    banks = ['Ing-DIBA', 'Sparkasse']
    for bank in banks:
        account_balances[bank] = {}

        path_to_docs = os.path.join(
            P2RD, 'finances', 'banking', bank, 'Auszuege'
        )

        docs = os.listdir(path_to_docs)
        for doc_name in sorted(docs):
            path_to_doc = os.path.join(path_to_docs, doc_name)

            # convert PDFs to txt with bash command pdftotext
            if doc_name.lower().endswith('.pdf'):
                os.system(f'pdftotext {path_to_doc}')
            # open txt file
            elif doc_name.lower().endswith('.txt'):
                with open(path_to_doc) as fp:
                    content = fp.readlines()

                date, balance = get_balance_details(
                    bank, doc_name, content
                )
                if balance:
                    account_balances[bank][date] = balance

    plot_networth(account_balances)
