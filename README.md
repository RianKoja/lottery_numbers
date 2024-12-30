# Get Lottery Numbers

This repository provides a method to optimally choose lottery games by:

1. **Ensuring no two games share three numbers in common**  
   This means that if a partial prize is awarded by matching three or more numbers, the likelihood of winning a partial prize is optimized.

2. **Ensuring no numbers below a given threshold are contained in these games**  
   A suggested threshold is 32, since many people pick "birthday" or "favorite" numbers that rarely go higher than 20. Excluding lower numbers helps minimize the chance of sharing any potential prize with those who pick common low numbers.

## Setup

1. Adjust the variables in `config.toml`.  
2. Recommended: set your own seed for the random number generator. This helps ensure that you won’t be playing the exact same games as someone else using this repository.  
3. The initial set of games (`games`) can be set with your lucky or favorite game sets.

## Running

To generate your set of lottery games, run:
```sh
cargo run
```
This will produce `optimized_games.csv`.

For a quick correctness check (written in Python), run:
```sh
python test_set.py
```
Both steps (Rust and Python) can also be executed in Docker by running:
```sh
bash run.sh
```

A file named `optimized_games.csv` will be created on the root folder.

## Testing

Run tests locally:
```sh
cargo test
```
Or run tests in a Docker container:
```sh
bash test.sh
```

## Contributing

Pull requests are welcome. Feel free to fork this repository and submit your changes.

> **May the odds be ever in your favor.**

