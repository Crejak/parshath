use std::collections::HashMap;
use crate::grammar::*;

// Parser
#[derive(Debug)]
pub struct Parser<'a> {
    table: HashMap<(&'a Symbol, &'a Variable), &'a Rule>
}

pub struct Node<'a> {
    parent: Option<&'a Node<'a>>,
    children: Vec<Node<'a>>,
    symbol: &'a Symbol
}

impl<'a> Parser<'a> {
    pub fn from(grammar: &'a Grammar) -> Self {
        let mut table: HashMap<(&Symbol, &Variable), &Rule> = HashMap::new();

        for rule in grammar.rules() {
            let fis = Parser::first_set(grammar, &rule.right);
            for sym in fis {
                if let Some(_) = table.insert((sym, &rule.left), &rule) {
                    panic!("Rule already in table");
                }
            }
            if Parser::eps(grammar, &rule.right) {
                let fos = Parser::follow_set(&grammar, &rule.left);
                for sym in fos {
                    if let Some(_) = table.insert((sym, &rule.left), &rule) {
                        panic!("Rule already in table");
                    }
                }
            }
        }

        Parser {
            table
        }
    }

    fn first_set(grammar: &'a Grammar, expr: &'a [Symbol]) -> Vec<&'a Symbol> {
        let mut set = Vec::new();

        let first_symbol = expr.first().unwrap();
        match first_symbol {
            Symbol::Ter(ter) => if let Terminal::Char(_) = ter {
                set.push(first_symbol)
            },
            Symbol::Var(var) => {
                for rule in grammar.rules() {
                    if rule.left != *var {
                        continue;
                    }
                    set.append(&mut Parser::first_set(grammar, &rule.right));
                }
            },
            Symbol::End => panic!()
        }

        set
    }

    fn eps(grammar: &'a Grammar, expr: &'a [Symbol]) -> bool {
        for sym in expr {
            if Parser::eps_symbol(grammar, sym) == false {
                return false;
            }
        }
        true
    }

    fn eps_symbol(grammar: &'a Grammar, sym: &'a Symbol) -> bool {
        match sym {
            Symbol::Ter(ter) => *ter == Terminal::Epsilon,
            Symbol::Var(var) => {
                let mut eps_of_right = false;
                'rule: for rule in grammar.rules() {
                    if rule.left != *var {
                        continue;
                    }
                    'symbol: for symbol in &rule.right {
                        if let Symbol::Var(variable) = &symbol {
                            if variable == var {
                                continue 'symbol;
                            }
                        }
                        if Parser::eps_symbol(grammar, symbol) == false {
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

    fn follow_set(grammar: &'a Grammar, var: &'a Variable) -> Vec<&'a Symbol> {
        let mut set = Vec::new();

        if grammar.rules().first().unwrap().left == *var {
            set.push(&Symbol::End);
        }
        for rule in grammar.rules() {
            for (index, sym) in rule.right.iter().enumerate() {
                if let Symbol::Var(variable) = &sym {
                    if variable == var {
                        let (_ , right_expr) = &rule.right.split_at(index + 1);
                        let eps = Parser::eps(grammar, right_expr);
                        if (eps || right_expr.is_empty()) && var != &rule.left {
                            let fos = &mut Parser::follow_set(grammar, &rule.left);
                            set.append(fos);
                        }
                        if !right_expr.is_empty() {
                            let fs = &mut Parser::first_set(grammar, right_expr);
                            set.append(fs);
                        }
                    }
                }
            }
        }

        set
    }

    // fn parse<T>(&self, source: T)
}