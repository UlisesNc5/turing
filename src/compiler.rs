use super::engine::{Program, Rule, Moves};
use super::errors::*;
use std::fs;
use std::io::Read;

pub fn compile_file(path: &str) -> Result<Program, Error>{
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => { return Error::new(ErrorCode::IO(e)); },
    };
    let size = file.metadata().unwrap().len() as usize;
    let mut whole_file = String::new();
    for _ in 0..size{
        whole_file.push('\0');
    }
    unsafe{
        match file.read(whole_file.as_bytes_mut()) {
            Ok(_) => {},
            Err(err) => {
                return Error::new(ErrorCode::IO(err));
            },
        }
    }

    while whole_file.contains("\n\n"){
        whole_file = whole_file.replace("\n\n", "\n");
    }
    whole_file = whole_file.replace(" ", "");
    
    let mut lines = Vec::<String>::new();
    lines.push(String::new());

    for letter in whole_file.chars(){
        if letter != '\n'{
            lines.last_mut().unwrap().push(letter);
        }
        else{
            lines.push(String::new());
        }
    }

    if lines.last().unwrap().len() < 2{
        lines.pop();
    }

    if (lines.len() - 3) % 2 != 0 || lines.len() == 0 {
        return Error::newe(ErrorCode::PreproError, "file with not enough lines");
    }
    else{}

    let mut prog = Program::new();
    if lines[0].starts_with("name:") == false{
        return Error::newe(ErrorCode::ParseError, "name not found");
    }
    prog.name = lines[0].replace("name:", "");

    if lines[1].starts_with("init:") == false{
        return Error::newe(ErrorCode::ParseError, "init not found");
    }
    prog.set_init(lines[1].replace("init:", ""));

    if lines[2].starts_with("accept:") == false{
        return Error::newe(ErrorCode::ParseError, "accept not found");
    }
    prog.end  = lines[2].replace("accept:", "");

    lines.remove(0);
    lines.remove(0);
    lines.remove(0);


    while lines.len() != 0{
        let delta_func = lines.remove(0);
        let result     = lines.remove(0);
        let mut rule = Rule::new();

        let mut split : Vec<&str> = delta_func.split(',').collect();
        if split.len() != 2{
            return Error::newe(ErrorCode::ParseError, "delta len is not 2")
        }
        rule.in_state  = split[0].to_string();
        rule.in_symbol = split[1].to_string();

        split = result.split(',').collect();
        if split.len() != 3{
            return Error::newe(ErrorCode::ParseError, "delta len is not 3")
        }
        rule.out_state  = split[0].to_string();
        rule.out_symbol = split[1].to_string();
        rule.out_move = match split[2].chars().next().unwrap(){
            '<' => {Moves::Left},
            '>' => {Moves::Right},
            '-' => {Moves::Stay},
            _ => {return Error::newe(ErrorCode::ParseError, "invalid move")},
        };
        match prog.add_rule(rule){
            Err(err) => {return Err(err);},
            Ok(())  => {},
        }
    }
    Ok(prog)
}
