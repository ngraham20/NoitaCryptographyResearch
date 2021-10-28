from enum import Enum

class Wheel:
    """
    The idea here is that you can have one or more wheels inside each other.
    There are many ways that wheels can interact.
    1. If they're the same size
        a. Simple substitution cipher (debunked most likely)
        b. Incrementing cipher, where the outer ring has the 83 glyphs, and the
            inner ring has the plaintext with gaps. Each time you encode a
            plaintext character to a glyph, rotate the ring one position.
    
    Ultimately, a wheel is just a wrapper for a list, where "rotating" the list
    pops an element from one side and pushes it to the other. The logic for having
    two or more wheels of different sizes may or may not be part of this class.
    Not sure yet.
    """

    Direction = Enum('Direction', 'CLOCKWISE ANTICLOCKWISE')

    def __init__(self, data):
        self.data = data
        self.datatype = type(data)
        self.radial_position = 0
    
    def __str__(self):
        return self.data
        

    def rotate(self, direction, distance=1):
        for i in range(distance):
            self._rotate(direction)

    def _rotate(self, direction):
        """
        The radial position describes the rotation of the wheel.
        For instance, "ghijklabcdef" would have a radial position of 6, assuming an alphabetical wheel.
        """
        if self.datatype is list:
            if direction == Wheel.Direction.CLOCKWISE:
                glyph = self.data.pop(-1)
                self.data.insert(0, glyph)
            elif direction == Wheel.Direction.ANTICLOCKWISE:
                glyph = self.data.pop(0)
                self.data.append(glyph)
        elif self.datatype is str:
            if direction == Wheel.Direction.CLOCKWISE:
                self.data = self.data[-1]+self.data[:-1]
            elif direction == Wheel.Direction.ANTICLOCKWISE:
                self.data = self.data[1:]+self.data[0]
        self.radial_position = (self.radial_position + 1) % len(self.data)