use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Computes the binomial coefficient C(n, k), which represents the number
/// of ways to choose k elements from a set of n elements.
///
/// # Arguments
/// * `n` - Total number of items.
/// * `k` - Number of items to choose.
///
/// # Returns
/// * The binomial coefficient as `i64`.
fn binomial(n: i64, k: i64) -> i64 {
    if k == 0 || n == k {
        return 1;
    }
    if k > n {
        return 0;
    }

    let mut result: u128 = 1;
    let k = std::cmp::min(k, n - k); // Leverage symmetry
    for i in 1..=k {
        result *= (n - k + i) as u128;
        result /= i as u128;
    }
    result as i64
}

/// Converts a combination (vector of integers) to its combinadic number representation.
///
/// # Arguments
/// * `combination` - A vector of integers representing the combination.
///
/// # Returns
/// * The combinadic number as `i64`.
fn combinadic(combination: Vec<i64>) -> i64 {
    let k = combination.len() as i64;
    combination
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &ci)| acc + binomial(ci, k - i as i64))
}

/// Converts a combinadic number to its corresponding combination.
///
/// # Arguments
/// * `combination_no` - The combinadic number.
/// * `n` - Total number of elements in the set.
/// * `k` - Number of elements in the combination.
///
/// # Returns
/// * A vector of integers representing the combination.
fn inverse_combinadic(combination_no: i64, n: i64, k: i64) -> Vec<i64> {
    let mut combination_no = combination_no;
    let mut combination = vec![0; k as usize];
    let mut ci = n - 1;

    for i in (1..=k).rev() {
        while binomial(ci, i) > combination_no {
            ci -= 1;
        }
        combination[(k - i) as usize] = ci;
        combination_no -= binomial(ci, i);
        ci -= 1;
    }

    combination
}

/// Converts a combinadic number to its corresponding lottery game.
///
/// # Arguments
/// * `game_no` - The combinadic number representing the game.
///
/// # Returns
/// * A vector of integers representing the game numbers.
pub fn enum2game(game_no: i64) -> Vec<i64> {
    inverse_combinadic(game_no, 60, 6)
        .iter()
        .map(|&x| x + 1)
        .rev()
        .collect()
}

/// Converts a lottery game (set of numbers) to its unique combinadic number.
///
/// # Arguments
/// * `game` - A vector of integers representing the game numbers.
///
/// # Returns
/// * The combinadic number representing the game.
pub fn game2enum(game: Vec<i64>) -> i64 {
    combinadic(game.iter().map(|&x| x - 1).rev().collect())
}

/// Generates all unique triplets from a game (set of 6 numbers).
///
/// # Arguments
/// * `game` - A vector of exactly 6 integers.
///
/// # Returns
/// * A vector of vectors, each containing 3 integers (triplets).
pub fn game2triplets(game: Vec<i64>) -> Vec<Vec<i64>> {
    if game.len() != 6 {
        return vec![]; // Return an empty vector if the game does not have exactly 6 numbers
    }

    let mut triplets = Vec::new();
    for i in 0..4 {
        for j in i + 1..5 {
            for k in j + 1..6 {
                triplets.push(vec![game[i], game[j], game[k]]);
            }
        }
    }
    triplets
}

/// Converts a triplet to its unique combinadic number.
///
/// # Arguments
/// * `triplet` - A vector of 3 integers.
///
/// # Returns
/// * The combinadic number representing the triplet.
pub fn triplet2enum(triplet: Vec<i64>) -> i64 {
    combinadic(triplet.iter().map(|&x| x - 1).rev().collect())
}

/// Creates a game validation closure based on the given configuration values.
///
/// # Arguments
/// * `min_desired_number` - The minimum number allowed in a valid game.
/// * `max_number` - The maximum number allowed in a valid game.
///
/// # Returns
/// * A closure that takes a reference to a game (a vector of numbers) and returns `true`
///   if the game is invalid, or `false` if it is valid.
pub fn create_invalidate_game(
    min_desired_number: i64,
    max_number: i64,
) -> impl Fn(&Vec<i64>) -> bool {
    move |game: &Vec<i64>| {
        game.iter()
            .any(|&x| x < min_desired_number || x > max_number)
    }
}

/// Computes the maximum combinadic number based on the total numbers in the game (`n`)
/// and the number of numbers per game (`k`).
///
/// # Arguments
/// * `n` - The maximum number in a game (e.g., 60).
/// * `k` - The number of numbers per game (e.g., 6).
///
/// # Returns
/// * The maximum combinadic number as `i64`.
fn max_combinadic(n: i64, k: i64) -> i64 {
    let mut result: u128 = 1;
    let k = std::cmp::min(k, n - k); // Leverage symmetry
    for i in 1..=k {
        result *= (n - k + i) as u128;
        result /= i as u128;
    }
    result as i64
}

/// Creates a random number generator function for generating combinadic numbers.
/// The range of random numbers is determined dynamically based on `n` and `k`.
///
/// # Arguments
/// * `seed` - A `u64` seed for reproducible randomness.
/// * `n` - The maximum number in a game (e.g., 60).
/// * `k` - The number of numbers per game (e.g., 6).
///
/// # Returns
/// * A closure that generates random combinadic numbers.
pub fn create_combinadic_rng(
    seed: u64,
    max_number: i64,
    numbers_per_game: i64,
) -> impl FnMut() -> i64 {
    let mut rng = StdRng::seed_from_u64(seed);
    let max_combinadic = max_combinadic(max_number, numbers_per_game);

    move || rng.gen_range(0..max_combinadic)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test functions:

    /// Converts a combinadic number to its corresponding triplet.
    ///
    /// # Arguments
    /// * `triplet_no` - The combinadic number representing the triplet.
    ///
    /// # Returns
    /// * A vector of 3 integers representing the triplet.
    fn enum2triplet(triplet_no: i64) -> Vec<i64> {
        inverse_combinadic(triplet_no, 60, 3)
            .iter()
            .map(|&x| x + 1)
            .rev()
            .collect()
    }

    #[test]
    fn test_binomial() {
        assert_eq!(binomial(5, 3), 10);
        assert_eq!(binomial(6, 2), 15);
        assert_eq!(binomial(60, 6), 50_063_860);
        assert_eq!(binomial(10, 0), 1); // Edge case: k = 0
        assert_eq!(binomial(10, 10), 1); // Edge case: k = n
    }

    #[test]
    fn test_combinadic() {
        assert_eq!(combinadic(vec![2, 1, 0]), 0); // Lowest combinadic number
        assert_eq!(combinadic(vec![8, 6, 3, 1, 0]), 72); // Mid-range case
    }

    #[test]
    fn test_inverse_combinadic() {
        assert_eq!(inverse_combinadic(0, 4, 3), vec![2, 1, 0]);
        assert_eq!(inverse_combinadic(1, 4, 3), vec![3, 1, 0]);
        assert_eq!(
            inverse_combinadic(50_063_859, 60, 6),
            vec![59, 58, 57, 56, 55, 54]
        ); // Edge case: last combination
    }

    #[test]
    fn test_game2enum_and_enum2game() {
        let game = vec![1, 2, 3, 4, 5, 6];
        let game_no = game2enum(game.clone());
        assert_eq!(game_no, 0);
        assert_eq!(enum2game(game_no), game);

        let game = vec![10, 20, 30, 40, 50, 60];
        let game_no = game2enum(game.clone());
        assert!(game_no > 0);
        assert_eq!(enum2game(game_no), game);
    }

    #[test]
    fn test_game2triplets() {
        let game = vec![1, 2, 3, 4, 5, 6];
        let triplets = game2triplets(game);
        assert_eq!(triplets.len(), 20); // There should be 20 triplets from 6 numbers
        assert!(triplets.contains(&vec![1, 2, 3]));
        assert!(triplets.contains(&vec![4, 5, 6]));
    }

    #[test]
    fn test_triplet2enum_and_enum2triplet() {
        let triplet = vec![1, 2, 3];
        let triplet_no = triplet2enum(triplet.clone());
        assert_eq!(triplet_no, 0);
        assert_eq!(enum2triplet(triplet_no), triplet);

        let triplet = vec![58, 59, 60];
        let triplet_no = triplet2enum(triplet.clone());
        assert!(triplet_no > 0);
        assert_eq!(enum2triplet(triplet_no), triplet);
    }

    #[test]
    fn test_create_invalidate_game() {
        let min_desired_number = 31;
        let max_number = 60;

        // Create the invalidate_game closure
        let invalidate_game = create_invalidate_game(min_desired_number, max_number);

        // Test cases
        let valid_game = vec![32, 35, 41, 48, 50, 59];
        assert_eq!(invalidate_game(&valid_game), false, "Game should be valid");

        let game_with_too_small_number = vec![30, 35, 41, 48, 50, 59];
        assert_eq!(
            invalidate_game(&game_with_too_small_number),
            true,
            "Game should be invalid due to a number < min_desired_number"
        );

        let game_with_too_large_number = vec![33, 35, 41, 48, 50, 61];
        assert_eq!(
            invalidate_game(&game_with_too_large_number),
            true,
            "Game should be invalid due to a number > max_number"
        );

        let game_with_boundary_values = vec![31, 60, 35, 41, 48, 50];
        assert_eq!(
            invalidate_game(&game_with_boundary_values),
            false,
            "Game should be valid as all numbers are within the boundaries"
        );
    }
}
