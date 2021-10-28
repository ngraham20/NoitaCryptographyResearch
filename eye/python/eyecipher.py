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
        # EyeCipher.record(history, plaintext)

        for letter in plaintext:
            EyeCipher.record(history, "Plaintext: " + letter)
            EyeCipher.record(history, "Wheel: " + self.wheels[0].data)
            EyeCipher.record(history, "Wheel: " + self.wheels[1].data)
            index = self.wheels[0].data.find(letter)
            datapoints = [letter]

            # if no results, add the letter
            if index == -1:
                ciphertext += letter
                datapoints.append(letter)
            
            # # if the new glyph is the previous glyph  🠖
            else:
                if len(ciphertext) > 0 and self.wheels[1].data[index] == ciphertext[-1]:

                    datapoints.append(ciphertext[-1])

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
            datapoints.append(ciphertext[-1])
            EyeCipher.record(history, "Encode " + ' 🠖 '.join(datapoints))
            EyeCipher.record(history, "Result: " + ciphertext)
        return ciphertext

    def decode(self, plaintext, history=None):
        """
        For each glyph in ciphertext
            for each wheel
                propegate the glyph backwards through to the first wheel
        """
        ciphertext = ""
        # EyeCipher.record(history, plaintext)

        for letter in plaintext:
            EyeCipher.record(history, "Plaintext: " + letter)
            EyeCipher.record(history, "Wheel: " + self.wheels[0].data)
            EyeCipher.record(history, "Wheel: " + self.wheels[1].data)
            index = self.wheels[1].data.find(letter)
            datapoints = [letter]

            # if no results, add the letter
            if index == -1:
                ciphertext += letter
                datapoints.append(letter)
            
            # # if the new glyph is the previous glyph  🠖
            else:
                if len(ciphertext) > 0 and self.wheels[0].data[index] == ciphertext[-1]:

                    datapoints.append(ciphertext[-1])

                    if self.cipheroptions.get("FindAltOnDouble", False):
                        # find the next one and check if _it's_ -1
                        subindex = self.wheels[1].data.find(letter, index+1)
                        if subindex != -1:
                            index = subindex

                    elif self.cipheroptions.get("RotateOnDouble", False):
                        # find the next one
                        EyeCipher.advance_wheels(self.wheels[1::], Wheel.Direction.CLOCKWISE) 
                
                ciphertext += self.wheels[0].data[index]

                if self.cipheroptions.get("RotateOnTranslate"):
                    EyeCipher.advance_wheels(self.wheels[1::], Wheel.Direction.CLOCKWISE)
            datapoints.append(ciphertext[-1])
            EyeCipher.record(history, "Encode " + ' 🠖 '.join(datapoints))
            EyeCipher.record(history, "Result: " + ciphertext)
        return ciphertext