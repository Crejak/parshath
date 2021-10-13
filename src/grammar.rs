use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Hash)]
pub struct Variable {
    pub name: String
}

impl Variable {
    pub fn from(name: String) -> Self {
        Variable {
            name
        }
    }
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}

#[derive(Debug, Eq)]
pub enum Terminal {
    Char(char),
    Epsilon
}

impl PartialEq<Terminal> for Terminal {
    fn eq(&self, other: &Terminal) -> bool {
        match (self, other) {
            (Terminal::Char(c1), Terminal::Char(c2)) => c1 == c2,
            (Terminal::Epsilon, Terminal::Epsilon) => true,
            _ => false
        }
    }

    fn ne(&self, other: &Terminal) -> bool {
        !(self == other)
    }
}

impl Hash for Terminal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Terminal::Char(c) => c.hash(state),
            Terminal::Epsilon => "epsilon".hash(state)
        }
    }
}

#[derive(Debug, Eq)]
pub enum Symbol {
    Var(Variable),
    Ter(Terminal),
    End
}

impl Symbol {
    pub fn var(name: String) -> Self {
        Symbol::Var(Variable {
            name
        })
    }

    pub fn ter(char: char) -> Self {
        Symbol::Ter(Terminal::Char(char))
    }

    pub fn eps() -> Self {
        Symbol::Ter(Terminal::Epsilon)
    }
}

impl PartialEq<Symbol> for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        match (self, other) {
            (Symbol::Var(v1), Symbol::Var(v2)) => v1.name == v2.name,
            (Symbol::Ter(t1), Symbol::Ter(t2)) => t1 == t2,
            (Symbol::End, Symbol::End) => true,
            _ => false
        }
    }

    fn ne(&self, other: &Symbol) -> bool {
        !(self == other)
    }
}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Symbol::Ter(ter) => ter.hash(state),
            Symbol::Var(var) => var.hash(state),
            Symbol::End => "end".hash(state)
        }
    }
}

#[derive(Debug)]
pub struct Rule {
    pub left: Variable,
    pub right: Vec<Symbol>
}

impl Rule {
    pub fn from(left: String, right: Vec<Symbol>) -> Self {
        Rule {
            left: Variable::from(left),
            right
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Neutral,
    InVar,
    InTer
}

#[derive(Debug)]
pub struct Grammar {
    rules: Vec<Rule>
}

impl Grammar {
    pub fn from(source: &str) -> Grammar {
        let mut rules = Vec::new();

        let lines = source.lines();

        for (line_index, line) in lines.enumerate() {
            let mut left = None;
            let mut right = Vec::new();
            let mut current_variable = String::new();
            let mut state = State::Neutral;
            let splits: Vec<&str> = line.split("::=").collect();

            if splits.len() != 2 {
                panic!("Found {} rule divide (::=) on line {}, expected 1", splits.len(), line_index);
            }

            // left
            let left_str = splits[0];
            for (_char_index, char) in left_str.chars().enumerate() {
                if state == State::InVar {
                    if char == '>' {
                        left = Some(current_variable.clone());
                        current_variable.clear();
                        state = State::Neutral;
                        break;
                    }
                    current_variable.push(char);
                } else if char == '<' {
                    state = State::InVar;
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
                        rules.push(Rule::from(left.clone().unwrap(), right));
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

            rules.push(Rule::from(left.unwrap(), right));
        }

        Grammar {
            rules
        }
    }

    pub fn rules(&self) -> &Vec<Rule> {
        &self.rules
    }
}
