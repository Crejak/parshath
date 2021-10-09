use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::State::{InVar, Neutral};

fn main() {
    let grammar = "<S> ::= \"(\" <L> \")\" | \"a\"
    <L> ::= <S> <L> | \"\"";
    let g = get_grammar(grammar).unwrap();
    println!("{:?}", g);
    let p = Parser::from(g);
    println!("{:?}", p);
}

#[derive(Debug, PartialEq)]
enum State {
    Neutral,
    InVar,
    InTer
}

#[derive(Debug, Eq, Hash)]
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

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }

    fn ne(&self, other: &Self) -> bool {
        self.name != other.name
    }
}

#[derive(Debug, Eq)]
enum Terminal {
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
enum Symbol {
    Var(Variable),
    Ter(Terminal),
    End
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

// Parser
#[derive(Debug)]
struct Parser<'a> {
    grammar: Grammar,
    table: HashMap<(&'a Symbol, &'a Variable), &'a Rule>,
    stack: Vec<Symbol>,
}

struct Node<'a> {
    parent: Option<&'a Node<'a>>,
    children: Vec<Node<'a>>,
    symbol: &'a Symbol
}

impl<'a> Parser<'a> {
    fn from(grammar: Grammar) -> Self {
        let mut parser = Parser {
            grammar,
            table: HashMap::new(),
            stack: vec![Symbol::End]
        };
        let mut table: HashMap<(&Symbol, &Variable), &Rule> = HashMap::new();

        for rule in &parser.grammar.rules {
            let fis = parser.first_set(&rule.right);
            for sym in fis {
                if let Some(_) = table.insert((sym, &rule.left), &rule) {
                    panic!("Rule already in table");
                }
            }
            if parser.eps(&rule.right) {
                let fos = parser.follow_set(&rule.left);
                for sym in fos {
                    if let Some(_) = table.insert((sym, &rule.left), &rule) {
                        panic!("Rule already in table");
                    }
                }
            }
        }

        println!("Table :");
        for (key, value) in table {
            println!("{:?} ==> {:?}", key, value);
        }

        parser
    }

    fn first_set(&'a self, expr: &'a [Symbol]) -> Vec<&'a Symbol> {
        let mut set = Vec::new();

        let first_symbol = expr.first().unwrap();
        match first_symbol {
            Symbol::Ter(ter) => if let Terminal::Char(_) = ter {
                set.push(first_symbol)
            },
            Symbol::Var(var) => {
                for rule in &self.grammar.rules {
                    if rule.left != *var {
                        continue;
                    }
                    set.append(&mut self.first_set(&rule.right));
                }
            },
            Symbol::End => panic!()
        }

        set
    }

    fn eps(&'a self, expr: &'a [Symbol]) -> bool {
        for sym in expr {
            if self.eps_symbol(sym) == false {
                return false;
            }
        }
        true
    }

    fn eps_symbol(&'a self, sym: &'a Symbol) -> bool {
        match sym {
            Symbol::Ter(ter) => *ter == Terminal::Epsilon,
            Symbol::Var(var) => {
                let mut eps_of_right = false;
                'rule: for rule in &self.grammar.rules {
                    if rule.left != *var {
                        continue;
                    }
                    'symbol: for symbol in &rule.right {
                        if let Symbol::Var(variable) = symbol {
                            if variable == var {
                                continue 'symbol;
                            }
                        }
                        if self.eps_symbol(symbol) == false {
                            continue 'rule;
                        }
                    }
                    eps_of_right = true;
                }
                eps_of_right
            },
            Symbol::End => panic!()
        }
    }

    fn follow_set(&'a self, var: &'a Variable) -> Vec<&Symbol> {
        let mut result = Vec::new();
        if self.grammar.rules.first().unwrap().left == *var {
            result.push(&Symbol::End);
        }
        for rule in &self.grammar.rules {
            for (index, sym) in rule.right.iter().enumerate() {
                if let Symbol::Var(variable) = sym {
                    if variable == var {
                        let (_ , right_expr) = &rule.right.split_at(index + 1);
                        let eps = self.eps(right_expr);
                        if (eps || right_expr.is_empty()) && var != &rule.left {
                            let fos = &mut self.follow_set(&rule.left);
                            result.append(fos);
                        }
                        if !right_expr.is_empty() {
                            let fs = &mut self.first_set(right_expr);
                            result.append(fs);
                        }
                    }
                }
            }
        }

        result
    }
}
