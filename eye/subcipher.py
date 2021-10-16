from wheelcipher import WheelCipher

class SubCipher(WheelCipher):
    def encode(self, plaintext):
        """
        For each letter in plaintext
            find the letter on the first wheel
            append equivilant letter on second wheel to output
        """
        print("sub cipher placeholder")
    
    def decode(self, ciphertext):
        """
        For each glyph in ciphertext
            find the glyph on the second wheel
            append equivilant letter on first wheel to output
        """
        print("sub cipher placeholder")