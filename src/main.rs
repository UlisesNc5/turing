mod compiler;
mod engine;
#[allow(dead_code)]
mod errors;

use std::{io, result, thread, time::Duration};
use tui::{
    backend::Backend,
    backend::CrosstermBackend, Terminal,
    widgets,
};

use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, Event,
        KeyEvent, KeyCode, KeyModifiers
    },
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen,
        enable_raw_mode,
        disable_raw_mode,
    }
};

const VALIDO :[&str; 2]= ["Invalido", "Valido"];
const SIMULA :[&str; 2]= ["No simulando", "Simulando"];
static mut SIMULATE : bool = false;
static mut VALID    : bool = false;
static mut IN_STR : String = String::new();

const TEXT : &str = "
Instrucciones:
    para ingresar una cadena solo presione las teclas que necesite
    para hacer que el programa simule la maquina de turing presione ctrl + a
    para hacer que el programe termine presione esc 
    para limpiar la cadena presione enter


Simulado:
Ingrese Cadena:
Resultado:
";

fn render<T : Backend>(terminal: &mut Terminal<T>) -> Result<(), io::Error> {
    // render
    terminal.draw(|f| {
        let mut size = f.size();
        size.x += 1;
        size.y += 1;
        size.height -= 2;
        size.width  -= 2;
        let static_text = widgets::Paragraph::new(TEXT);
        let in_str : widgets::Paragraph;
        let result_str : widgets::Paragraph;
        let simulate_str : widgets::Paragraph;
        unsafe{
        result_str = widgets::Paragraph::new(VALIDO[VALID as usize]);
        in_str = widgets::Paragraph::new(IN_STR.clone());
        simulate_str = widgets::Paragraph::new(SIMULA[SIMULATE as usize]);
        }
        let mut in_size = size.clone();
        in_size.y +=  9;
        in_size.x += 16;
        in_size.width  -= 16;
        in_size.height -=  9;

        let mut valid_s = in_size.clone();
        valid_s.y += 1;
        valid_s.width -= 1;

        let mut simul_s = in_size.clone();
        simul_s.y -= 1;
        simul_s.width -= 1;

        f.render_widget(static_text, size);
        f.render_widget(simulate_str, simul_s);
        f.render_widget(in_str, in_size);
        f.render_widget(result_str, valid_s);
    })?;
    // end render

    Ok(())
}

fn fast(instr : &String) -> bool {
    for letter in instr.chars(){
        if letter != 'a' && letter != 'b'{
            return false;
        }
    }

    return instr.len().is_power_of_two();
}

fn slow(in_str: &String) -> bool{
    let mut machine = match compiler::compile_file("test.tr"){
        Ok(r) => r,
        Err(_) => {return false;},
    };
    machine.set_tape(in_str.clone() + "\0");
    loop {

        match machine.step(){
            engine::Current::Ended => {return true;},
            engine::Current::Died  => {return false;}
            _ => {}
        }
    }
}

fn main() -> Result<(), io::Error>{
    // setup terminal
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // end setup

    let mut result : Result<(), io::Error> = Ok(());
    loop{
        result = render(&mut terminal);

        let input = crossterm::event::read()?;
        match input{
            Event::Paste(x) => {
                unsafe{
                    IN_STR.push_str(x.as_str());
                }
            }
            Event::Key(key_event) => {
                let control = match key_event.modifiers{
                    KeyModifiers::CONTROL => true,
                    _ => false,
                };
                match key_event.code{
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        unsafe{
                            if control && c == 'a'{
                                SIMULATE = !SIMULATE;
                                continue;
                            }
                            IN_STR.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        unsafe {
                            IN_STR.pop();
                        }
                    }
                    KeyCode::Enter => {
                        unsafe{
                            IN_STR.clear();
                        }
                    }
                    _ => {}
                }
                unsafe{
                    VALID = if SIMULATE {
                        slow(&IN_STR)
                    }
                    else{
                        fast(&IN_STR)
                    };
                }
            },
            _ => {}
        }

    }

    // restore terminal
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    // end restore
    return result;
}
