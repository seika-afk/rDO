use clap::{Command};
use dialoguer::Select;
use figlet_rs::FIGfont;
use std::io::stdin;
use std::io::{self, BufRead,BufReader,Write};
use std::fs::OpenOptions;
use std::fs::File;
use std::fs;

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
    .with_prompt("===============================")
    .items(v)
    .interact()
    .unwrap();
selection

}
fn main()-> std::io::Result<()> {

//#################VARIABLES
let choices = vec!["> Add ","> Show ","> Cut","> Clear"];

//######################FUNC CALLS
//setup();
    
let selection = take_input(choices);

//println!("{}",selection);

// ####################IF ADD -> ADD NEW TEXT 
//configuing for text file /db

let mut file= OpenOptions::new()
        .append(true)
        .create(true)
        .open("db.txt")?;



    let mut input= String::new();
match selection{
    0=>{
        loop{
      

print!("> ");
 io::stdout().flush().unwrap();
                stdin()
            .read_line(&mut input)
                .expect("Try again.");
            if input.trim()=="q" {
                    break;

                }

                file.write_all(input.as_bytes())?;
            
input.clear();

            }
        },
1=>{

let file= File::open("db.txt")?;
let reader=BufReader::new(file);

            for line in reader.lines(){
                let line=line?;
                print!("> ");
                println!("{}",line);


            }


    },
   2 => {   
let mut input = String::new();
    println!("Enter line number to remove:");
    stdin().read_line(&mut input)?;
    let line_num: usize = input.trim().parse().expect("Please enter a valid number");

    // Read all lines into a vector
    let reader = BufReader::new(File::open("db.txt")?);
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|l| l.ok())
        .enumerate() 
        .filter(|(idx, _)| *idx + 1 != line_num) 
        .map(|(_, line)| line)
        .collect();

    // Rewrite the file
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("db.txt")?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }}
    ,
3=>fs::remove_file("db.txt")?,
_=>println!("_")

    }


Ok(())
}


