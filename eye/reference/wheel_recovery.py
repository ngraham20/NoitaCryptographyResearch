# Toboter from Noita Discord

from collections import namedtuple
from itertools import permutations

DisplayMode_WheelSection = 0
DisplayMode_EyeDecode = 1

DISPLAY_MODE = DisplayMode_WheelSection




eyeMessages = ((50, 66, 5, 48, 62, 13, 75, 29, 24, 61, 42, 70, 66, 62, 32, 14, 81, 8,  15, 78, 2,  29, 13, 49, 1,  80, 82, 40, 63, 81, 21, 19, 0, 40, 51, 65, 26, 14, 21, 70, 47, 44, 48, 42, 19, 48, 13, 47, 19, 49, 72, 31, 5, 24, 3, 43, 59, 67, 33, 49, 41, 60, 21, 26, 30, 5, 25, 20, 71, 11, 74, 56, 4, 74, 19, 71, 4, 51, 41, 43, 80, 72, 54, 63, 79, 81, 15, 16, 44, 31, 30, 12, 33, 57, 28, 13, 64, 43, 48),
            (80, 66, 5, 48, 62, 13, 75, 29, 24, 61, 42, 70, 66, 62, 32, 14, 81, 8,  15, 78, 2,  29, 13, 49, 1,  29, 11, 30, 52, 81, 21, 19, 0, 25, 26, 54, 20, 14, 21, 70, 47, 44, 48, 42, 19, 48, 13, 47, 19, 49, 44, 26, 59, 77, 64, 43, 79, 28, 72, 64, 1, 30, 73, 23, 67, 6, 33, 25, 64, 81, 68, 46, 17, 36, 13, 17, 21, 68, 13, 9, 46, 67, 57, 34, 62, 82, 15, 10, 73, 62, 2, 11, 65, 72, 37, 44, 10, 43, 68, 62, 9, 34, 18),
            (36, 66, 5, 48, 62, 13, 75, 29, 24, 61, 42, 70, 66, 62, 32, 14, 81, 8,  15, 78, 2,  29, 13, 49, 1,  69, 76, 52, 9,  48, 66, 80, 22, 64, 57, 40, 49, 78, 3, 16, 56, 19, 47, 40, 80, 6, 13, 64, 29, 49, 64, 63, 6, 49, 31, 13, 16, 10, 45, 24, 26, 77, 10, 60, 81, 61, 34, 54, 70, 21, 15, 4, 66, 77, 42, 37, 30, 22, 0, 11, 41, 72, 57, 20, 23, 57, 65, 41, 23, 18, 72, 42, 5, 3, 26, 78, 8, 5, 54, 45, 77, 25, 64, 61, 16, 44, 54, 51, 20, 63, 25, 11, 26, 45, 53, 60, 38, 34),
            (76, 66, 5, 49, 75, 54, 69, 46, 32, 1,  42, 60, 26, 48, 50, 80, 32, 24, 55, 61, 47, 12, 21, 12, 49, 54, 34, 25, 36, 15, 56, 55, 20, 9, 8, 62, 13, 82, 9, 44, 29, 60, 53, 82, 42, 80, 5, 43, 71, 3, 80, 77, 47, 78, 34, 25, 62, 18, 10, 49, 62, 64, 52, 81, 11, 66, 62, 13, 47, 17, 52, 70, 26, 23, 32, 31, 64, 23, 35, 32, 50, 6, 1, 25, 8, 37, 47, 43, 26, 76, 65, 68, 80, 17, 7, 45, 63, 14, 53, 63, 60, 16),
            (63, 66, 5, 49, 75, 54, 2,  60, 29, 40, 78, 47, 60, 75, 67, 71, 60, 2,  65, 7,  47, 14, 45, 74, 59, 41, 80, 13, 60, 13, 81, 22, 35, 50, 40, 39, 2, 59, 48, 31, 76, 2, 80, 75, 1, 56, 67, 11, 21, 8, 40, 65, 45, 75, 55, 39, 60, 42, 13, 3, 22, 57, 2, 6, 58, 9, 70, 1, 58, 56, 63, 68, 25, 79, 7, 20, 19, 64, 2, 66, 73, 30, 71, 16, 12, 30, 65, 37, 20, 13, 22, 63, 18, 46, 64, 59, 41, 81, 82, 22, 78, 36, 47, 17, 4, 6, 17, 5, 36, 79, 63, 1, 64, 69, 15, 43, 4, 58, 56, 31, 14, 64, 58, 18, 44, 78, 69, 1, 0, 46, 20, 71, 73, 25, 35, 8, 24),
            (34, 66, 5, 49, 75, 54, 23, 74, 11, 13, 28, 26, 19, 48, 67, 57, 37, 60, 34, 28, 74, 10, 17, 32, 11, 18, 19, 43, 19, 81, 42, 4, 62, 9, 46, 49, 32, 51, 76, 58, 4, 43, 47, 17, 67, 79, 21, 32, 44, 16, 30, 37, 26, 28, 41, 68, 57, 34, 51, 10, 69, 70, 8, 6, 46, 43, 18, 39, 47, 43, 15, 13, 33, 30, 35, 62, 37, 0, 37, 5, 38, 55, 37, 13, 40, 25, 9, 21, 11, 64, 5, 79, 42, 68, 11, 71, 11, 48, 3, 67, 61, 40, 22, 14, 35, 50, 61, 39, 11, 2, 66, 49, 51, 53, 17, 73, 36, 75, 74, 54, 24, 30, 54, 70),
            (27, 66, 5, 49, 75, 54, 2,  60, 29, 40, 2,  55, 9,  15, 59, 18, 68, 3,  36, 5,  47, 77, 44, 38, 1,  18, 28, 76, 4,  34, 60, 63, 58, 80, 17, 54, 79, 75, 48, 54, 55, 19, 62, 64, 14, 47, 51, 70, 75, 5, 11, 47, 45, 58, 68, 69, 79, 25, 38, 45, 73, 47, 68, 50, 34, 45, 78, 26, 79, 57, 4, 56, 22, 60, 18, 75, 43, 60, 59, 67, 63, 42, 49, 33, 40, 65, 79, 77, 7, 3, 26, 62, 31, 78, 26, 57, 69, 40, 4, 23, 26, 13, 67, 42, 38, 72, 11, 39, 65, 60, 25, 6, 80, 66, 68, 77, 59, 78, 19),
            (77, 66, 5, 49, 75, 54, 2,  60, 29, 40, 2,  55, 9,  15, 59, 18, 68, 3,  36, 5,  47, 60, 21, 80, 1,  72, 55, 16, 82, 35, 57, 19, 1, 66, 18, 27, 39, 17, 74, 81, 39, 14, 78, 0, 25, 65, 43, 66, 64, 38, 81, 23, 24, 50, 57, 30, 71, 75, 26, 68, 54, 57, 56, 50, 71, 73, 14, 21, 8, 32, 26, 63, 5, 37, 19, 43, 66, 47, 53, 34, 66, 23, 73, 31, 54, 38, 77, 67, 11, 63, 79, 6, 22, 21, 51, 69, 74, 21, 5, 17, 67, 37, 29, 21, 60, 14, 82, 44, 30, 4, 20, 42, 35, 1, 31, 54, 46, 20, 40, 30),
            (33, 66, 5, 49, 75, 54, 2,  60, 29, 40, 2,  55, 9,  15, 59, 18, 68, 3,  36, 5,  47, 33, 21, 59, 44, 18, 28, 76, 59, 34, 60, 63, 79, 27, 12, 54, 5, 49, 48, 54, 55, 52, 62, 72, 69, 10, 57, 22, 58, 48, 67, 53, 7, 34, 32, 30, 31, 19, 26, 8, 34, 46, 7, 30, 71, 55, 34, 75, 54, 9, 6, 60, 5, 23, 25, 45, 42, 80, 25, 12, 22, 76, 20, 51, 62, 21, 40, 9, 41, 10, 44, 73, 8, 33, 70, 73, 6, 31, 21, 72, 5, 40, 61, 51, 42, 66, 64, 74, 61, 25, 63, 42, 24, 41))


Message = namedtuple("Message",["plaintext","trigrams","offset"])

#assumed shared message sections
messages = (
    Message("&ABCDEFGHI","fOLPJ3P-O3",39), #m1
    Message("JKLMNOPQR","53sHSa:.5",30),
    Message("&ABCDEFGHI","4g+jX$j3g$",67),
    
    Message("&ABCDEFGHI","qdN1D-15d-",69), #m2
    Message("JKLMNOPQR","53s9:V4.5",30),
    Message("STUVWX","`Ko<h`",54),
    Message("STUVWX","^r/*i^",84),
    
    Message("&ABCDEFGHI","p&-`=Q`_&Q",44), #m3
    Message("&ABCDEFGHI","+IhY47YaI7",79),
    Message("JKLMNOPQR","HQn#0X3OH",35),
    Message("STUVWX","*M8:m*",57),
    Message("STUVWX","%#:n(%",92),
    
    
    Message("abcdefghijkl","OMZdeo9FMiOd",51), #m7
    Message("mnopqrstuvwxyz0123456789","\\2kK\\[c_JQAHaom'#:^?n:YeH",83),
    Message("+%*/#!$.","VokPVW3^",35),
    
    Message("abcdefghijkl","RY>gk:dVYXRg",53), #m8
    Message("mnopqrstuvwxyz0123456789","bOUBb7i?VFmc+_o&65Sej5%1c",86),
    Message("+%*/#!$.","G1jqG.n ",36),
    
    Message("abcdefghijkl","'B@>?3:(BN'>",52), #m9
    Message("mnopqrstuvwxyz0123456789","9MJp9,6l4S^5H)I*Li(Afi&?5",84),
    Message("+%*/#!$.","V%QPVWT^",35),
    )


class WheelSection:
    def __init__(self,trigram,charOffsets=[],trigramOffsets=[]):
        self.trigram = trigram
        self.charOffsets = list(charOffsets)
        self.trigramOffsets = list(trigramOffsets)

    def addCharOffset(self,charOffset):
        global addEquivalence
        
        for o in self.charOffsets:
            if o.offset == charOffset.offset:
                if o.character != charOffset.character:
                    addEquivalence(o.character, charOffset.character)
                return
        else:
            self.charOffsets.append( charOffset)

    def addTrigramOffset(self,trigramOffset):
        
        if trigramOffset.offset == 0:
            if trigramOffset.trigram == self.trigram:
                return
            else:
                raise Exception(f"Trigram Collision between {self.trigram} and {trigramOffset.trigram}")
        
        for tri in self.trigramOffsets:
            if tri.offset == trigramOffset.offset:
                if tri.trigram == trigramOffset.trigram:
                    return
                else:
                    raise Exception(f"Trigram Collision between {tri.trigram} and {trigramOffset.trigram}")

        self.trigramOffsets.append(trigramOffset)

    def addWheelSection(self, other):
        "Try to merge the offsets of a different wheel section. return true on success, otherwise false"
        #find a matching offset between characters, and match accordingly
        sharedCharacterOffsets = []
        for co in self.charOffsets:
            for co2 in other.charOffsets:
                if co.character == co2.character:
                    sharedCharacterOffsets.append((co,co2))

        if len(sharedCharacterOffsets) == 0:
            return False

        
        offset = None #try to find offset

        if len(sharedCharacterOffsets) >= 2:
            #try to find offset based on distance between shared characters
            proposedOffsets = list()
            for a,b in permutations(sharedCharacterOffsets, 2):
                diffSelf = a[0].offset-b[0].offset
                diffOther = a[1].offset-b[1].offset
                if diffSelf == diffOther:
                    proposedOffsets.append(a[0].offset - a[1].offset)

            proposedOffsetSet = tuple(set(proposedOffsets))

            if len(proposedOffsetSet) > 0:  
                
                counts = {}
                for o in proposedOffsetSet:
                    counts[o] = proposedOffsets.count(o)

                if tuple(counts.values()).count(max(counts.values())) == 1: #if two propsed offsets are equally likely, don't merge

                    allCounts = list(counts.values())
                    allCounts.sort(reverse=True)

                    offset = None
                    for o in counts.keys():
                        if counts[o] == max(counts.values()):
                            offset = o

        if offset == None:
            #if shared character matching wasn't successful, try shared character offset matching
            offsetScores = {}
            for a, b in sharedCharacterOffsets:
                offsetsSelf = set()
                
                for o in self.charOffsets:
                    if o!=a:
                        offsetsSelf.add(o.offset - a.offset)

                offsetsOther = set()
                for o in other.charOffsets:
                    if o!=b:
                        offsetsOther.add(o.offset - b.offset)

                score = offsetScores.setdefault(a.offset - b.offset,0)

                for o in offsetsSelf:
                    if o in offsetsOther:
                        score += 1

                offsetScores[a.offset - b.offset] = score

            offsSorted = list(offsetScores.values())
            offsSorted.sort(reverse = True)

            if offsSorted.count(offsSorted[0]) == 1 and offsSorted[0]>=4: #no two offsets equally likely and there is enough evidence
                for o in offsetScores.keys():
                    if offsetScores[o] == offsSorted[0]:
                        offset = o
                        break

        if offset == None: #if still no offset found, return
            return False
                       
        self.mergeWheelSection(other, offset)
        
        return True

    def mergeWheelSection(self, other, offset):
        self.addTrigramOffset(TrigramOffset(other.trigram, offset))

        for tri in other.trigramOffsets:
            new = TrigramOffset(tri.trigram[:], tri.offset + offset)
            self.addTrigramOffset(new)

        
        for char in other.charOffsets:
            new = CharOffset(char.character, char.offset + offset)
            self.addCharOffset(new)
        
        
            
        
class CharOffset:
    def __init__(self, character, offset):#offset = how far before that trigram
        global getEquivalent
        self.character = getEquivalent(character)
        self.offset = offset % 83

    def __str__(self):
        return f"chr {self.character} {self.offset}"

class TrigramOffset:
    def __init__(self, trigram, offset):#offset = how far before that trigram
        self.trigram = trigram
        self.offset = offset % 83

    def __str__(self):
        return f"tri {self.trigram} {self.offset}"



sections = []   #the found wheel sections

equivalences = []   #the found character equivalences




#a is the old character, b the new one
def addEquivalence(a,b):
    
    #check for exiting equivalence
    for e in equivalences:
        if a in e:
            e.append(b)
            a = e[0]
            break
    else:
        equivalences.append([a,b])

    #update all the character offsets inside sections
    for s in sections:
        for o in s.charOffsets:
            if o.character == b:
                o.character = a

def getEquivalent(a):
    for e in equivalences:
        if a in e:
            return e[0]
    return a
    
            

#step one: extract character to trigram offsets
for m in messages:
    for i in range(len(m.plaintext)):
        offset = (i + m.offset)%83
        trigram = m.trigrams[i]
        character = getEquivalent(m.plaintext[i])

        for e in equivalences:
            if character in e:
                character = e[0]
                break
        
        section = None  #grab the section of that trigram
        
        for s in sections:
            if s.trigram == trigram:
                section = s
                break
        else:
            section = WheelSection(trigram,[])
            sections.append(section)

        #detect repeats/equivalences
        section.addCharOffset(CharOffset(character,offset))



#try to merge as many sections as possible

notCurrent = sections[:]    #have never been current

rerty = False

current = notCurrent.pop()
currentChanged = False

while notCurrent:
    print(f"Remaining sections: {len(notCurrent)}, available sections {len(sections)}")
    
    for sect in sections[:]:
        if sect == current:
            continue
        print(sect.trigram,end="")
        if current.addWheelSection(sect):
            currentChanged = True
            retry = True
            sections.remove(sect)
            if sect in notCurrent:
                notCurrent.remove(sect)
    print()

    if not currentChanged:
        current = notCurrent.pop()
    currentChanged = False

##    if not notCurrent and retry:
##        retry = False
##        notCurrent = sections[:]
        



if DISPLAY_MODE == DisplayMode_WheelSection:
        
    print("Equivalent values:",equivalences)

    alphabet = set()

    for m in messages:
        for c in m.plaintext:
            alphabet.add(getEquivalent(c))

    print("Alphabet:",len(alphabet))

    sections.sort(key = lambda a: len(a.charOffsets)+len(a.trigramOffsets), reverse =True)

    print("Wheel Sections (character or trigram, value, offset):")
    print()
    for sect in sections:
        offs = sect.charOffsets+sect.trigramOffsets
        offs.sort(key=lambda a:a.offset,reverse = True)
        for off in offs:
            print(str(off))
        print("tri",sect.trigram,0)
        print()

elif DISPLAY_MODE == DisplayMode_EyeDecode:
    print()
    print()
    for m in eyeMessages:
        for i in range(len(m)):
            tri = chr(m[i]+32)
            char = None
            for sect in sections:
                off = None
                if sect.trigram == tri:
                    off = 0
                else:
                    for triOff in sect.trigramOffsets:
                        if triOff.trigram == tri:
                            off = triOff.offset

                if off != None:
                    off = (off+i)%83
                    for charOff in sect.charOffsets:
                        if charOff.offset == off:
                            char = charOff.character
                            break
                if char:
                    break
            if char:
                print(char,end="")
            else:
                print("-",end="")
        print()
                        
            

    


##print(equivalences)
##for s in sections:
##    print(s.trigram,":")
##    for co in s.charOffsets:
##        print(co.character, co.offset)
            

##alphabet = list(alphabet)
##alphabet.sort()
##
##for a,b in permutations(alphabet,2):
##    distancesD = set()
##    distances = []
##    for s in sections:
##        aOffsets = []
##        for co in s.charOffsets:
##            if co.character == a:
##                aOffsets.append(co.offset)
##        for co in s.charOffsets:
##            if co.character == b:
##                for i in aOffsets:
##                    distancesD.add(i-co.offset)
##                    distances.append(i-co.offset)
##    distancesD = list(distancesD)
##    distancesD.sort()
##    if distancesD:
##        print(len(distances),len(distancesD),distances,distancesD,a,b)




            
    

                    
        
