# RDS

# Google Code Jam 2017 Practice in Rust programming language.

###problem : 2014 Magic trick : 
    Recently you went to a magic show. You were very impressed by one of the tricks, so you decided to try to figure out the secret behind it!
    The magician starts by arranging 16 cards in a square grid: 4 rows of cards, with 4 cards in each row. Each card has a different number from 1 to 16 written on the side that is showing. Next, the magician asks a volunteer to choose a card, and to tell him which row that card is in.
    Finally, the magician arranges the 16 cards in a square grid again, possibly in a different order. Once again, he asks the volunteer which row her card is in. With only the answers to these two questions, the magician then correctly determines which card the volunteer chose. Amazing, right?
    You decide to write a program to help you understand the magician's technique. The program will be given the two arrangements of the cards, and the volunteer's answers to the two questions: the row number of the selected card in the first arrangement, and the row number of the selected card in the second arrangement. The rows are numbered 1 to 4 from top to bottom.
    Your program should determine which card the volunteer chose; or if there is more than one card the volunteer might have chosen (the magician did a bad job); or if there's no card consistent with the volunteer's answers (the volunteer cheated).

### Problem: 2012 Speaking in Tongues :
    We have come up with the best possible language here at Google, called Googlerese. To translate text into Googlerese, we take any message and replace each English letter with another English letter. This mapping is one-to-one and onto, which means that the same input letter always gets replaced with the same output letter, and different input letters always get replaced with different output letters. A letter may be replaced by itself. Spaces are left as-is.
    For example (and here is a hint!), our awesome translation algorithm includes the following three mappings: 'a' -> 'y', 'o' -> 'e', and 'z' -> 'q'. This means that "a zoo" will become "y qee".
    Googlerese is based on the best possible replacement mapping, and we will never change it. It will always be the same. In every test case. We will not tell you the rest of our mapping because that would make the problem too easy, but there are a few examples below that may help.
    Given some text in Googlerese, can you translate it to back to normal text?
    
    
INPUT :
```$xslt
30
ejp mysljylc kd kxveddknmc re jsicpdrysi
rbcpc ypc rtcsra dkh wyfrepkym veddknkmkrkcd
de kr kd eoya kw aej tysr re ujdr lkgc jv
hello i am the google code jam test data
how are you
aynny iynny aynny iynny aynny iynny aynny iynny aynny iynny aynny iynny aynny iynny aynny ieeeeeeeee
y n f i c w l b k u o m x s e v z p d r j g a t h a q set k oset xa ynfd
schr rkxc tesr aej dksl tkrb xc
wep rbedc tbe dvcyo ks y resljc ie ser dvcyo re erbcp vcevmc
seneia jsicpdrysid rbcx dksfc rbca ypc dvcyoksl xadrcpkcd ks rbc dvkpkr
rbkd kd de chfkrksl k bygc re le rbc nyrbpeex
kr tyd rbc ncdr ew rkxcd kr tyd rbc nmjpdr ew rkxcd
mcr mkvd ie tbyr bysid ie
rbkd bcpc kd ljsveticp yfrkgyrci rtcsra dcgcs fymkncp wjmm yjre se okfonyfo sykmrbpetksl xyabcx
k bygc ncdrci wpjkr dvkoc ysi xees set k dbymm ncdr aej rbc lja
eb byk kx ks jp fexvjrcp cyrksl aejp fbccqnjplcpd ysi leelmcpcdksl aejp rchrq
ys cac wep ys cac ysi y vklces wep y vklces
ymm aejp nydc ypc ncmesl re cppep rbc dveesa nypi
aej vkddci eww rbc fbkfocs myia
set kd rbc djxxcp ew ejp myfo ew ikdfesrcsr
na rbc vpkfoksl ew xa rbjxnd dexcrbksl tkfoci rbkd tya fexcd
ks y tepmi ew ikpctemgcd ysi mkesd dexcrkxcd rbc pypcdr fpcyrjpc kd y wpkcsi
lpccrksld fbccdc vevdkfmc rbc sjxncp aej bygc ikymci kd fjppcsrma ejr ew vepofbevd
tba ie vpelpyxxcpd ymtyad xkh jv bymmetccs ysi fbpkdrxyd
kx fexxysicp dbcvypi ysi rbkd kd xa wygepkrc vpenmcx es rbc leelmc feic uyx
w ew rte czjymd w ew esc czjymd esc
wep k ncrtccs rbpcc ysi s w ew k czjymd w ew k xksjd esc vmjd w ew k xksjd rte
bet ypc aej bemiksl jv ncfyjdc kx y veryre
ip qykjd ip qykjd ip qykjd ip qykjd eeeeeeeeeeeeb ip qykjd
tbeeeeeeeeeeeeeeeeeeeyyyyyyyyy k oset f vmjd vmjd

```
   
OUTPUT :

```$xslt
Case #1: our language is impossible to understand
Case #2: there are twenty six factorial possibilities
Case #3: so it is okay if you want to just give up
Case #4: xoggk d yl wxo vkkvgo ekso uyl wonw sywy
Case #5: xkf yto akj
Case #6: yabba dabba yabba dabba yabba dabba yabba dabba yabba dabba yabba dabba yabba dabba yabba dooooooooo
Case #7: a b c d e f g h i j k l m n o p q r s t u v y w x y z now i know my abcs
Case #8: next time wont you sing with me
Case #9: for those who speak in a tongue do not speak to other people
Case #10: nobody understands them since they are speaking mysteries in the spirit
Case #11: this is so exciting i have to go the bathroom
Case #12: it was the best of times it was the blurst of times
Case #13: let lips do what hands do
Case #14: this here is gunpowder activated twenty seven caliber full auto no kickback nailthrowing mayhem
Case #15: i have bested fruit spike and moon now i shall best you the guy
Case #16: oh hai im in ur computer eating your cheezburgers and googleresing your textz
Case #17: an eye for an eye and a pigeon for a pigeon
Case #18: all your base are belong to error the spoony bard
Case #19: you pissed off the chicken lady
Case #20: now is the summer of our lack of discontent
Case #21: by the pricking of my thumbs something wicked this way comes
Case #22: in a world of direwolves and lions sometimes the rarest creature is a friend
Case #23: greetings cheese popsicle the number you have dialed is currently out of porkchops
Case #24: why do programmers always mix up halloween and christmas
Case #25: im commander shepard and this is my favorite problem on the google code jam
Case #26: f of two equals f of one equals one
Case #27: for i between three and n f of i equals f of i minus one plus f of i minus two
Case #28: how are you holding up because im a potato
Case #29: dr zaius dr zaius dr zaius dr zaius ooooooooooooh dr zaius
Case #30: whoooooooooooooooooooaaaaaaaaa i know c plus plus
```