use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Get index of max value of collection of items that are Copy
pub fn get_max_index_copy<T: Ord + Copy>(slice: &[T]) -> Option<usize> {
    slice
        .iter()
        .enumerate()
        .max_by_key(|(_, &value)| value)
        .map(|(idx, _)| idx)
}
