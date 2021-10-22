from wheel import Wheel
from subcipher import SubCipher
from alberticipher import AlbertiCipher
from analysis import FreqAnalysis
from eyecipher import EyeCipher
import random
import json

class Gene:
    def __init__(self, data):
        self.data = data
        self.fitness = None
        self.age = 0

    def __lt__(self, gene):
        return self.fitness < gene.fitness

    @staticmethod
    def get_children(p1, p2):
        # children = []
        p1 = p1.data
        p2 = p2.data

        # new generation
        # for _ in range(count):
        #     children.append([None] * len(p1))
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
        # print(c1)
        # print(max([x for _, (_, x) in enumerate(td.items())]))
        # print(c2)
        # print(max([x for _, (_, x) in enumerate(td2.items())]))

        return (Gene(''.join(c1)), Gene(''.join(c2)))
        

            

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

    def test_population_fitness(self, population, against):
        # print([x.data for x in population])

        for gene in population:
            # decode with wheel

            self.cipher.wheels[1] = Wheel(gene.data)
            message = self.cipher.decode(against)
            (l, c, f) = FreqAnalysis.get_stats(message)

            freqdata = [{
                "letters": l,
                "count": c,
                "frequency": f,
                "color": "blue",
                "title": "name"
            }]
            self.analysis.data = freqdata
            gene.fitness = self.analysis.analyze()[0]
        population.sort()

    def pick_parents(self, population, count):
        """
        numbers range from 0 to 2000
        say they're 300, 400, 500, 600, 900
        convert to ranges 0 to 100
        sum is 2700
        300/2700 = 0.11 110 slots
        400/2700 = 0.15 150 slot
        500/2700 = 0.19 190 slots
        600/2700 = 0.22 220 slots
        900/2700 = 0.33 330 slots

        """
        s = sum([x.fitness for x in population])
        slots = []
        selected = []
        
        for gene in population:
            slots.append(1000 * gene.fitness / s)
        slots.reverse()

        i = 0
        while i < count:
            picker = random.randint(0,1000)
            for j, gene in enumerate(population):
                if picker >= slots[j] and gene not in selected:
                    selected.append(gene)
                    i+=1
                    break
        return selected

    def produce_next_generation(self, population, parents, size, deathage):
        children = []
        # nextpopulation = []
        mcp = 10

        dcc = mcp

        # grab parents in groups of 2
        sections = [population[x:x+2] for x in range(0, len(parents), 2)]
        if len(sections[-1]) < 2:
            sections = sections[:-1]
        # print(sections)

        count = 0
        i = 0
        while len(children) < dcc:
            parents = sections[i]
            children += list(Gene.get_children(parents[0], parents[1]))
            count+=2
            i = (i + 1) % len(sections)
        
        for x in population:
            x.age += 1
            # if x.age > deathage:
            #     population.remove(x)

        # kc = 0
        # if len(population) > size - mcp:
        #     kc = len(population) - size - mcp
        
        # dcc = size - len(population)

        # if too many produced, ditch the last one
        # print(children)
        if len(children) > dcc:
            children = children[:-1*(count - dcc)]
        # print(children)
        nextpopulation = sorted(population, key=lambda x: x.age)[:-10] + children
        return nextpopulation

    
def load_config(filename):
    f = open(filename)
    eyes=json.load(f)
    f.close()
    return eyes

wheels = [
    Wheel("oyqpucbdsgmevgfbjauwtdpalbtmxkizezlsiffzxrrvakhwnoqvhaxnicnyobyqjmpewsdhctdrcgkejul"),
    Wheel("ᛒᛕᚿᛡᛳᛉᚻᛄᛞᛆᚶᚸᛦᚠᚬᚱᛎᛖᚧᚪᚨᚷᛥᛮᛟᛣᛈᛏᛑᛐᚭᚫᛤᛩᛓᛗᛴᛇᛰᚢᛋᛙᛛᛠᛯᚵᚰᚺᛚᚼᚴᛁᛵᚣᚤᛪᛂᛜᚦᛢᛔᚮᛀᛷᚡᛸᛱᚹᚩᛝᛅᛶᚽᛍᚾᛨᛲᚯᛃᚥᛘᛊᚳ")
]

# cipheroptions = {"RotateOnTranslate": true}


eyes = load_config('data/genetic-d.json')
analysisoptions = eyes.get("analysisoptions")
cipheroptions = eyes.get("cipheroptions")

fa = FreqAnalysis([], analysisoptions, 1)           

# cipher = SubCipher(wheels)
# cipher = AlbertiCipher(wheels)
cipher = EyeCipher(wheels, cipheroptions)

# print(cipher.decode("ᛖᛮᛥᛖᛛᛦᚬᛮᛒᛒᛍᚠᛃᚯᛝᚥᛨᚫᛸᛕᚽᛡᛞᛘᚥᛝᚱᚷᛪᚾᛅᛲᛨᛅᛦᛚᛪᛜᚡᛄᛠᚮᚦᛉᚰᚼᛃᚣᛲᚵᛜᛤᚢᚴ"))


g = Genetic(cipher, fa)

cm = "ᛖᛮᛥᛖᛛᛦᚬᛮᛒᛒᛍᚠᛃᚯᛝᚥᛨᚫᛸᛕᚽᛡᛞᛘᚥᛝᚱᚷᛪᚾᛅᛲᛨᛅᛦᛚᛪᛜᚡᛄᛠᚮᚦᛉᚰᚼᛃᚣᛲᚵᛜᛤᚢᚴᚼᚾᛰᛈᚼᚫᛠᚩᛰᛋᚼᛈᛣᛴᛂᚬᚮᚷᚧᚶᚨᛎᛪᚻᛣᛮᛩᛥᚺᚸᛋᛉᛞᛄᛊᛗᛲᛨᚸᛊᚻᛐᛊᛓᚩᛳᛟᛲᛊᚩᛣᚽᚹᛸᛢᚯᚪᛱᚩᚸᛵᛢᛅ"
ow = "ᛒᛕᚿᛡᛳᛉᚻᛄᛞᛆᚶᚸᛦᚠᚬᚱᛎᛖᚧᚪᚨᚷᛥᛮᛟᛣᛈᛏᛑᛐᚭᚫᛤᛩᛓᛗᛴᛇᛰᚢᛋᛙᛛᛠᛯᚵᚰᚺᛚᚼᚴᛁᛵᚣᚤᛪᛂᛜᚦᛢᛔᚮᛀᛷᚡᛸᛱᚹᚩᛝᛅᛶᚽᛍᚾᛨᛲᚯᛃᚥᛘᛊᚳ"

pop = g.generate_population(100)
while True:
    g.test_population_fitness(pop, cm)
    # print([x.fitness for x in pop])
    mf = min(pop, key= lambda x: x.fitness)
    # print(mf)
    parents = g.pick_parents(pop, 2)
    pop = g.produce_next_generation(pop, parents, 100, 30)
    if mf.fitness < 30:
        break


# g.test_population_fitness(pop)
# m = min(pop, key= lambda x: x.fitness)
cipher.wheels[1] = Wheel(mf.data)
message = cipher.decode(cm)
print(mf.data)
print(mf.fitness)
print(message)

cipher.wheels[1] = Wheel(ow)
m2 = cipher.decode(cm)
print(ow)
(l, c, f) = FreqAnalysis.get_stats(m2)

freqdata = [{
    "letters": l,
    "count": c,
    "frequency": f,
    "color": "blue",
    "title": "name"
}]
fa.data = freqdata
print(fa.analyze()[0])
print(m2)
