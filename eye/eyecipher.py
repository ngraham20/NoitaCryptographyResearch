from wheelcipher import WheelCipher
from wheel import Wheel

class EyeCipher(WheelCipher):
    """
    EyeCipher is a custom cipher, which may or may not closely resemble another type of cipher.
    It's purpose is to allow for customization of encoding and decoding without confusing
    a user by just modifying a more well-known cipher
    """
    def __init__(self, wheels, cipheroptions=None):
        self.cipheroptions = cipheroptions or {}
        super().__init__(wheels)

    def encode(self, plaintext, history=None):
        """
        For each letter in plaintext
            for each wheel
                propegate the letter through to the last wheel
                if the resulting letter is the same as the last, find the next

                another option here is instead of having two alphabets on the wheel,
                just have a full ascii conversion including uppercase, lowercase,
                and symbols and rotate only when a double occurs, OR rotate each time,
                but then rotate again if a double occurs
        """
        ciphertext = ""
        for letter in plaintext:
            index = self.wheels[0].data.find(letter)

            # if no results, add the letter
            if index == -1:
                ciphertext += letter
            
            # # if the new glyph is the previous glyph
            else:
                if len(ciphertext) > 0 and self.wheels[1].data[index] == ciphertext[-1]:
                    if self.cipheroptions.get("FindAltOnDouble", False):
                        # find the next one and check if _it's_ -1
                        subindex = self.wheels[0].data.find(letter, index+1)
                        if subindex != -1:
                            index = subindex

                    elif self.cipheroptions.get("RotateOnDouble", False):
                        # find the next one
                        EyeCipher.advance_wheels(self.wheels[1::], Wheel.Direction.CLOCKWISE) 
                
                ciphertext += self.wheels[1].data[index]

                if self.cipheroptions.get("RotateOnTranslate"):
                    EyeCipher.advance_wheels(self.wheels[1::], Wheel.Direction.CLOCKWISE)
            EyeCipher.record(history, ciphertext)
        return ciphertext

    # def _translate(self, rotate_on_translate, rotate_on_double):

        


    def decode(self, ciphertext, history=None):
        """
        For each glyph in ciphertext
            for each wheel
                propegate the glyph backwards through to the first wheel
        """
        plaintext = ""
        flipwheels = self.wheels[::-1]
        for glyph in ciphertext:
            index = flipwheels[0].data.find(glyph)
            if index == -1:
                plaintext += glyph
            else:
                plaintext += flipwheels[1].data[index]
                self.advance_wheels(flipwheels[1::], Wheel.Direction.ANTICLOCKWISE)
            AlbertiCipher.record(history, plaintext)
        return plaintext