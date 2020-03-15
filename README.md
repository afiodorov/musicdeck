# musicdeck
Generating anki flashcards for learning musical theory.

I am writing this in rust for the lolz.

Currently I used this for generating major scales:

```.bash
> cargo run > /tmp/music.txt

# outputs:
3 enharmonics; C# F# B# <-> Db Gb Cb
major scale of C; F C G D A E B
major scale of G; C G D A E B F#
major scale of D; G D A E B F# C#
major scale of A; D A E B F# C# G#
major scale of E; A E B F# C# G# D#
major scale of Ab; Db Ab Eb Bb F C G
major scale of Eb; Ab Eb Bb F C G D
major scale of Bb; Eb Bb F C G D A
major scale of F; Bb F C G D A E
major scale of Db; (<-> C#) Gb Db Ab Eb Bb F C
major scale of Gb; (<-> F#) Cb Gb Db Ab Eb Bb F
major scale of Cb; (<-> B) E Cb Gb Db Ab Eb Bb
major scale of C#; (<-> Db) F# C# G# D# A# F C
major scale of F#; (<-> Gb) B F# C# G# D# A# F
major scale of B; (<-> Cb) E B F# C# G# D# A#
```
