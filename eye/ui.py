import constants
import json

class UI:
    size=41
    conf="conf.json"

    @staticmethod
    def program_header(conf=None, size=None):
        conf = conf or UI.conf
        size = size or UI.size

        print('─'*size)
        print(" " + constants.PROGRAM_NAME_FULL + " : Version " + constants.PROGRAM_VERSION)
        print(" by " + constants.AUTHOR_NAME +
        " (@" + (', @'.join(constants.AUTHOR_ALIASES))+")")
        print('─'*size)
        for segment in constants.EYE:
            print(' '*((size-len(segment))//2) + segment)
        print('─'*size)


    @staticmethod
    def header(header, size=None):
        size = size or UI.size
        headersize = len(header)
        hprespace = (size-headersize)//2
        print('─'*size)
        print((' '*hprespace) + header)
        print('─'*size)