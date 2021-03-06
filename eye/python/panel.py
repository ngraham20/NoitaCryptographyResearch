
class Panel:
    """
    panel is a UI element between '─' bars

    ──────────────────
    step: 15
    bcda
    fghe
    ijkl
    ──────────────────

    or

    ─────────────────────────────────────────                                                                                                                                  
            east1: Trigrams (runic)                                                                                                                                           
    ─────────────────────────────────────────                                                                                                                                  
    ᛒ ᛢ ᚥ ᛐ ᛞ ᚭ ᛫ ᚽ ᚸ ᛝ ᛊ ᛦ ᛢ ᛞ ᛀ ᚮ ᛱ ᚨ ᚯ ᛮ ᚢ ᚽ ᚭ ᛑ ᚡ ᛰ                                                                                                                        
    ᛲ ᛈ ᛟ ᛱ ᚵ ᚳ ᚠ ᛈ ᛓ ᛡ ᚺ ᚮ ᚵ ᛦ ᛏ ᛌ ᛐ ᛊ ᚳ ᛐ ᚭ ᛏ ᚳ ᛑ ᛨ ᚿ                                                                                                                        
    ᚥ ᚸ ᚣ ᛋ ᛛ ᛣ ᛁ ᛑ ᛉ ᛜ ᚵ ᚺ ᚾ ᚥ ᚹ ᚴ ᛧ ᚫ ᛪ ᛘ ᚤ ᛪ ᚳ ᛧ ᚤ ᛓ                                                                                                                        
    ᛉ ᛋ ᛰ ᛨ ᛖ ᛟ ᛯ ᛱ ᚯ ᚰ ᛌ ᚿ ᚾ ᚬ ᛁ ᛙ ᚼ ᚭ ᛠ ᛋ ᛐ
    ─────────────────────────────────────────

    modules should have a to_panel(self) method, and the UI should simply print panels.
    """

    def __init__(self, data, size, inset=1, title=None, subtitle=None):
        self.data = data
        self.size = size
        self.inset = inset
        self.title = title
        self.subtitle = subtitle
        self.datalines = self._generate_lines()
    
    def __str__(self):
        return '\n'.join(self.datalines)

    def append_data(self, data):
        self.data += data
        return self

    def as_lines(self):
        return self._generate_lines()

    def with_groupsize(self, size):
        if size > 0:
        # we do this backwards, because the size of the array grows past what it's told to count
            for i in range(len(self.data)-size, size-1, size*-1):
                self.data.insert(i, '')
            self.datalines = self._generate_lines()
        return self

    def _generate_lines(self):
        lines = []
        lines.append(self._bar())
        if self.title:
            lines.append(self._centerstring(self.title))
        if self.subtitle:
            lines.append(' '*self.inset + self.subtitle)
        lines.append(self._bar())
        for line in self.data:
            lines.append(' '*self.inset + str(line))
        return lines

    def _bar(self):
        return '─'*self.size
    
    def _centerstring(self, data):
        """
        Center a string within a line of a specified size
        """
        return ' '*((self.size-len(data))//2) + data