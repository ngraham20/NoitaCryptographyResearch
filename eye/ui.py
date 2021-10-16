import constants
import argparse
import json
import sys
from message import Message

class UI:
    size=41
    conf="conf.json"

    @staticmethod
    def parse_args():
        parser = argparse.ArgumentParser(description="Run various cryptographic algorithms against the Eyes Messages from Noita.")
        subparsers = parser.add_subparsers(required=True)

        # ----- MESSAGE PARSING -----
        messageparser = subparsers.add_parser('message')
        messageparser.add_argument('-m',action='append',nargs=2,metavar=('message','style'))

        # ----- CIPHER PARSING -----
        cipherparser = subparsers.add_parser('cipher')
        subciphers = cipherparser.add_subparsers(required=True)

        # ---------- WHEEL CIPHER -----
        wheelparser = subciphers.add_parser('wheel')
        wheelparser.add_argument('-c', '--cipher', required=True, choices=['alberti'])
        wheelparser.add_argument('-w', action='append', nargs='+', metavar='wheels', help='add wheels to the cipher')

        eyeargs = vars(parser.parse_args())
        trueargs = {k: v for k, v in eyeargs.items() if v is not None}
        if len(trueargs) < 1:
            parser.print_help()
            sys.exit(2)
        return trueargs
    
    @staticmethod
    def handle_args(eyes, data):
        if 'm' in data:
            for message in data['m']:
                fmt = None
                style = None
                if message[1][0] == 't':
                    fmt = "trigrams"
                elif message[1][0] == 'l':
                    fmt = "lines"
                if len(message[1]) > 1:
                    if message[1][1] == 's':
                        style = "ascii"
                    elif message[1][1] == 'a':
                        style = "alchemic"
                    elif message[1][1] == 'r':
                        style = "runic"
                    elif message[1][1] == 'd':
                        style = "decimal"
                print(Message.from_eyes(eyes, message[0]).as_panel(41, fmt, style))

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