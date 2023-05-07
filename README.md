# Probability - Gamblers Fallacy
Testing out intuition about the Law of Large Numbers versus the Gamblers Fallacy.

# Usage

```
./probability stdout --total 100 --apps 100
./probability csv --total 10000 --apps 10000 > results.csv
```

# Results

10,000 times executed 10,000 coin flips.  100,000,000 flips total.

| Kind | AVERAGE of accuracy | SUM of money | AVERAGE of money |
| --- | --- | --- | --- |
| Opposite	| 0.50000831	| 6400	| 0.64 |
| Control	| 0.50003331	| 7520	| 0.752 |
| Flipper	| 0.50001001	| 8078	| 0.8078 |
| Money	| 0.50000831	| 8698	| 0.8698 |
| Grand Total	| 0.500014985	| 30696	| 0.7674 |

The average accuracy is always around 0.5, in essense The Gambler's Fallacy.

* **Control** - either bets 2 or 1 randomly, and guesses heads or tails randomly.
* **Flipper** - guesses heads then tails, bets in sequence [2, 2, 1, 1] then repeat.
* **Opposite** - always guesses the opposite of last run, bets [2, 2, 1, 1]
* **Money** - always guesses the opposite, bets 2 * length of run, so if the run is 20 heads in a row it will bet 40 on tails

The amount of money won is slightly higher for Money, but not significantly more than flipper which mechanically bets an even amount according to a pattern.

I think this shows merely that higher risk gives higer reward.

# Future Work

Calculate the probability of each run length, and make betters that bet randomly according to the distribution of that p over the total lifetime of the game.

For example:
* 50% of the time bet 1
* 25% of the time bet 2
* 12.5% of the time bet 4
* 6.25% of the time bet 8

Then have money use 2^run_length to calculate it's bet.  Both should be betting the same amount the same number of times, but one has a strategy based on previous flips, and the other is random or mechanical.