# Probability
Testing out intuition about the Law of Large Numbers versus the Gamblers Fallacy.

# Results
```
total runs: 1000000
longest run: 19
Control: 0.500046, money: 1000092
Flipper: 0.499792, money: 999584
Opposite: 0.499609, money: 999218
Money: 0.499609, money: 1000409

total runs: 1000000
longest run: 20
Control: 0.499658, money: 999316
Flipper: 0.500396, money: 1000792
Opposite: 0.499589, money: 999178
Money: 0.499589, money: 1000543

total runs: 1000000
longest run: 18
Control: 0.500814, money: 1001628
Flipper: 0.5002, money: 1000400
Opposite: 0.499482, money: 998964
Money: 0.499482, money: 996945
```

Guessing randomly (control) performs equal to mechanically guessing true/false (flipper) and guessing the opposite of the last answer.

The accuracy is about the same for all predictors.  But "Money" seems to be making more money than the others which bet 1 every time, just not all the time as seen in result 3.

I think the reason for this is that yes in a single variable situation, the probability is still 0.5 for a fair coin, however there is another variable money, and the probability it will switch with Large Numbers is higher as the run increases.

Still need to do more tests.
