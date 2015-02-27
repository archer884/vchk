vchk
====

Four letter names are the best. The idea there was "vowel check", and what this program does can be mimicked at least in part by a shell one-liner even my granny could grok:

> `cat /usr/share/dict/words | grep '.*a.*e.*i.*o.*u.*'`

But no one in their right mind can write a regex that will tell me if a word has all of the vowels in it *but* they aren't in order. I realize that is a perfect example of the *No true Scotsman* fallacy, but I doubt it's one that people are going to argue over too much...

Actually, it may also be a perfect example of a *tautology.* That's right, kids, you're gonna need your thesaurus for this readme.

## Usage

`./vchk list` ## list all *panvowelular* words (don't look that one up--I just invented it)

`./vchk count` ## print the number of *panvowelular* words

`./vchk list sorted` ## print all *ordopanvowelular* words (that's words that have all the vowels *and* they're in order!)

`./vchk count sorted` ## ...you get the idea...

## Known bugs

The letter Y is occasionally a vowel, but my program calls bullshit on that. In fact, the `VWord` struct I store these words in is totally incapable of representing a sixth vowel. Too damn bad. :)
