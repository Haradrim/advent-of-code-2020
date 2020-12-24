use std::fs;
use std::{collections::HashMap, time::Instant};
use std::{error::Error, str::FromStr};

// TODO: bleh refactor this solution bit messy.
fn main() -> Result<(), Box<dyn Error>> {
    let rules = read_file("input.txt")?;

    let start = Instant::now();

    println!("Answer 1: {:?}", part_01(&rules, "Shiny gold".to_string()));
    println!("Completed in {:?}", start.elapsed());

    println!("Answer 2: {:?}", part_02(&rules, "Shiny gold".to_string()));
    println!("Completed in {:?}", start.elapsed());

    Ok(())
}

fn part_01(rules: &Vec<String>, bag_color: Color) -> usize {
    let rules = Rules::parse(rules);

    rules.bag_count_color(&bag_color)
}

fn part_02(rules: &Vec<String>, bag_color: Color) -> usize {
    let rules = Rules::parse(rules);

    rules.get_total_bags(&bag_color)
}

fn read_file(filename: &str) -> std::io::Result<Vec<String>> {
    let input = fs::read_to_string(filename)?;

    Ok(input.lines().map(|line| line.to_string()).collect())
}

type Color = String;

#[derive(Debug)]
struct Rules {
    content: HashMap<Color, Vec<BagItem>>,
}

impl Rules {
    fn parse(input: &Vec<String>) -> Rules {
        let mut content = HashMap::new();

        for line in input {
            let rule: Vec<String> = line
                .replace(" bags", "")
                .replace(" bag", "")
                .replace(".", "")
                .split("contain")
                .map(|e| e.to_string())
                .collect();

            let color = rule[0].trim().to_string().to_uppercase();

            let items: Vec<BagItem> = rule[1]
                .split(',')
                .filter_map(|item| item.parse().ok())
                .collect();

            content.insert(color, items);
        }

        Rules { content }
    }

    fn bag_count_color(&self, bag_color: &String) -> usize {
        let mut bag_count = 0;

        for (_, content) in self.content.iter() {
            let count = self.search_content(content, &bag_color.to_uppercase());
            if count > 0 {
                bag_count += 1
            }
        }

        bag_count
    }

    fn get_total_bags(&self, bag_color: &String) -> usize {
        let content = self.content.get(&bag_color.to_uppercase()).unwrap();

        self.count_bags(content)
    }

    fn count_bags(&self, content: &Vec<BagItem>) -> usize {
        let mut count = 0;

        count += content
            .iter()
            .map(|item| item.count * (1 + self.count_bags(self.content.get(&item.color).unwrap())))
            .sum::<usize>();

        count
    }

    fn search_content(&self, content: &Vec<BagItem>, bag_color: &String) -> usize {
        let mut count = 0;

        for bag in content.iter() {
            if bag.color.to_uppercase() == bag_color.to_uppercase() {
                count += bag.count;
            } else {
                count += match self.content.get(&bag.color) {
                    Some(content) => self.search_content(content, bag_color),
                    None => 0,
                }
            }
        }

        count
    }
}

#[derive(Debug)]
struct BagItem {
    count: usize,
    color: Color,
}

impl FromStr for BagItem {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count_string = s
            .trim()
            .chars()
            .take_while(|c| c.is_digit(10))
            .collect::<String>();

        let color = s
            .trim()
            .chars()
            .skip_while(|c| c.is_digit(10))
            .collect::<String>()
            .trim()
            .to_string()
            .to_uppercase();

        Ok(BagItem {
            count: count_string.parse()?,
            color,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_01() {
        let rules = read_file("example.txt").unwrap();

        assert_eq!(part_01(&rules, "Shiny gold".to_string()), 4);
    }

    #[test]
    fn example_02() {
        let rules = read_file("example.txt").unwrap();

        assert_eq!(part_02(&rules, "Shiny gold".to_string()), 32);
    }

    #[test]
    fn example_02_1() {
        let rules = read_file("example_02.txt").unwrap();

        assert_eq!(part_02(&rules, "Shiny gold".to_string()), 126);
    }
}
