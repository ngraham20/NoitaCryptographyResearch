from ui import UI

class Message:
    def __init__(self, name=None, text=None, delimiter=None):
        self.name = name
        self.text = text
        self.delimiter = delimiter
    
    @staticmethod
    def from_eyes(eyes, name):
        return Message(name, eyes["messages"][name], eyes["delimiter"])

    def display(self, format, style=None):
        if format == "trigrams":
            self._display_trigrams(style)
        elif format == "lines":
            self._display_lines()
        elif format == "pixels":
            self._display_pixels()

    def _display_pixels(self):
        print(self.__name__ + " is not yet implemented.")
    
    def _display_lines(self):
        rows = [x for x in self.text.split('5') if x]
        UI.header(self.name + ": Lines")
        for row in rows:
            print(row)

    def _display_trigrams(self, style):
        rows = [x for x in self.text.split('5') if x]
        tris = []
        UI.header(self.name + ": Trigrams")
        for i in range(0, len(rows), 2):
            woven_tris = Message._weave_lines_to_trigrams(rows[i], rows[i+1])
            for tri in woven_tris:
                tris.append(tri)

        for i, tri in enumerate(tris):
            btennum=int(tri,5)
            if style == "plain":
                print(tri+' ', end='')
            elif style == "base10":
                print(str(btennum)+' ', end='')
            elif style == "ascii":
                print(chr(btennum+32), end='')
            elif style == "runic":
                print(chr(btennum+5792), end='')
            if (i+1) % 26 == 0:
                print()
        print()
    
    @staticmethod
    def _weave_lines_to_trigrams(l1, l2):
        '''
        ### step 0 : lines
        0123456789...
        abcdefghijklmnopqrstuvwxyz
        zyxwvutsrqponmlkjihgfedcba

        ### step 1 : patterns
        0  1 2  3
        ab c de f 
        z yx w vu

        ### step 2 : grouping
        abz cyx dew fvu

        ### step 3 : reversing
        abz xyc dew uvf

        ### line 1 patterns
        i   |      L,R  |  sec  |        L      |      R
        ----+-----------+-------+---------------+-----------
        0   |   i+0,i+2 |   ab  | i+(i+1)//2    | i+(i+4)//2
        1   |   i+1,i+2 |   c   | i+(i+1)//2    | i+(i+4)//2
        2   |   i+1,i+3 |   de  | i+(i+1)//2    | i+(i+4)//2
        3   |   i+2,i+3 |   f   | i+(i+1)//2    | i+(i+4)//2
        4   |   i+2,i+4 |   gh  | i+(i+1)//2    | i+(i+4)//2
        5   |   i+3,i+4 |   i   | i+(i+1)//2    | i+(i+4)//2
        6   |   i+3,i+5 |   jk  | i+(i+1)//2    | i+(i+4)//2
        7   |   i+4,i+5 |   l   | i+(i+1)//2    | i+(i+4)//2


        ### line 2 patterns
        i   |      L,R  |  sec  |        L      |      R
        ----+-----------+-------+---------------+-----------
        0   |   i+0,i+1 |   z   | i+(i//2)      | i+(i+3)//2
        1   |   i+0,i+2 |   yx  | i+(i//2)      | i+(i+3)//2
        2   |   i+1,i+2 |   w   | i+(i//2)      | i+(i+3)//2
        3   |   i+1,i+3 |   vu  | i+(i//2)      | i+(i+3)//2
        4   |   i+2,i+3 |   t   | i+(i//2)      | i+(i+3)//2
        5   |   i+2,i+4 |   sr  | i+(i//2)      | i+(i+3)//2
        6   |   i+3,i+4 |   q   | i+(i//2)      | i+(i+3)//2
        7   |   i+3,i+5 |   po  | i+(i//2)      | i+(i+3)//2
        '''

        trigrams = []
        i = 0
        
        while True:
            tri = l1[(i+((i+1)//2)):i+((i+4)//2)] + l2[(i+(i//2)):(i+(i+3)//2)]
            if i % 2 == 0:
                trigrams.append(tri)
            else:
                trigrams.append(tri[::-1])
            i+=1
            if i >= ((len(l1)+len(l2))//3):
                break
        return trigrams