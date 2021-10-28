from wheelcipher import WheelCipher

class SubCipher(WheelCipher):
    def encode(self, plaintext, history=None):
        """
        For each letter in plaintext
            find the letter on the first wheel
            append equivilant letter on second wheel to output
        """
        ciphertext = ""
        for letter in plaintext:
            index = self.wheels[0].data.find(letter)
            if index == -1:
                ciphertext += letter
            else:
                ciphertext += self.wheels[1].data[index]
            SubCipher.record(history, ciphertext)
        return ciphertext

    def decode(self, ciphertext, history=None):
        """
        For each glyph in ciphertext
            find the glyph on the second wheel
            append equivilant letter on first wheel to output
        """
        plaintext = ""
        for glyph in ciphertext:
            index = self.wheels[1].data.find(glyph)
            if index == -1:
                plaintext += glyph
            else:
                plaintext += self.wheels[0].data[index]
            SubCipher.record(history, plaintext)
        return plaintext