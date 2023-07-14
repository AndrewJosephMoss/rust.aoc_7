use nom::{IResult, ToUsize};
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::sequence::separated_pair;
use nom::branch::alt;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
enum Line<'a> {
    CdUp,
    CdIn(&'a str),
    CdRoot,
    Ls,
    Dir(&'a str),
    File(File)
}

#[derive(PartialEq, Debug)]
struct File {
    pub size: usize,
    pub name: String,
}

pub fn process_part_1(input: &str) -> usize {
    let mut dirs = vec!["/"];
    let mut dir_sizes = HashMap::<String, usize>::new();
    input.lines().map(|line| parse_line(line).unwrap().1).for_each(|cmd| {
        match cmd {
            Line::CdUp => {
                if dirs.len() > 1 {
                    dirs.pop();
                }
            },
            Line::CdRoot => {
                dirs.truncate(1);
            },
            Line::CdIn(dir) => {
                dirs.push(dir);
            },
            Line::Ls => (),
            Line::Dir(_) => (),
            Line::File(file) => {
                let mut dir_path = String::from("");
                dirs.iter().for_each(|dir| {
                    dir_path += dir;
                    dir_sizes.entry(dir_path.clone()).and_modify(|size| *size += file.size).or_insert(file.size);
                });
            }
        }
    });
    
    let sum_of_below_100_000: usize = dir_sizes.iter().filter(|(_, size)| **size <= 100_000).map(|(_, size)| size).sum();
    sum_of_below_100_000
}


pub fn process_part_2(input: &str) -> usize {
    let mut dirs = vec!["/"];
    let mut dir_sizes = HashMap::<String, usize>::new();
    input.lines().map(|line| parse_line(line).unwrap().1).for_each(|cmd| {
        match cmd {
            Line::CdUp => {
                if dirs.len() > 1 {
                    dirs.pop();
                }
            },
            Line::CdRoot => {
                dirs.truncate(1);
            },
            Line::CdIn(dir) => {
                dirs.push(dir);
            },
            Line::Ls => (),
            Line::Dir(_) => (),
            Line::File(file) => {
                let mut dir_path = String::from("");
                dirs.iter().for_each(|dir| {
                    dir_path += dir;
                    dir_sizes.entry(dir_path.clone()).and_modify(|size| *size += file.size).or_insert(file.size);
                });
            }
        }
    });
    
    let root_size = dir_sizes.get("/").unwrap();
    let diff = 70_000_000 - root_size;
    let req = 30_000_000 - diff;
    let mut possible_dirs = dir_sizes.into_iter().filter(|dir| dir.1 >= req).collect::<Vec<(String, usize)>>();
    possible_dirs.sort_by(|a, b| a.1.cmp(&b.1));
    let smallest = &possible_dirs[0];
    smallest.1
}


fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, line) = alt((parse_cd, parse_dir, parse_file, parse_ls))(input)?;
    Ok((input, line))
}

fn parse_ls(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ ")(input)?;
    Ok((input, Line::Ls))
}

fn parse_cd(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("$ cd ")(input)?;
    let cmd = match input {
        "/" => Line::CdRoot,
        ".." => Line::CdUp,
        dir => Line::CdIn(dir)
    };
    Ok((input, cmd))
}

fn parse_dir(input: &str) -> IResult<&str, Line> {
    let (input, _) = tag("dir ")(input)?;
    Ok((input, Line::Dir(input)))
}

fn parse_file(input: &str) -> IResult<&str, Line> {
    let (input, (size, name)) = separated_pair(nom::character::complete::u32
        , tag(" "), many1(one_of("abcdefghijklmnopqrstuvwxyz.")))(input)?;
    Ok((input, Line::File(File {
        size: size.to_usize(),
        name: name.into_iter().collect()
    })))
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_process_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let result = process_part_1(&input);
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_parse_line() {
        let input = "dir a";
        let result = parse_line(input).unwrap().1;
        assert_eq!(result, Line::Dir("a"));

        let input = "5555 h.ytk";
        let result = parse_line(input).unwrap().1;
        assert_eq!(result, Line::File(File {
            name: String::from("h.ytk"),
            size: 5555
        }));

        let input = "$ cd /";
        let result = parse_line(input).unwrap().1;
        assert_eq!(result, Line::CdRoot);

        let input = "$ cd ..";
        let result = parse_line(input).unwrap().1;
        assert_eq!(result, Line::CdUp);

        let input = "$ cd a";
        let result = parse_line(input).unwrap().1;
        assert_eq!(result, Line::CdIn("a"));

        let input = "$ ls";
        let result = parse_line(input).unwrap().1;
        assert_eq!(result, Line::Ls);

        let input = "garbagesldkjfls";
        let result = parse_line(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_dir() {
        let input = "dir a";
        let result = parse_dir(input).unwrap().1;
        assert_eq!(result, Line::Dir("a"));

        let input = "safdjkosljs";
        let result = parse_dir(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_file() {
        let input = "239849 g.txt";
        let result = parse_file(input).unwrap().1;
        assert_eq!(result, Line::File(File {
            size: 239849,
            name: String::from("g.txt")
        }));

        let input = "sdfijopsjfd";
        let result = parse_file(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_ls() {
        let input = "$ ls";
        let result = parse_ls(&input).unwrap().1;
        assert_eq!(result, Line::Ls);

        let input = "askfjslf";
        let result = parse_ls(input);
        assert!(result.is_err())
    }

    #[test]
    fn test_parse_cd() {
        let input = "$ cd /";
        let result = parse_cd(input).unwrap().1;
        assert_eq!(result, Line::CdRoot);

        let input = "$ cd ..";
        let result = parse_cd(input).unwrap().1;
        assert_eq!(result, Line::CdUp);

        let input = "$ cd a";
        let result = parse_cd(input).unwrap().1;
        assert_eq!(result, Line::CdIn("a"));
        
        let input = "$ cd d";
        let result = parse_cd(input).unwrap().1;
        assert_eq!(result, Line::CdIn("d"));

        let input = "asdjfqksf";
        let result = parse_cd(input);
        assert!(result.is_err());
    }
}