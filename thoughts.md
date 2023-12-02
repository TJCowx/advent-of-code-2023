# Just Random Thoughts and Hangups

Just going to place my thoughts and what I got hung up with on the days

## Day 1

### Part 1

Was pretty simple and had no hangups with it. Spent more time setting up the
project than actually working on the part.

### Part 2

Didn't account for cases such as `eightwo`/`eighthree` where the beginning of
the word was combined with the end of the previous. The eliminated my original
regex solution and just decided to do some pre-processing on the lines
before I worked on getting the numbers (eg: replace "one" with "o1e" and get 1
from that)
