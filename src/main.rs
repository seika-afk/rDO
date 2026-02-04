use clap::{Command};
use dialoguer::Select;
use figlet_rs::FIGfont;



fn setup(){

    let _= Command::new("rDO")
    .version("0.1")
    .author("seika")
    .about("A cli based rust todo.");
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("r-DO");
    assert!(figure.is_some());
    println!("{}", figure.unwrap());
}

fn take_input(v:Vec<&'static str>)->usize{

let selection = Select::new()
    .with_prompt("What do you choose?")
    .items(v)
    .interact()
    .unwrap();
selection

}
fn main() {

//#################VARIABLES
let choices = vec!["> Add ","> Cut","> Clear"];

//######################FUNC CALLS
setup();
let selection = take_input(choices);

println!("{}",selection);


}


