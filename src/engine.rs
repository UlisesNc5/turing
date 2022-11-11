use std::collections::hash_map;
use super::errors::*;

pub type State = String;
pub type Symbol = char;

#[derive(Default, Debug)]
pub enum Moves{
    Right,
    Left,
    #[default]
    Stay
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct InData{
    pub in_state : State,
    pub in_symbol: Symbol,
}

impl InData{
    fn new(in_state: State, in_symbol: char) -> Self{
        InData{in_state, in_symbol}
    }

    fn set(in_state: &State, in_symbol: &char) -> Self{
        InData{in_state: in_state.clone(), in_symbol: in_symbol.clone()}
    }
}

#[derive(Debug)]
struct OutData{
    pub out_state : State,
    pub out_symbol: Symbol,
    pub out_move  : Moves,
}

impl OutData{
    fn new(out_state: State, out_symbol: Symbol, out_move : Moves) -> Self{
        OutData{out_state, out_symbol, out_move}
    }
}

#[derive(Debug)]
pub struct Rule{
    pub in_state  : State,
    pub in_symbol : Symbol,
    pub out_state : State,
    pub out_symbol: Symbol,
    pub out_move  : Moves,
}

impl Rule{
    pub fn new() -> Self{
        Rule { in_state: State::new(), in_symbol: '\0', out_state: State::new(), out_symbol: '\0', out_move: Moves::Stay}
    }
}

#[derive(Debug)]
pub struct Program{
    pub name: State,
    pub init: State,
    pub end : State,

    pub curr : State,
    pub tape : String,
    pub pos  : usize,
    pub min  : i32,
    map  : hash_map::HashMap<InData, OutData>,
}

pub enum Current{
    Ended,
    Continue,
    Died,
}

pub struct ProCopy{
    pub curr : State,
    pub tape : String,
    pub pos  : usize,
    pub min  : i32,
    pub mov  : Moves,
}

impl Program{
    pub fn new() -> Self{
        let mut new = Program {
            name : State::new(),
            init : State::new(),
            end  : State::new(),
            curr : State::new(),
            tape : String::new(),
            pos  : 0usize,
            min  : 0i32,
            map  : hash_map::HashMap::new(),
        };

        new.tape += "\0";
        return new;
    }

    //only compile thing in this file
    pub fn add_rule(&mut self, mut rule : Rule) -> Result<(),Error>{
        if rule.in_symbol == '_'{
            rule.in_symbol = '\0';
        }
        if rule.out_symbol == '_'{
            rule.out_symbol = '\0';
        } match self.map.insert(
             InData::new(rule.in_state, rule.in_symbol),
            OutData::new(rule.out_state, rule.out_symbol, rule.out_move),
        ){
            None => Ok(()),
            Some(_) => {Error::new(ErrorCode::DuplicateDelta)},
        }
    }

    pub fn set_init(&mut self, init: State){
        self.init = init.clone();
        self.curr = init;
    }

    pub fn set_tape(&mut self, ntape : String){
        self.tape = ntape;
    }

    pub fn step(&mut self) -> Current{
        if self.end == self.curr{
            return Current::Ended;
        }
        let in_data = InData::set(&self.curr, &self.tape.chars().nth(self.pos).unwrap());

        let out_data = match self.map.get(&in_data){
            Some(s) => s,
            None => {return Current::Died;},
        };
    
        self.tape.replace_range(self.pos..self.pos+1, out_data.out_symbol.to_string().as_str());
        self.curr = out_data.out_state.clone();
        match out_data.out_move{
            Moves::Right => {
                self.pos += 1;
                if self.pos + 1 == self.tape.len(){
                    self.tape.push('\0');
                }
            },
            Moves::Left  => {
                if self.pos == 0 {
                    self.min -= 1;
                    self.pos  = 0;
                    self.tape = String::from("\0") + self.tape.as_str();
                }
                else{
                    self.pos -= 1;
                }
            },
            Moves::Stay  => {},
        }

        Current::Continue
    }
}
