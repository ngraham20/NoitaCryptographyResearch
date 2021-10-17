from panel import Panel

class Message:
    def __init__(self, name=None, text=None, delimiter=None):
        self.name = name
        self.text = text
        self.delimiter = delimiter
    
    @staticmethod
    def from_eyes(eyes, name):
        return Message(name, eyes["messages"][name], eyes["delimiter"])
    
    @staticmethod
    def _dec_to_rune(tri):
        return chr(tri+5792)
    
    @staticmethod
    def _dec_to_ascii(tri):
        return chr(tri+32)
    
    @staticmethod
    def _dec_to_alchemic(tri):
        return chr(tri+128768)
    
    @staticmethod
    def _tri_to_dec(tri):
        return int(tri,5)

    def as_trigrams(self, style):
        rows = [x for x in self.text.split('5') if x]
        tris = []
        for i in range(0, len(rows), 2):
            woven_tris = Message._weave_lines_to_trigrams(rows[i], rows[i+1])
            for tri in woven_tris:
                tris.append(tri)

        if style != "plain":
            tris = map(Message._tri_to_dec, tris)
        if style == "ascii":
            tris = map(Message._dec_to_ascii, tris)
        elif style == "runic":
            tris = map(Message._dec_to_rune, tris)
        elif style == "alchemic":
            tris = map(Message._dec_to_alchemic, tris)

        return list(map(str, list(tris)))

    def as_panel(self, size, fmt, style=None):
        if fmt == "trigrams":
            return self._get_trigrams_panel(size, style)
        elif fmt == "lines":
            return self._get_lines_panel(size)
        # elif format == "pixels":
        #     self._display_pixels()

    # def _display_pixels(self):
    #     print(self.__name__ + " is not yet implemented.")
    
    def _get_lines_panel(self, size):
        datarows = [x for x in self.text.split('5') if x]
        title = self.name + ": Lines"
        return Panel(datarows, size, 1, title)

    def _get_trigrams_panel(self, size, style=None):
        style = style or "plain"
        tris = self.as_trigrams(style)
        datarows = []
        trilines = [tris[x:x+26] for x in range(0, len(tris), 26)]
        for line in trilines:
            datarows.append(' '.join(line))
        
        title = self.name + ": Trigrams (" + style + ")"
        return Panel(datarows, size, 1, title)
    
    @staticmethod
    def _weave_lines_to_trigrams(l1, l2):
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