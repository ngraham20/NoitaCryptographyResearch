from enum import Enum
from cipher import Cipher

class WheelCipher(Cipher):
    def __init__(self, wheels, reflector=False):
        """
        Takes a list of wheels and converts a message through them one at a time.
        Ultimately, this can be configured to work similar to the enigma machine.
        That said, the sockets part won't be implemented at this time, just the wheels.
        """
        self.wheels = wheels
        self.reflector = reflector
    
    @staticmethod
    def advance_wheel(wheels, direction):
        """
        BUG: if you rotate once forward, then once back, the next wheel will rotate backwards due to the index being 0 again
        """
        if len(wheels) == 0:
            return

        wheels[0].rotate(direction)

        # if the fromwheel has gone all the way around, advance the next wheel by one position (recursively)
        # the towheel will become the advancing wheel during this recurse
        if wheels[0].radial_position == 0:
            WheelCipher.advance_wheel(wheels[1:], direction)

    def encode(self, plaintext):
        """
        [message] is a plaintext string
        - With each input,
            - send through every wheel, encoding as it goes
            - if reflector is True, reflect, and send it through again backwards
            - rotate the firstt wheel one place.
                - if a full rotation is completed, the next wheel is rotated one place
        """
        for c in plaintext:
            pass
    
    def decode(self, ciphertext):
        pass

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