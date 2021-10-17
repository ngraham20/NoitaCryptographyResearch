from cipher import Cipher
from panel import Panel
from wheel import Wheel

class WheelCipher(Cipher):
    def __init__(self, wheels):
        """
        Takes a list of wheels and converts a message through them one at a time.
        Ultimately, this can be configured to work similar to the enigma machine.
        That said, the sockets part won't be implemented at this time, just the wheels.
        """
        self.wheels = wheels

    def as_panel(self, size):
        return Panel(self.wheels, size, 1)
    
    @staticmethod
    def advance_wheels(wheels, direction):
        """
        BUG: if you rotate once forward, then once back, the next wheel will rotate backwards due to the index being 0 again
        """
        if len(wheels) == 0:
            return

        wheels[0].rotate(direction)

        # if the fromwheel has gone all the way around, advance the next wheel by one position (recursively)
        # the towheel will become the advancing wheel during this recurse
        if wheels[0].radial_position == 0:
            WheelCipher.advance_wheels(wheels[1:], direction)