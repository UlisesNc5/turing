use std::collections::hash_map;
use super::errors::*;

pub type State = String;

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
    pub in_symbol: String,
}

impl InData{
    fn new(in_state: String, in_symbol: String) -> Self{
        InData{in_state, in_symbol}
    }

    fn set(in_state: &String, in_symbol: &String) -> Self{
        InData{in_state: in_state.clone(), in_symbol: in_symbol.clone()}
    }
}

#[derive(Debug)]
struct OutData{
    pub out_state : State,
    pub out_symbol: String,
    pub out_move  : Moves,
}

impl OutData{
    fn new(out_state: String, out_symbol: String, out_move : Moves) -> Self{
        OutData{out_state, out_symbol, out_move}
    }
}

#[derive(Debug)]
pub struct Rule{
    pub in_state: State,
    pub in_symbol: String,
    pub out_state: State,
    pub out_symbol: String,
    pub out_move : Moves,
}

impl Rule{
    pub fn new() -> Self{
        Rule { in_state: String::new(), in_symbol: String::new(), out_state: String::new(), out_symbol: String::new(), out_move: Moves::Stay}
    }
}

#[derive(Debug)]
pub struct Program{
    pub name: String,
    pub init: State,
    pub end : State,

    curr : State,
    tape : Vec<String>,
    pos  : usize,
    min  : i32,
    map  : hash_map::HashMap<InData, OutData>,
}

pub enum Current{
    Ended,
    Continue,
    Died,
}

pub struct ProCopy{
    pub curr : State,
    pub tape : Vec<String>,
    pub pos  : usize,
    pub min  : i32,
    pub mov  : Moves,
}

impl Program{
    pub fn new() -> Self{
        Program {
            name    : String::new(),
            init    : String::new(),
            end     : String::new(),
            curr    : String::new(),
            tape    : Vec::new(),
            pos     : 0usize,
            min     : 0i32,
            map     : hash_map::HashMap::new(),
        }
    }

    //only compile thing in this file
    pub fn add_rule(&mut self, mut rule : Rule) -> Result<(),Error>{
        if rule.in_symbol == "_"{
            rule.in_symbol = String::new();
        }
        if rule.out_symbol == "_"{
            rule.out_symbol = String::new();
        } match self.map.insert(
             InData::new(rule.in_state, rule.in_symbol),
            OutData::new(rule.out_state, rule.out_symbol, rule.out_move),
        ){
            None => Ok(()),
            Some(_) => {Error::new(ErrorCode::DuplicateDelta)},
        }
    }

    pub fn set_init(&mut self, init: String){
        self.init = init.clone();
        self.curr = init;
    }

    pub fn set_tape(&mut self, ntape : Vec<String>){
        self.tape = ntape;
    }

    pub fn init(&mut self) {
        self.tape = vec!["\0".to_string()];
    }


    pub fn step(&mut self) -> Current{
        if self.end == self.curr{
            return Current::Ended;
        }
        let in_data = InData::set(&self.curr, &self.tape[self.pos]);

        let out_data = match self.map.get(&in_data){
            Some(s) => s,
            None => {return Current::Died;},
        };

        self.tape[self.pos] = out_data.out_symbol.clone();
        self.curr = out_data.out_state.clone();
        match out_data.out_move{
            Moves::Right => {
                self.pos += 1;
                if self.pos == self.tape.len(){
                    self.tape.push(String::new());
                }
            },
            Moves::Left  => {
                if self.pos == 0 {
                    self.min -= 1;
                    self.pos += 1;
                    self.tape.reverse();
                    self.tape.push(String::new());
                    self.tape.reverse();
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
