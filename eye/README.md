


necpy 


https://en.wikipedia.org/wiki/Hermetic_Order_of_the_Golden_Dawn
https://en.wikipedia.org/wiki/Cipher_Manuscripts

minimum number of trigrams is 83.
Alchemy's highest purpose is to turn lead into gold
atomic number of lead is also 83
atomic number for gold is 79

https://docs.google.com/spreadsheets/d/195Rtc8kj4b74LtIyakqGP-iHhm36vyT5i8w7H5JjOV8/edit#gid=202652133

https://medium.com/analytics-vidhya/how-to-build-an-enigma-machine-virtualisation-in-python-b5476a1fd922

```
### step 0 : lines
0123456789...
abcdefghijklmnopqrstuvwxyz
zyxwvutsrqponmlkjihgfedcba

### step 1 : patterns
0  1 2  3
ab c de f 
z yx w vu

### step 2 : grouping
abz cyx dew fvu

### step 3 : reversing
abz xyc dew uvf

### line 1 patterns
i   |      L,R  |  sec  |        L      |      R
----+-----------+-------+---------------+-----------
0   |   i+0,i+2 |   ab  | i+(i+1)//2    | i+(i+4)//2
1   |   i+1,i+2 |   c   | i+(i+1)//2    | i+(i+4)//2
2   |   i+1,i+3 |   de  | i+(i+1)//2    | i+(i+4)//2
3   |   i+2,i+3 |   f   | i+(i+1)//2    | i+(i+4)//2
4   |   i+2,i+4 |   gh  | i+(i+1)//2    | i+(i+4)//2
5   |   i+3,i+4 |   i   | i+(i+1)//2    | i+(i+4)//2
6   |   i+3,i+5 |   jk  | i+(i+1)//2    | i+(i+4)//2
7   |   i+4,i+5 |   l   | i+(i+1)//2    | i+(i+4)//2


### line 2 patterns
i   |      L,R  |  sec  |        L      |      R
----+-----------+-------+---------------+-----------
0   |   i+0,i+1 |   z   | i+(i//2)      | i+(i+3)//2
1   |   i+0,i+2 |   yx  | i+(i//2)      | i+(i+3)//2
2   |   i+1,i+2 |   w   | i+(i//2)      | i+(i+3)//2
3   |   i+1,i+3 |   vu  | i+(i//2)      | i+(i+3)//2
4   |   i+2,i+3 |   t   | i+(i//2)      | i+(i+3)//2
5   |   i+2,i+4 |   sr  | i+(i//2)      | i+(i+3)//2
6   |   i+3,i+4 |   q   | i+(i//2)      | i+(i+3)//2
7   |   i+3,i+5 |   po  | i+(i//2)      | i+(i+3)//2
```