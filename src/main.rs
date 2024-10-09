pub mod disemvowel;

use crate::disemvowel::disemvowel;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // This should be called with two command line arguments,
    // both file names. The first should be the name of a file
    // containing the text to disemvowel, and the second should
    // be the file we want to write the disemvoweled text to.
    let args: Vec<String> = env::args().collect();

    //TODO: Panic if not enough arguments are provided
    //Panic should output the string "Not enough arguments"

    //TODO:
    //  * Pass an argument to read_file to read the original text
    //  * Pass that to disemvowel to remove the vowels
    //  * Write the disemvoweled text using write_file

    // Replace String::from("dummy text") with what you get from read_file
    let s = String::from("dummy text");

    let s_disemvowel = disemvowel(&s);

    // Use command-line arguments for the name of the file,
    // and s_disemvowel for the text to write out.
    write_file(Path::new("dummy.txt"), "output string");
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("Could not read the file")
}

fn write_file(path: &Path, s: &str) {
    fs::write(path, s).expect("Unable to write file");
}

// Everything from here down is Rust test code. You shouldn't need to
// change any of this.
//
// Use `cargo test` to run all these tests. All the tests will initially
// fail because there's no definition for the `disemvowel` function. Add
// that up above and work to get the tests to pass. See the lab write-up
// for some tips.

// Tests that check that the correct panics are generated when
// there aren't the correct number of command line arguments
// or the input file isn't readable.
#[cfg(test)]
mod tests {
    use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
    use predicates::prelude::predicate;
    use std::process::Command; // Run programs // Used for writing assertions

    #[test]
    fn requires_two_arguments() {
        let mut cmd = Command::cargo_bin("disemvowel-in-rust").unwrap();
        cmd.arg("1");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Not enough arguments"));
    }

    #[test]
    fn requires_read_file() {
        let mut cmd = Command::cargo_bin("disemvowel-in-rust").unwrap();
        cmd.arg("/this/path/does/not/exist")
            .arg("output/path/does/not/matter");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Could not read the file"));
    }
}
