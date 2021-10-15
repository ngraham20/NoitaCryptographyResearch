import constants
import argparse
import json
import sys

class UI:
    size=41
    conf="conf.json"

    def parse_args():
        parser = argparse.ArgumentParser(description="Run various cryptographic algorithms against the Eyes Messages from Noita.")
        parser.add_argument('-m',action='append',nargs=2,metavar=('message','style'))
        eyeargs = parser.parse_args()
        if eyeargs.m is None:
            parser.print_help()
            sys.exit(2)
        return eyeargs
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