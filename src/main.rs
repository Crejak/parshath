use crate::State::{InVar, Neutral};

fn main() {
    let grammar = "<Test> ::= \"a\" <Test> | \"\"";
    let g = get_grammar(grammar);
    println!("{:?}", g);
}

#[derive(Debug, PartialEq)]
enum State {
    Neutral,
    InVar,
    InTer
}

#[derive(Debug)]
struct Variable {
    name: String
}

impl Variable {
    fn from(name: String) -> Self {
        Variable {
            name
        }
    }
}

#[derive(Debug)]
enum Terminal {
    Char(char),
    Epsilon
}

#[derive(Debug)]
enum Symbol {
    Var(Variable),
    Ter(Terminal)
}

impl Symbol {
    fn var(name: String) -> Self {
        Symbol::Var(Variable {
            name
        })
    }

    fn ter(char: char) -> Self {
        Symbol::Ter(Terminal::Char(char))
    }

    fn eps() -> Self {
        Symbol::Ter(Terminal::Epsilon)
    }
}

#[derive(Debug)]
struct Rule {
    left: Variable,
    right: Vec<Symbol>
}

impl Rule {
    fn from(left: String, right: Vec<Symbol>) -> Self {
        Rule {
            left: Variable::from(left),
            right
        }
    }
}

#[derive(Debug)]
struct Grammar {
    rules: Vec<Rule>
}

fn get_grammar(source: &str) -> Option<Grammar> {
    let mut rules = Vec::new();

    let lines = source.lines();

    for (line_index, line) in lines.enumerate() {
        let mut left = None;
        let mut right = Vec::new();
        let mut current_variable = String::new();
        let mut state = Neutral;
        let splits: Vec<&str> = line.split("::=").collect();

        if splits.len() != 2 {
            panic!("Found {} rule divide (::=) on line {}, expected 1", splits.len(), line_index);
        }

        // left
        let left_str = splits[0];
        for (_char_index, char) in left_str.chars().enumerate() {
            if state == InVar {
                if char == '>' {
                    left = Some(current_variable.clone());
                    current_variable.clear();
                    state = Neutral;
                    break;
                }
                current_variable.push(char);
            } else if char == '<' {
                state = InVar;
            }
        }

        if left == None {
            panic!("No variable found on left hand sign (line {})", line_index);
        }

        // right
        let right_str = splits[1];
        let mut chars_in_terminal = 0;
        for (_char_index, char) in right_str.chars().enumerate() {
            state = match (state, char) {
                (State::InVar, '>') => {
                    right.push(Symbol::var(current_variable.clone()));
                    current_variable.clear();
                    State::Neutral
                },
                (State::InVar, _) => {
                    current_variable.push(char);
                    State::InVar
                },
                (State::InTer, '"') => {
                    if chars_in_terminal == 0 {
                        right.push(Symbol::eps());
                    } else {
                        chars_in_terminal = 0;
                    }
                    State::Neutral
                },
                (State::InTer, _) => {
                    right.push(Symbol::ter(char));
                    chars_in_terminal += 1;
                    State::InTer
                },
                (State::Neutral, '|') => {
                    rules.push(Rule::from(left.clone()?, right));
                    right = Vec::new();
                    State::Neutral
                },
                (State::Neutral, '"') => {
                    State::InTer
                },
                (State::Neutral, '<') => {
                    State::InVar
                },
                (State::Neutral, _) => {
                    State::Neutral
                }
            };
        }

        rules.push(Rule::from(left?, right));
    }

    Some(Grammar {
        rules
    })
}
