import numpy as np
import math
import matplotlib.pyplot as plt

class Analysis:
    def __init__(self, charts, width):
        self.charts = charts
        self.figures = []
        self.width = width
        self.layout = self._calc_layout()
        # self.add_charts(charts)

    def _calc_layout(self, extrasize=0):
        width = 0
        for width in range(math.ceil(math.sqrt(len(self.charts))) + extrasize, 0, -1):
            if (len(self.charts) % width == 0):
                break
        height = len(self.charts) // width
        return width*10 + height

    def generate_figures(self):
        pass

    @staticmethod
    def analyze():
        # fnt = {"fontname":"Noto Mono"}
        # plt.xlabel("title", **fnt)
        plt.show()

class FreqAnalysis(Analysis):
    """
    [
        {
            "bars": [
                {
                "letters": ['E', 'T', 'A', 'O','I','N','S','R','H','D','L','U','C','M','F','Y','W','G','P','B','V','K','X','Q','J','Z'],
                "frequency": [12.02, 9.10, 8.12, 7.68, 7.31, 6.95, 6.28, 6.02, 5.92, 4.32, 3.98, 2.88, 2.71, 2.61, 2.30, 2.11, 2.09, 2.03, 1.82, 1.49, 1.11, 0.69, 0.17, 0.11, 0.10, 0.07],
                "color": "orange"
                }
            ],
            "title": "English"
        }
    ]    
    """
    ENGLISHDATA = {
            "letters": ['E', 'T', 'A', 'O','I','N','S','R','H','D','L','U','C','M','F','Y','W','G','P','B','V','K','X','Q','J','Z'],
            "frequency": [12.02, 9.10, 8.12, 7.68, 7.31, 6.95, 6.28, 6.02, 5.92, 4.32, 3.98, 2.88, 2.71, 2.61, 2.30, 2.11, 2.09, 2.03, 1.82, 1.49, 1.11, 0.69, 0.17, 0.11, 0.10, 0.07],
            "color": "blue",
            "title": "English"
            }
    FINNISHDATA = {
            "letters": ['A', 'I','T','N','E','S','L','O','Ä','K','U','M','H','V','R','J','P','Y','D','Ö','G','C','B','F','W','Z','X','Q','Å','Š','Ž'],
            "frequency": [11.45, 10.46, 9.68, 9.14, 8.42, 6.59, 5.87, 5.50, 5.21, 4.92, 4.67, 3.18, 2.49, 2.45, 2.15, 2.07, 1.77, 1.71, 0.91, 0.39, 0.30, 0.27, 0.12, 0.09, 0.06, 0.04, 0.03, 0.01, 0.01, 0.01, 0.01],
            "color": "blue",
            "title": "Finnish"
            }

    def generate_figures(self):
        for fig in self.figures:
            plt.figure().close(fig)
        fig = plt.figure()
        for i, chart in enumerate(self.charts):
            # fig = plt.figure()
            self.figures.append(fig)
            # print(self.layout)
            ax = fig.add_subplot(self.layout * 10 + i + 1)
            ax.set_ylabel("Frequency")
            ax.set_title("Frequency Analysis: " + chart["title"])
            ind = np.arange(len(chart["letters"]))
            ax.bar(ind, chart["frequency"], self.width, color=chart["color"])
            fig.tight_layout()



    @staticmethod
    def get_frequency(text):
        """
        text is a string
        """
        countdict = {}
        letters = []
        frequency = []
        for character in text:
            if character in countdict:
                countdict[character] += 1
            else:
                countdict[character] = 1
        
        for _, (k, v) in enumerate(countdict.items()):
            letters.append(k)
            frequency.append(v)

        szl = sorted(zip(frequency, letters))
        letters = [f for _, f in szl][::-1]
        frequency = [ 100*l/len(text) for l, _ in szl][::-1]
        return (letters, frequency)