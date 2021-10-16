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
        subparsers = parser.add_subparsers(required=True, dest='eyemodule')

        # ----- MESSAGE PARSING -----
        messageparser = subparsers.add_parser('message')
        messageparser.add_argument('-m',action='append',nargs=2,metavar=('message','style'))

        # ----- CIPHER PARSING -----
        cipherparser = subparsers.add_parser('cipher')
        subciphers = cipherparser.add_subparsers(required=True, dest='ciphermodule')
        cipherparser.add_argument('-e', '--encrypt', help='encrypt specified plaintext')
        cipherparser.add_argument('-d', '--decrypt', help='decrypt specified ciphertext')

        # ---------- WHEEL CIPHER -----
        wheelparser = subciphers.add_parser('wheel')
        wheelparser.add_argument('-c', '--cipher', required=True, choices=['alberti', 'substitution'])
        wheelparser.add_argument('-w', nargs='+', metavar='wheels', help='add wheels to the cipher')

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
        print('─'*size)


    @staticmethod
    def header(header, size=None):
        size = size or UI.size
        headersize = len(header)
        hprespace = (size-headersize)//2
        print('─'*size)
        print((' '*hprespace) + header)
        print('─'*size)