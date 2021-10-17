class Cipher:

    def encode(self, plaintext, history=None):
        print("encode not yet implemented")

    def decode(self, ciphertext, history=None):
        print("Decode not yet implemented")
    
    @staticmethod
    def record(history, data):
        if history is not None:
            history.append(data)