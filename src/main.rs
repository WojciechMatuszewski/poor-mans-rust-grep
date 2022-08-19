use std::{
    env,
    error::Error,
    fs::{metadata, read_to_string},
};

use glob::glob;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let query = args.get(1).expect("You must provide the query");
    let path = args.get(2).expect("You must provide the location");

    println!("Searching for {} in {}", query, path);

    let md = metadata(path)?;
    if md.is_file() {
        find_in_file(path, query)?;
    }

    if md.is_dir() {
        let glob_pattern = dir_to_glob_pattern(path);
        let entries = glob(&glob_pattern)?;

        for entry in entries {
            match entry {
                Ok(path) => {
                    println!("\nFilename: {:?}", path);

                    let path = path.to_str().expect("Failed to convert to str");
                    find_in_file(path, query)?;
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    return Ok(());
}

fn find_in_file(path: &str, query: &str) -> Result<(), Box<dyn Error>> {
    let contents = read_to_string(path)?;
    let results = find_in_contents(&contents, query);

    for result in results {
        println!("{} {}", result.line_number, result.line_contents);
    }

    return Ok(());
}

#[derive(Debug)]
struct SearchResult {
    line_number: usize,
    line_contents: String,
}

fn find_in_contents<'a>(contents: &'a str, query: &str) -> Vec<SearchResult> {
    let matches: Vec<SearchResult> = contents
        .lines()
        .enumerate()
        .filter(|(_, line_contents)| {
            let found = match line_contents.find(query) {
                Some(_) => true,
                None => false,
            };
            return found;
        })
        .map(|(line_number, line_contents)| {
            return SearchResult {
                line_contents: String::from(line_contents),
                line_number: line_number + 1,
            };
        })
        .collect();

    return matches;
}

fn dir_to_glob_pattern(path: &str) -> String {
    if path == "." {
        return String::from("**/*.txt");
    }

    if path.ends_with("/") {
        let glob_pattern = format!("{}**/*.txt", path);
        return glob_pattern;
    }

    let glob_pattern = format!("{}/**/*.txt", path);
    return glob_pattern;
}

mod test_dir_to_glob_pattern {
    use crate::dir_to_glob_pattern;

    #[test]
    fn test_dot() {
        let dir_name = ".";
        let glob_pattern = dir_to_glob_pattern(dir_name);

        assert_eq!("**/*.txt", glob_pattern);
    }

    #[test]
    fn test_ends_with_slash() {
        let dir_name = "./src/";
        let glob_pattern = dir_to_glob_pattern(dir_name);

        assert_eq!("./src/**/*.txt", glob_pattern);
    }

    #[test]
    fn test_absolute_path() {
        let dir_name = "src";
        let glob_pattern = dir_to_glob_pattern(dir_name);

        assert_eq!("src/**/*.txt", glob_pattern);
    }
}

mod test_reading_file {
    use crate::{find_in_contents, SearchResult};

    #[test]
    fn test_find_in_contents_empty() {
        let contents = "";
        let matches = find_in_contents(contents, "Wojtek");

        assert_eq!(0, matches.len());
    }

    #[test]
    fn test_find_in_contents_matches() {
        let contents = "Wojtek\nMateusz Matuszewski\nWojtek";
        let matches = find_in_contents(contents, "Mateusz");
        assert_eq!(1, matches.len());

        let found_match = matches.get(0).unwrap();
        assert_eq!(2, found_match.line_number);
        assert_eq!(
            String::from("Mateusz Matuszewski"),
            found_match.line_contents
        )
    }
}
