# Rusty Dice

### Random Numbers in Rust

## Features

## `d(sides: i8) -> i8` 
Returns random integer in range [1, sides]
- Output range: 1-127
- Max number of sides: 127
- Common examples:
```
d2:   d(2)   -> [1..2]
d4:   d(4)   -> [1..4]
d6:   d(6)   -> [1..6]
d8:   d(8)   -> [1..8]
d10:  d(10)  -> [1..10]
d12:  d(12)  -> [1..12]
d20:  d(20)  -> [1..20]
d100: d(100) -> [1..100]
```

## `dice(rolls: i8, sides: i8) -> i16` 
Returns random integer in range [rolls, rolls * sides]
- Output range: 1-16129 (127^2)
- Max number of sides: 127
- Max number of rolls: 127
- Common examples:
```
2d10: dice(2, 10) -> [2..20]
3d6:  dice(3, 6)  -> [3..18]
6d8:  dice(6, 8)  -> [6..48]
```

## `ability_dice(power: i8) -> i8`
Returns sum of top 3 rolls of Nd6 where N = power. Used to determine the starting 
ability scores of characters; strength, dexterity, charisma etc. Always in range 3-18, higher power will produce higher 
average results. Typically power = 4, but other values (3-9) can be used at the discretion of the DM.

## `ability_check(dc: i8, bonus: i8) -> bool`
True indicates a success, false indicates a failure. DC ranges from 8 (very easy) to
25 (nearly impossible). Bonuses range from -5 to +5, typically characters start with zero bonus and retire
at max level with +5. A negative bonus can be the result of a curse or other negative effect.

## `attack_roll(ac: i8, bonus: i8) -> bool`
True indicates hit, false indicates miss. AC ranges from 8 (very easy to hit) to
25 (nearly impossible). Bonuses range from -5 to +5, typically characters start with zero bonus and retire
at max level with +5. A negative bonus can be the result of a curse or other negative effect.
