# Rust Dice

## Random Numbers in Rust

### Features
### `d(sides: i8) -> i8` 
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

### `dice(rolls: i8, sides: i8) -> i16` 
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
