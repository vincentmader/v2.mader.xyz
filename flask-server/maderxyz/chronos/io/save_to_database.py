from maderxyz import config
from maderxyz.config import MDB


def main(document, path=['uncategorized']):

    # get target collection
    collection = MDB
    for i in path:
        collection = collection[i]

    # prevent duplicate entries
    for i in ['date', 'title']:
        try:
            foo = {i: document[i]}
            if collection.find_one(foo) is not None:
                collection.delete_one(foo)
        except KeyError:  # KeyError: 'dates'
            pass  # TODO: rethink prevention of duplicates

    # inser document into collection
    collection.insert_one(document)
