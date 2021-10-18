import constants
import argparse
import json
import sys

class UI:
    size=41
    conf="conf.json"

    @staticmethod
    def parse_args():
        parser = argparse.ArgumentParser(description="Run various cryptographic algorithms against the Eyes Messages from Noita.")
        subparsers = parser.add_subparsers(dest='eyemodule')
        parser.add_argument('-f', '--file', metavar='JSON', dest='confjson', help='use a config json file')
        eyeargs = vars(parser.parse_args())
        trueargs = {k: v for k, v in eyeargs.items() if v is not None}
        if len(trueargs) < 1:
            parser.print_help()
            sys.exit(2)
        return trueargs

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

    @staticmethod
    def header(header, size=None):
        size = size or UI.size
        headersize = len(header)
        hprespace = (size-headersize)//2
        print('─'*size)
        print((' '*hprespace) + header)
        print('─'*size)