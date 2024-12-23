use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;

/// Represents a set of unique numbers with functionality for manipulation and persistence.
#[derive(Serialize, Deserialize)]
pub struct NumberSet {
    /// The internal hash set storing unique numbers.
    numbers: HashSet<i64>,
}

impl NumberSet {
    /// Creates a new, empty `NumberSet`.
    pub fn new() -> Self {
        Self {
            numbers: HashSet::new(),
        }
    }

    /// Adds a number to the set.
    ///
    /// # Arguments
    /// * `number` - The number to be added.
    ///
    /// # Returns
    /// * `true` if the number was successfully added (not already present).
    /// * `false` if the number was already in the set.
    pub fn add_number(&mut self, number: i64) -> bool {
        self.numbers.insert(number)
    }

    /// Saves the `NumberSet` to a file in JSON format.
    ///
    /// # Arguments
    /// * `filename` - The path to the file where the data should be saved.
    ///
    /// # Returns
    /// * `Ok(())` on success.
    /// * An error if the file could not be written.
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self.numbers)?;
        fs::write(filename, serialized)
    }

    /// Loads a `NumberSet` from a file in JSON format.
    ///
    /// # Arguments
    /// * `filename` - The path to the file to load from.
    ///
    /// # Returns
    /// * `Ok(NumberSet)` if the file was successfully loaded and parsed.
    /// * An error if the file could not be read or parsed.
    #[cfg(test)]
    pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let contents = fs::read_to_string(filename)?;
        let numbers: HashSet<i64> = serde_json::from_str(&contents)?;
        Ok(Self { numbers })
    }

    /// Attempts to insert all numbers in a vector into the set.
    ///
    /// # Arguments
    /// * `numbers` - A vector of numbers to be added.
    ///
    /// # Returns
    /// * `true` if all numbers were successfully added (none were duplicates).
    /// * `false` if at least one number was already in the set.
    pub fn check_and_insert_all(&mut self, numbers: Vec<i64>) -> bool {
        if numbers.iter().any(|&num| self.numbers.contains(&num)) {
            return false; // At least one number already exists in the set
        }

        for num in numbers {
            self.numbers.insert(num);
        }
        true // All numbers successfully added
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_new() {
        let number_set = NumberSet::new();
        assert!(number_set.numbers.is_empty());
    }

    #[test]
    fn test_add_number() {
        let mut number_set = NumberSet::new();
        assert!(number_set.add_number(5)); // Successfully added
        assert!(!number_set.add_number(5)); // Duplicate, not added
    }

    #[test]
    fn test_save_and_load_from_file() -> std::io::Result<()> {
        // 1. Create and populate a NumberSet.
        let mut number_set = NumberSet::new();
        number_set.add_number(5);
        number_set.add_number(10);

        // 2. Create a temporary file. It’s removed automatically
        //    when `tmp_file` goes out of scope.
        let tmp_file = NamedTempFile::new()?;
        let tmp_path = tmp_file.path(); // Path to the temporary file

        // 3. Save the NumberSet to the temporary file.
        number_set.save_to_file(tmp_path.to_str().unwrap())?;

        // 4. Load it back and verify contents.
        let loaded_number_set = NumberSet::load_from_file(tmp_path.to_str().unwrap())?;
        assert_eq!(number_set.numbers, loaded_number_set.numbers);

        // No manual cleanup needed; `tmp_file` gets deleted automatically.
        Ok(())
    }

    #[test]
    fn test_check_and_insert_all() {
        let mut number_set = NumberSet::new();
        let numbers_to_insert = vec![1, 2, 3, 4, 5];

        // Insert all numbers successfully
        assert!(number_set.check_and_insert_all(numbers_to_insert.clone()));

        // Verify all numbers were added
        for &num in &numbers_to_insert {
            assert!(!number_set.add_number(num)); // Should already exist
        }

        // Attempt to insert a mix of existing and new numbers
        let new_numbers_to_insert = vec![6, 7, 8, 9, 5];
        assert!(!number_set.check_and_insert_all(new_numbers_to_insert));
    }
}
