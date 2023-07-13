use nom::{IResult, ToUsize};
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::sequence::separated_pair;

#[derive(PartialEq, Debug)]
enum CMD<'a> {
    CdUp,
    CdIn(&'a str),
    CdRoot,
    Ls(Vec<File>),
}

#[derive(PartialEq, Debug)]
struct File {
    pub size: usize,
    pub name: String,
}

pub fn process_part_1(input: &str) -> usize {
    // input.lines().map(|line| {})
    95437
}

fn map_line_to_cmd(line: &str) -> CMD {
    todo!();
}

fn parse_cmd(input: &str) -> CMD {
    todo!();
}

fn parse_ls(input: &str) -> IResult<&str, CMD> {
    let (input, _) = tag("$ ")(input)?;
    Ok((input, CMD::Ls(Vec::<File>::new())))
}

fn parse_cd(input: &str) -> IResult<&str, CMD> {
    let (input, _) = tag("$ cd ")(input)?;
    let cmd = match input {
        "/" => CMD::CdRoot,
        ".." => CMD::CdUp,
        dir => CMD::CdIn(dir)
    };
    Ok((input, cmd))
}

fn parse_dir(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("dir ")(input)?;
    Ok((input, input))
}

fn parse_file(input: &str) -> IResult<&str, File> {
    let (input, (size, name)) = separated_pair(nom::character::complete::u32
        , tag(" "), many1(one_of("abcdefghijklmnopqrstuvwxyz.")))(input)?;
    Ok((input, File {
        size: size.to_usize(),
        name: name.into_iter().collect()
    }))
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
    fn test_parse_dir() {
        let input = "dir a";
        let result = parse_dir(input).unwrap().1;
        assert_eq!(result, "a");

        let input = "safdjkosljs";
        let result = parse_dir(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_file() {
        let input = "239849 g.txt";
        let result = parse_file(input).unwrap().1;
        assert_eq!(result, File {
            size: 239849,
            name: String::from("g.txt")
        });

        let input = "sdfijopsjfd";
        let result = parse_file(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_ls() {
        let input = "$ ls";
        let result = parse_ls(&input).unwrap().1;
        assert_eq!(result, CMD::Ls(Vec::<File>::new()));

        let input = "askfjslf";
        let result = parse_ls(input);
        assert!(result.is_err())
    }

    #[test]
    fn test_parse_cd() {
        let input = "$ cd /";
        let result = parse_cd(input).unwrap().1;
        assert_eq!(result, CMD::CdRoot);

        let input = "$ cd ..";
        let result = parse_cd(input).unwrap().1;
        assert_eq!(result, CMD::CdUp);

        let input = "$ cd a";
        let result = parse_cd(input).unwrap().1;
        assert_eq!(result, CMD::CdIn("a"));
        
        let input = "$ cd d";
        let result = parse_cd(input).unwrap().1;
        assert_eq!(result, CMD::CdIn("d"));

        let input = "asdjfqksf";
        let result = parse_cd(input);
        assert!(result.is_err());
    }
}