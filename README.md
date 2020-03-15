# musicdeck
Generating anki flashcards for learning musical theory.

I am writing this in rust for the lolz.

Currently I used this for generating major scales:

```.bash
> cargo run > /tmp/music.txt

# outputs:
3 enharmonics; C# F# B# <-> Db Gb Cb
major scale of C; C D E F G A B
major scale of G; G A B C D E F#
major scale of D; D E F# G A B C#
major scale of A; A B C# D E F# G#
major scale of E; E F# G# A B C# D#
major scale of Ab; Ab Bb C Db Eb F G
major scale of Eb; Eb F G Ab Bb C D
major scale of Bb; Bb C D Eb F G A
major scale of F; F G A Bb C D E
major scale of Db; (<-> C#) Db Eb F Gb Ab Bb C
major scale of Gb; (<-> F#) Gb Ab Bb Cb Db Eb F
major scale of Cb; (<-> B) Cb Db Eb E Gb Ab Bb
major scale of C#; (<-> Db) C# D# F F# G# A# C
major scale of F#; (<-> Gb) F# G# A# B C# D# F
major scale of B; (<-> Cb) B C# D# E F# G# A#
```
