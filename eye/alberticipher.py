from wheelcipher import WheelCipher
from wheel import Wheel

class AlbertiCipher(WheelCipher):
    def encode(self, plaintext, history=None):
        """
        For each letter in plaintext
            for each wheel
                propegate the letter through to the last wheel
        """
        ciphertext = ""
        for letter in plaintext:
            index = self.wheels[0].data.find(letter)
            if index == -1:
                ciphertext += letter
            else:
                ciphertext += self.wheels[1].data[index]
                self.advance_wheels(self.wheels[1::], Wheel.Direction.CLOCKWISE)
            AlbertiCipher.record(history, ciphertext)
        return ciphertext