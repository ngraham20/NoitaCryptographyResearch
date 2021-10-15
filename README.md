# README

## Example Output
```rust

─────────────────────────────────────────
 Noita Eyes Cipher : Version 0.1.0
 by Nathaniel Graham (@ngraham20, @Raevenant)
─────────────────────────────────────────
                 ██████        
             ████      ████    
           ██      ██      ██  
         ██      ██████      ██
           ██      ██      ██  
             ████      ████    
                 ██████        
─────────────────────────────────────────
─────────────────────────────────────────
              east1: Lines
─────────────────────────────────────────
201013223304041130232114313033004024000
032041220001422242122220110003201341113
310221044000200104040144142033022034241
231313130031132120142231331441341441211
014003212114130041110100241241004031001
040331432341122101010040120412442442402
13331220330103113111211210322314
1310424224130304110203123204313
─────────────────────────────────────────
         east1: Trigrams (plain)
─────────────────────────────────────────
200 231 010 143 222 023 300 104 044 221 132 240 231 222 112 024 311 013 030 303 002 104 023 144 001 310 
312 130 223 311 041 034 000 130 201 230 101 024 041 240 142 134 143 132 034 143 023 142 034 144 242 111 
010 044 003 133 214 232 113 144 131 220 041 101 110 010 100 040 241 021 244 211 004 244 034 241 004 201 
131 133 310 242 204 223 304 311 030 031 134 111 110 022 113 212 103 023 224 133 143 
─────────────────────────────────────────
        east1: Trigrams (base10)
─────────────────────────────────────────
50 66 5 48 62 13 75 29 24 61 42 70 66 62 32 14 81 8 15 78 2 29 13 49 1 80 
82 40 63 81 21 19 0 40 51 65 26 14 21 70 47 44 48 42 19 48 13 47 19 49 72 31 
5 24 3 43 59 67 33 49 41 60 21 26 30 5 25 20 71 11 74 56 4 74 19 71 4 51 
41 43 80 72 54 63 79 81 15 16 44 31 30 12 33 57 28 13 64 43 48 
─────────────────────────────────────────
         east1: Trigrams (ascii)
─────────────────────────────────────────
Rb%P^-k=8]Jfb^@.q(/n"=-Q!p
rH_q53 HSa:.5fOLPJ3P-O3Qh?
%8#K[cAQI\5:>%94g+jX$j3g$S
IKphV_oq/0L?>,AY<-`KP
─────────────────────────────────────────
         east1: Trigrams (runic)
─────────────────────────────────────────
ᛒᛢᚥᛐᛞᚭ᛫ᚽᚸᛝᛊᛦᛢᛞᛀᚮᛱᚨᚯᛮᚢᚽᚭᛑᚡᛰ
ᛲᛈᛟᛱᚵᚳᚠᛈᛓᛡᚺᚮᚵᛦᛏᛌᛐᛊᚳᛐᚭᛏᚳᛑᛨᚿ
ᚥᚸᚣᛋᛛᛣᛁᛑᛉᛜᚵᚺᚾᚥᚹᚴᛧᚫᛪᛘᚤᛪᚳᛧᚤᛓ
ᛉᛋᛰᛨᛖᛟᛯᛱᚯᚰᛌᚿᚾᚬᛁᛙᚼᚭᛠᛋᛐ
─────────────────────────────────────────
       east1: Trigrams (alchemic)
─────────────────────────────────────────
🜲🝂🜅🜰🜾🜍🝋🜝🜘🜽🜪🝆🝂🜾🜠🜎🝑🜈🜏🝎🜂🜝🜍🜱🜁🝐
🝒🜨🜿🝑🜕🜓🜀🜨🜳🝁🜚🜎🜕🝆🜯🜬🜰🜪🜓🜰🜍🜯🜓🜱🝈🜟
🜅🜘🜃🜫🜻🝃🜡🜱🜩🜼🜕🜚🜞🜅🜙🜔🝇🜋🝊🜸🜄🝊🜓🝇🜄🜳
🜩🜫🝐🝈🜶🜿🝏🝑🜏🜐🜬🜟🜞🜌🜡🜹🜜🜍🝀🜫🜰
```
## What We Know
There are 9 messages hidden in the two direct parallel worlds *East 1* and *West 1*. For every message location in the East Parallel, there is an equivalent message in the same location in the West Parallel, save for East 5, noted below.
- East 1
- West 1
- East 2
- West 2
- East 3
- West 3
- East 4
- West 4
- East 5

Notice that there is no West 5. The reason for this is unknown, however it is speculated that the location where West 5 would be may be either a trigger location or an event location that triggers when the puzzle is solved.

## What has been tried
This is an incomplete list. If a theory has been tested, please add it here. See [Contributing](./CONTRIBUTING.md) for information on how to update this.

Going forward, we will assume that these ciphers have been tried on the **trigrams**, since we're assuming that the trigrams are the correct way to begin reading the messages.

### Simple Substitution Cipher
Frequency analysis has been tried on the trigrams, with no workable results, indicating that simple substition isn't the answer.

### Wheel Ciphers
This is being currently explored. The idea here is that you can have one or more wheels inside each other. There are many ways that wheels can interact.
   1. If they're the same size
        a. Simple substitution cipher (debunked most likely, see above)
        b. Incrementing cipher, where the outer ring has the 83 glyphs, and the inner ring has the plaintext with gaps. Each time you encode a plaintext character to a glyph, rotate the ring one position.
