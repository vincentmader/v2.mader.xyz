from datetime import datetime as dt

import numpy as np


GRADE_HISTORY = {
    'JSG': {
        'WS04/05': [
        ],
        'SS05': [
        ],
        'WS05/06': [
        ],
        'SS06': [
            # {'name': 'D', 'grade': 1, 'ignore': False},
            # {'name': 'M', 'grade': 1, 'ignore': False},
        ],
        'WS06/07': [
            {'name': 'Re', 'grade': 1.75, 'ignore': False},
            {'name': 'D', 'grade': 1.5, 'ignore': False},
            {'name': 'M', 'grade': 1.75, 'ignore': False},
            {'name': 'E', 'grade': 2.25, 'ignore': False},
            {'name': 'MNK', 'grade': 2, 'ignore': False},
            {'name': 'S', 'grade': 2.25, 'ignore': False},
        ],
        'SS07': [
            {'name': 'Re', 'grade': 2, 'ignore': False},
            {'name': 'D', 'grade': 1, 'ignore': False},
            {'name': 'M', 'grade': 1, 'ignore': False},
            {'name': 'E', 'grade': 2, 'ignore': False},
            {'name': 'MNK', 'grade': 2, 'ignore': False},
            {'name': 'S', 'grade': 2, 'ignore': False},
        ],
        'WS07/08': [
            {'name': 'Re', 'grade': 1.75, 'ignore': False},
            {'name': 'D', 'grade': 1.5, 'ignore': False},
            {'name': 'M', 'grade': 1.75, 'ignore': False},
            {'name': 'E', 'grade': 2, 'ignore': False},
            {'name': 'MNK', 'grade': 2.25, 'ignore': False},
            {'name': 'S', 'grade': 2.5, 'ignore': False},
        ],
        'SS08': [
            {'name': 'Re', 'grade': 2, 'ignore': False},
            {'name': 'D', 'grade': 1, 'ignore': False},
            {'name': 'M', 'grade': 2, 'ignore': False},
            {'name': 'E', 'grade': 2, 'ignore': False},
            {'name': 'MNK', 'grade': 2, 'ignore': False},
            {'name': 'S', 'grade': 2, 'ignore': False},
        ],
    }, 'HSSG UuMS': {
        'WS08/09': [
            {'name': 'Re', 'grade': 2.5, 'ignore': False},
            {'name': 'D', 'grade': 2.5, 'ignore': False},
            {'name': 'Gg', 'grade': 3, 'ignore': False},
            {'name': 'E', 'grade': 2.25, 'ignore': False},
            {'name': 'M', 'grade': 2, 'ignore': False},
            {'name': 'Bio', 'grade': 1.75, 'ignore': False},
            {'name': 'S', 'grade': 2.25, 'ignore': False},
            {'name': 'Mu', 'grade': 3, 'ignore': False},
            {'name': 'BK', 'grade': 2, 'ignore': False},
            {'name': 'NP', 'grade': 1.75, 'ignore': False},
        ],
        'SS09': [
            {'name': 'Re', 'grade': 3, 'ignore': False},
            {'name': 'D', 'grade': 2, 'ignore': False},
            {'name': 'Gg', 'grade': 3, 'ignore': False},
            {'name': 'E', 'grade': 2, 'ignore': False},
            {'name': 'M', 'grade': 2, 'ignore': False},
            {'name': 'Bio', 'grade': 2, 'ignore': False},
            {'name': 'S', 'grade': 3, 'ignore': False},
            {'name': 'Mu', 'grade': 3, 'ignore': False},
            {'name': 'BK', 'grade': 2, 'ignore': False},
            {'name': 'NP', 'grade': 2, 'ignore': False},
        ],
        'WS09/10': [
            {'name': 'Re', 'grade': 3.25, 'ignore': False},
            {'name': 'D', 'grade': 2.25, 'ignore': False},
            {'name': 'Gg', 'grade': 2.5, 'ignore': False},
            {'name': 'G', 'grade': 1.5, 'ignore': False},
            {'name': 'E', 'grade': 2.5, 'ignore': False},
            {'name': 'F', 'grade': 2.75, 'ignore': False},
            {'name': 'M', 'grade': 2, 'ignore': False},
            {'name': 'Bio', 'grade': 3.25, 'ignore': False},
            {'name': 'S', 'grade': 3.25, 'ignore': False},
            {'name': 'Mu', 'grade': 3, 'ignore': False},
            {'name': 'BK', 'grade': 2, 'ignore': False},
        ],
        'SS10': [
            {'name': 'Re', 'grade': 3, 'ignore': False},
            {'name': 'D', 'grade': 3, 'ignore': False},
            {'name': 'Gg', 'grade': 3, 'ignore': False},
            {'name': 'G', 'grade': 2, 'ignore': False},
            {'name': 'E', 'grade': 2, 'ignore': False},
            {'name': 'F', 'grade': 3, 'ignore': False},
            {'name': 'M', 'grade': 3, 'ignore': False},
            {'name': 'Bio', 'grade': 3, 'ignore': False},
            {'name': 'S', 'grade': 3, 'ignore': False},
            {'name': 'Mu', 'grade': 3, 'ignore': False},
            {'name': 'BK', 'grade': 2, 'ignore': False},
        ],
        'WS10/11': [
            {'name': 'Re', 'grade': 4, 'ignore': False},
            {'name': 'D', 'grade': 3, 'ignore': False},
            {'name': 'Gg', 'grade': 2.75, 'ignore': False},
            {'name': 'G', 'grade': 2.5, 'ignore': False},
            {'name': 'E', 'grade': 2.75, 'ignore': False},
            {'name': 'F', 'grade': 3.5, 'ignore': False},
            {'name': 'M', 'grade': 2.75, 'ignore': False},
            {'name': 'P', 'grade': 2.75, 'ignore': False},
            {'name': 'Bio', 'grade': 3.25, 'ignore': False},
            {'name': 'S', 'grade': 3.5, 'ignore': False},
            {'name': 'Mu', 'grade': 5, 'ignore': False},
            {'name': 'BK', 'grade': 2.5, 'ignore': False},
        ],
        'SS11': [
            {'name': 'Re', 'grade': 3, 'ignore': False},
            {'name': 'D', 'grade': 3, 'ignore': False},
            {'name': 'Gg', 'grade': 3, 'ignore': False},
            {'name': 'G', 'grade': 3, 'ignore': False},
            {'name': 'Gk', 'grade': 3, 'ignore': False},
            {'name': 'E', 'grade': 3, 'ignore': False},
            {'name': 'F', 'grade': 3, 'ignore': False},
            {'name': 'L', 'grade': 3, 'ignore': False},
            {'name': 'M', 'grade': 4, 'ignore': False},
            {'name': 'P', 'grade': 3, 'ignore': False},
            {'name': 'Bio', 'grade': 3, 'ignore': False},
            {'name': 'S', 'grade': 4, 'ignore': False},
            {'name': 'Mu', 'grade': 4, 'ignore': False},
            {'name': 'BK', 'grade': 4, 'ignore': False},
        ],
        'WS11/12': [
            {'name': 'Re', 'grade': 2, 'ignore': False},
            {'name': 'D', 'grade': 3, 'ignore': False},
            {'name': 'Gg', 'grade': 4, 'ignore': False},
            {'name': 'G', 'grade': 3.75, 'ignore': False},
            {'name': 'Gk', 'grade': 3.5, 'ignore': False},
            {'name': 'E', 'grade': 2.75, 'ignore': False},
            {'name': 'F', 'grade': 3, 'ignore': False},
            {'name': 'L', 'grade': 2.5, 'ignore': False},
            {'name': 'M', 'grade': 4, 'ignore': False},
            {'name': 'P', 'grade': 3, 'ignore': False},
            {'name': 'C', 'grade': 2.5, 'ignore': False},
            {'name': 'S', 'grade': 2.5, 'ignore': False},
            {'name': 'BK', 'grade': 5, 'ignore': False},
        ],
        'SS12': [
            {'name': 'Re', 'grade': 4, 'ignore': False},
            {'name': 'D', 'grade': 3, 'ignore': False},
            {'name': 'Gg', 'grade': 4, 'ignore': False},
            {'name': 'G', 'grade': 3, 'ignore': False},
            {'name': 'Gk', 'grade': 3, 'ignore': False},
            {'name': 'E', 'grade': 3, 'ignore': False},
            {'name': 'F', 'grade': 3, 'ignore': False},
            {'name': 'L', 'grade': 3, 'ignore': False},
            {'name': 'M', 'grade': 4, 'ignore': False},
            {'name': 'P', 'grade': 3, 'ignore': False},
            {'name': 'C', 'grade': 3, 'ignore': False},
            {'name': 'S', 'grade': 3, 'ignore': False},
            {'name': 'Mu', 'grade': 4, 'ignore': False},
        ],
        'WS12/13': [
            {'name': 'Re', 'grade': 4.0, 'ignore': False},
            {'name': 'D', 'grade': 4.75, 'ignore': False},
            {'name': 'Gg', 'grade': 3.0, 'ignore': False},
            {'name': 'G', 'grade': 3.75, 'ignore': False},
            {'name': 'Gk', 'grade': 3.5, 'ignore': False},
            {'name': 'E', 'grade': 3.5, 'ignore': False},
            {'name': 'F', 'grade': 3.75, 'ignore': False},
            {'name': 'L', 'grade': 4.0, 'ignore': False},
            {'name': 'M', 'grade': 2.75, 'ignore': False},
            {'name': 'P', 'grade': 3.75, 'ignore': False},
            {'name': 'C', 'grade': 2.5, 'ignore': False},
            {'name': 'Bio', 'grade': 2.75, 'ignore': False},
            {'name': 'S', 'grade': 2.75, 'ignore': False},
            {'name': 'BK', 'grade': 2.75, 'ignore': False},
        ],
        'SS13': [
            {'name': 'Re', 'grade': 3.0, 'ignore': False},
            {'name': 'D', 'grade': 4.0, 'ignore': False},
            {'name': 'Gg', 'grade': 3.0, 'ignore': False},
            {'name': 'G', 'grade': 3.0, 'ignore': False},
            {'name': 'Gk', 'grade': 4.0, 'ignore': False},
            {'name': 'E', 'grade': 3.0, 'ignore': False},
            {'name': 'F', 'grade': 3.0, 'ignore': False},
            {'name': 'L', 'grade': 4.0, 'ignore': False},
            {'name': 'M', 'grade': 3.0, 'ignore': False},
            {'name': 'P', 'grade': 4.0, 'ignore': False},
            {'name': 'C', 'grade': 3.0, 'ignore': False},
            {'name': 'Bio', 'grade': 2.0, 'ignore': False},
            {'name': 'S', 'grade': 3.0, 'ignore': False},
            {'name': 'BK', 'grade': 3.0, 'ignore': False},
        ],
        'WS13/14': [
            {'name': 'Re', 'grade': 1.75, 'ignore': False},
            {'name': 'D', 'grade': 3.0, 'ignore': False},
            {'name': 'G', 'grade': 3.0, 'ignore': False},
            {'name': 'Gk', 'grade': 2.25, 'ignore': False},
            {'name': 'E', 'grade': 2.25, 'ignore': False},
            {'name': 'F', 'grade': 3.25, 'ignore': False},
            {'name': 'L', 'grade': 2.0, 'ignore': False},
            {'name': 'M', 'grade': 3.5, 'ignore': False},
            {'name': 'P', 'grade': 2.0, 'ignore': False},
            {'name': 'C', 'grade': 3.0, 'ignore': False},
            {'name': 'Bio', 'grade': 2.25, 'ignore': False},
            {'name': 'S', 'grade': 2.25, 'ignore': False},
            {'name': 'Mu', 'grade': 2.0, 'ignore': False},
        ],
        'SS14': [
            {'name': 'Re', 'grade': 3.0, 'ignore': False},
            {'name': 'D', 'grade': 3.0, 'ignore': False},
            {'name': 'Gg', 'grade': 3.0, 'ignore': False},
            {'name': 'G', 'grade': 3.0, 'ignore': False},
            {'name': 'Gk', 'grade': 3.0, 'ignore': False},
            {'name': 'E', 'grade': 2.0, 'ignore': False},
            {'name': 'F', 'grade': 3.0, 'ignore': False},
            {'name': 'L', 'grade': 1.0, 'ignore': False},
            {'name': 'M', 'grade': 2.0, 'ignore': False},
            {'name': 'P', 'grade': 2.0, 'ignore': False},
            {'name': 'C', 'grade': 4.0, 'ignore': False},
            {'name': 'Bio', 'grade': 2.0, 'ignore': False},
            {'name': 'S', 'grade': 2.0, 'ignore': False},
            {'name': 'Mu', 'grade': 2.0, 'ignore': False},
            {'name': 'BK', 'grade': 2.0, 'ignore': False},
        ],
    }, 'HSSG KS': {
        'WS14/15': [
            {'name': 'D', 'grade': 7, 'ignore': False},
            {'name': 'E', 'grade': 12, 'ignore': False},
            {'name': 'BK', 'grade': 12, 'ignore': False},
            {'name': 'G', 'grade': 13, 'ignore': False},
            {'name': 'Gk', 'grade': 12, 'ignore': False},
            {'name': 'Rel', 'grade': 10, 'ignore': True},
            {'name': 'M', 'grade': 10, 'ignore': False},
            {'name': 'Ph', 'grade': 11, 'ignore': False},
            {'name': 'Bio', 'grade': 10, 'ignore': False},
            {'name': 'S', 'grade': 9, 'ignore': True},
            {'name': 'VMa', 'grade': 9, 'ignore': True},

        ],
        'SS15': [
            {'name': 'D', 'grade': 8, 'ignore': False},
            {'name': 'E', 'grade': 13, 'ignore': False},
            {'name': 'BK', 'grade': 11, 'ignore': False},
            {'name': 'G', 'grade': 12, 'ignore': False},
            {'name': 'Gg', 'grade': 11, 'ignore': False},
            {'name': 'Rel', 'grade': 11, 'ignore': False},
            {'name': 'M', 'grade': 13, 'ignore': False},
            {'name': 'Ph', 'grade': 13, 'ignore': False},
            {'name': 'Bio', 'grade': 11, 'ignore': False},
            {'name': 'S', 'grade': 11, 'ignore': True},
            {'name': 'VMa', 'grade': 14, 'ignore': False},

        ],
        'WS15/16': [
            {'name': 'D', 'grade': 9, 'ignore': False},
            {'name': 'E', 'grade': 14, 'ignore': False},
            {'name': 'BK', 'grade': 11, 'ignore': False},
            {'name': 'G', 'grade': 12, 'ignore': False},
            {'name': 'Gg', 'grade': 11, 'ignore': False},
            {'name': 'Rel', 'grade': 11, 'ignore': True},
            {'name': 'M', 'grade': 14, 'ignore': False},
            {'name': 'Ph', 'grade': 14, 'ignore': False},
            {'name': 'Bio', 'grade': 13, 'ignore': False},
            {'name': 'Inf', 'grade': 14, 'ignore': False},
            {'name': 'S', 'grade': 13, 'ignore': False},
            {'name': 'VMa', 'grade': 12, 'ignore': False},

        ],
        'SS16': [
            {'name': 'D', 'grade': 11, 'ignore': False},
            {'name': 'E', 'grade': 14, 'ignore': False},
            {'name': 'BK', 'grade': 13, 'ignore': False},
            {'name': 'G', 'grade': 13, 'ignore': False},
            {'name': 'Gk', 'grade': 14, 'ignore': False},
            {'name': 'Rel', 'grade': 14, 'ignore': False},
            {'name': 'M', 'grade': 15, 'ignore': False},
            {'name': 'Ph', 'grade': 15, 'ignore': False},
            {'name': 'Bio', 'grade': 12, 'ignore': False},
            {'name': 'Inf', 'grade': 14, 'ignore': False},
            {'name': 'S', 'grade': 10, 'ignore': True},
            {'name': 'VMa', 'grade': 14, 'ignore': False},
            {'name': 'Abi D', 'grade': 11, 'ignore': False},
            {'name': 'Abi M', 'grade': 15, 'ignore': False},
            {'name': 'Abi E', 'grade': 13, 'ignore': False},
            {'name': 'Abi Ph', 'grade': 14, 'ignore': False},
            {'name': 'M.Abi G', 'grade': 12, 'ignore': False},
        ],
    }, 'UHD BSc.': {
        'WS16/17': [
            {'name': 'M. Vorkurs', 'cp': 3, 'grade': None, 'ignore': False},
            {'name': 'Basiskurs', 'cp': 4, 'grade': None, 'ignore': False},
            {'name': 'PEP1', 'cp': 7, 'grade': 3.7, 'ignore': False},
            {'name': 'PTP1', 'cp': 8, 'grade': 3.0, 'ignore': False},
            {'name': 'LA1', 'cp': 8, 'grade': 4.0, 'ignore': True},
        ], 'SS17': [
            {'name': 'PEP2', 'cp': 7, 'grade': 4.0, 'ignore': True},
            {'name': 'PTP2', 'cp': 8, 'grade': 3.7, 'ignore': False},
            {'name': 'HM2', 'cp': 8, 'grade': 2.3, 'ignore': False},
            {'name': 'PAP1', 'cp': 6, 'grade': 1.3, 'ignore': False},
        ], 'WS17/18': [
            {'name': 'PEP3', 'cp': 7, 'grade': 2.7, 'ignore': False},
            {'name': 'PTP3', 'cp': 8, 'grade': 2.3, 'ignore': False},
            {'name': 'HM3', 'cp': 8, 'grade': 1.3, 'ignore': False},
            {'name': 'PAP2', 'cp': 7, 'grade': 1.3, 'ignore': False},
            {'name': 'IPI', 'cp': 7, 'grade': 3.7, 'ignore': False},
        ], 'SS18': [
            {'name': 'PEP4', 'cp': 7, 'grade': 2.3, 'ignore': False},
            {'name': 'PTP4', 'cp': 8, 'grade': 3.0, 'ignore': False},
            {'name': 'MMP', 'cp': 8, 'grade': 3.3, 'ignore': False},
            {'name': 'Astro1', 'cp': 4, 'grade': 2.3, 'ignore': False},
        ], 'WS18/19': [
            {'name': 'PEP5', 'cp': 7, 'grade': 2.0, 'ignore': False},
            {'name': 'AC1', 'cp': 6, 'grade': 2.0, 'ignore': False},
            {'name': 'ITI', 'cp': 7, 'grade': 1.7, 'ignore': False},
            {'name': 'Astro2', 'cp': 4, 'grade': 1.3, 'ignore': False},
            {'name': 'Astro-Prakt.', 'cp': 2, 'grade': None, 'ignore': False},
        ], 'SS19': [
            {'name': 'Comp. Phys.', 'cp': 6, 'grade': None, 'ignore': False},
            {'name': 'PSem', 'cp': 3, 'grade': 2.3, 'ignore': False},
            {'name': 'PFP1', 'cp': 4, 'grade': 1.0, 'ignore': False},
            {'name': 'Python-Kurs', 'cp': 2, 'grade': None, 'ignore': False},
            {'name': 'Logik', 'cp': 8, 'grade': 3.0, 'ignore': False},
        ], 'WS19/20': [
            {'name': 'BA', 'cp': 8, 'grade': 1.0, 'ignore': False},
            {'name': 'BK', 'cp': 4, 'grade': 1.7, 'ignore': False},
            {'name': 'Proj.-Prakt.', 'cp': 4, 'grade': None, 'ignore': False},
            {'name': 'PFP2', 'cp': 7, 'grade': 1.0, 'ignore': False},
        ]
    }, 'UHD MSc': {
        'SS20': [
            {'name': 'Env. Phys.', 'cp': 8, 'grade': 1.7, 'ignore': False}
        ]
    }
}


def convert_from_KS(grade):
    return round((17 - grade) / 3, 1)


def calc_avg_grade(institution, semester, lectures):

    grades = [
        i['grade'] for i in lectures
        if i['grade'] and not i['ignore']
    ]
    if institution in ['HSSG KS']:
        grades = [convert_from_KS(i) for i in grades]

    # print(round(np.mean(grades), 1), '\t', semester)

    return np.mean(grades)


def get_semester_dt(semester):
    if semester.startswith('W'):
        year = 2001 + int(semester[-5:-3])
        return dt(year, 3, 15)
    elif semester.startswith('S'):
        year = 2000 + int(semester[-2:])
        return dt(year, 9, 15)


def main():

    semesters, avg_grades = [], []
    # foo = {}
    # timeseries = {'semester_dt': [], 'avg_grade': []}

    for institution in GRADE_HISTORY.keys():
        for semester in GRADE_HISTORY[institution].keys():
            lectures = GRADE_HISTORY[institution][semester]
            if not lectures:  # skip semesters where no grades where recorded
                continue
            # for lecture in lectures:
            #     name = lecture['name']
            #     grade = lecture['grade']
            #     if institution in ['HSS_KS']:
            #         grade = convert_from_KS(grade)
            #     ignore = lecture['ignore']
            #     if 'cp' in lecture.keys():
            #         cp = lecture['cp']

            avg_grade = calc_avg_grade(institution, semester, lectures)
            semester = get_semester_dt(semester)

            semesters.append(semester)
            avg_grades.append(avg_grade)

            # foo[semester] = avg_grade

    # return semesters, avg_grades
    return semesters, avg_grades
