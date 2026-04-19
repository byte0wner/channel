use std::fs::read_to_string;
use rayon::prelude::*;

#[derive(Debug)]
enum Rule {
    DoNothing,                                      // :
    ConvertToLowercase,                             // l
    ConvertToUppercase,                             // u
    Reverse,                                        // r
    AppendCharacterToTheWord(u8),                   // $x
    PrefixTheWordWithCharacter(u8),                 // ^x
    DeleteTheFirstCharacter,                        // [
    DeleteTheLastCharacter,                         // ]
    ReplaceAllCharactersInTheWordWith(u8, u8),      // sXY
    PurgeAllCharactersFromTheWord(u8),              // @x
    Duplicate,                                      // d "Fred" -> "FredFred"
}

impl Rule {
    fn apply_rule(&self, password: &str) -> String {
        match self {
            Rule::DoNothing => password.to_string(),
            Rule::ConvertToLowercase => password.to_lowercase(),
            Rule::ConvertToUppercase => password.to_uppercase(),
            Rule::Reverse => password.chars().rev().collect(),
            Rule::AppendCharacterToTheWord(x) => format!("{}{}", password, *x as char),
            Rule::PrefixTheWordWithCharacter(x) => format!("{}{}", *x as char, password),
            Rule::DeleteTheFirstCharacter => password.chars().skip(1).collect(),
            Rule::DeleteTheLastCharacter => password[..password.len() - 1].chars().collect(),
            Rule::ReplaceAllCharactersInTheWordWith(x, y) => password.replace(*x as char, &(*y as char).to_string()),
            Rule::PurgeAllCharactersFromTheWord(x) => password.replace(*x as char, ""),
            Rule::Duplicate => format!("{}{}", password, password),
        }
    }
}

fn parse_rules(combined_rules: &[String]) -> Vec<Vec<Rule>> {
    let mut rules: Vec<Vec<Rule>> = vec![];

    for simple_rules in combined_rules {
        let mut combined_rule: Vec<Rule> = vec![];

        let parts = simple_rules.split(' ');

        let collection: Vec<&str> = parts.collect();

        for rule in &collection {
            match rule.as_bytes()[0] {
                b':' => combined_rule.push(Rule::DoNothing),
                b'l' => combined_rule.push(Rule::ConvertToLowercase),
                b'u' => combined_rule.push(Rule::ConvertToUppercase),
                b'r' => combined_rule.push(Rule::Reverse),
                b'$' => combined_rule.push(Rule::AppendCharacterToTheWord(rule.as_bytes()[1])),
                b'^' => combined_rule.push(Rule::PrefixTheWordWithCharacter(rule.as_bytes()[1])),
                b'[' => combined_rule.push(Rule::DeleteTheFirstCharacter),
                b']' => combined_rule.push(Rule::DeleteTheLastCharacter),
                b's' => combined_rule.push(Rule::ReplaceAllCharactersInTheWordWith(rule.as_bytes()[1], rule.as_bytes()[2])),
                b'@' => combined_rule.push(Rule::PurgeAllCharactersFromTheWord(rule.as_bytes()[1])),
                b'd' => combined_rule.push(Rule::Duplicate),
                _ => continue,
            };
            
        }
        rules.push(combined_rule);
    }
    return rules;
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let rules: Vec<String> = read_lines("hob064.rule");

    let words: Vec<String> = read_lines("wordlist.txt");

    let parsed_rules: Vec<Vec<Rule>> = parse_rules(&rules);

    let passwords: Vec<String> = words.par_iter()
        .flat_map(|word| {
            let mut word_results = Vec::new();

            for rule_set in &parsed_rules {
                let mut current_word = word.clone();
                
                for rule in rule_set {
                    current_word = rule.apply_rule(&current_word);
                }
                
                word_results.push(current_word);
            }

            word_results
        })
        .collect();

    
    for password in passwords {
        println!("{:?}", password);
    }
}