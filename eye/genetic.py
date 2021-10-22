from wheel import Wheel
from subcipher import SubCipher
from analysis import FreqAnalysis
import random
import json

class Gene:
    def __init__(self, data):
        self.data = data
        self.fitness = None

    def better_than(self, gene):
        if self.fitness <= gene.fitness:
            return True
        else:
            return False

    @staticmethod
    def get_children(parents):
        children = []
        p1 = list(parents[0].data)
        p2 = list(parents[1].data)

        # new generation
        c1 = [None] * len(p1)
        c2 = [None] * len(p1)

        points = sorted(random.sample(range(len(p1)), 2))
        a , b = points[0], points[1]
        for i in range(a,b):
            c1[i] = p1[i]
            c2[i] = p2[i]

        indeces = list(range(len(p1)))
        cslots = indeces[b:] + indeces[:a]
        i1 = 0
        i2 = 0
        j1 = 0
        j2 = 0
        while i1 < len(cslots) or i2 < len(cslots):
            if p2[j1] not in c1:
                c1[cslots[i1]] = p2[j1]
                i1+=1
            if p1[j2] not in c2:
                c2[cslots[i2]] = p1[j2]
                i2+=1
            j1+=1
            j2+=1

        td = {}
        td2 = {}
        for i in range(len(c1)):
            if c1[i] in td:
                td[i] += 1
            else:
                td[i] = 1
            if c2[i] in td2:
                td2[i] += 1
            else:
                td2[i] = 1
        print(c1)
        print(max([x for _, (_, x) in enumerate(td.items())]))
        print(c2)
        print(max([x for _, (_, x) in enumerate(td2.items())]))
        # TODO: fix this to create whole generation from two parents
        

            

class Genetic:
    def __init__(self, cipher, analysis):
        self.cipher = cipher
        self.analysis = analysis

    def generate_population(self, size):
        population = []
        for i in range(size):
            # copy a wheel and shuffle its data
            data = self.cipher.wheels[-1].data
            cipherwheel = Gene(''.join(random.sample(data, len(data))))
            population.append(cipherwheel)
        return population

    def test_population_fitness(self, population):

        for gene in population:
            # decode with wheel

            self.cipher.wheels[1] = Wheel(gene.data)
            message = self.cipher.decode("ᛖᛟᛟᚨᚰᛖᚨᚭᛞᛆᛒᛟᛄᛄᛒᚸᛞᛚᛒᚨᛆᛟᚭᚨᚨᚸᛙᛚᛒᚨᛖᛟᛟᚨᚰᛒᛞᚸᚪᚰᛒᚪᛖᛚᛄᚸᛙᛖᛙᚸᛟᛒᛞᚨ")
            (l, c, f) = FreqAnalysis.get_stats(message)

            freqdata = [{
                "letters": l,
                "count": c,
                "frequency": f,
                "color": "blue",
                "title": "name"
            }]
            self.analysis.data = freqdata
            gene.fitness = self.analysis.analyze()

    def produce_next_generation(self, population):
        # create a whole new population based off this one, of the same size
        # pick in groups of two. Odd one out survives too
        # parents make two children, of opposite crossovers

        # TODO pick two parents only, produce whole population from them
        popsize = len(population)
        sections = [population[x:x+2] for x in range(0, popsize, 2)]
        nextpopulation = []
        for parents in sections:
            if len(parents) < 2:
                nextpopulation.append(parents[0])
            else:
                Gene.get_children(parents)

    
def load_config(filename):
    f = open(filename)
    eyes=json.load(f)
    f.close()
    return eyes

wheels = [
    Wheel("oyqpucbdsgmevgfbjauwtdpalbtmxkizezlsiffzxrrvakhwnoqvhaxnicnyobyqjmpewsdhctdrcgkejul"),
    Wheel("ᛒᛕᚿᛡᛳᛉᚻᛄᛞᛆᚶᚸᛦᚠᚬᚱᛎᛖᚧᚪᚨᚷᛥᛮᛟᛣᛈᛏᛑᛐᚭᚫᛤᛩᛓᛗᛴᛇᛰᚢᛋᛙᛛᛠᛯᚵᚰᚺᛚᚼᚴᛁᛵᚣᚤᛪᛂᛜᚦᛢᛔᚮᛀᛷᚡᛸᛱᚹᚩᛝᛅᛶᚽᛍᚾᛨᛲᚯᛃᚥᛘᛊᚳ")
]

eyes = load_config('data/genetic-d.json')
analysisoptions = eyes.get("analysisoptions")

fa = FreqAnalysis([], analysisoptions, 1)            

cipher = SubCipher(wheels)

g = Genetic(cipher, fa)
pop = g.generate_population(5)
g.test_population_fitness(pop)
g.produce_next_generation(pop)