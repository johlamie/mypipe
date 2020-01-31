extern crate clap;
use clap::{App, Arg, SubCommand};
use std::process::Command;

fn main() {
    // println!("Hello, world!");
    let matches = App::new("mypipe")
                          .version("1.0")
                          .author("Doubalo John Eddy LAMIEN <johnlamien@gmail.com>")
                          .about("pipe")
                          .arg(Arg::with_name("input")
                               .short("in")
                               .long("in")
                               .value_name("intput")
                               .help("Sets the input to use")
                               .takes_value(true))
                            .arg(Arg::with_name("output")
                               .short("out")
                               .long("out")
                               .value_name("output")
                               .help("Sets the output to use")
                               .takes_value(true))
                          .get_matches();

    let args1 = matches.value_of("input").unwrap( );
    
    let args2 = matches.value_of("output").unwrap( );


    let inputExecution = Command::new(args1.to_string()).output().expect("Runtime error");


    let inputArgs = String::from_utf8_lossy(&inputExecution.stdout).to_string();

    let outputExec = Command::new(args2.to_string()).arg(inputArgs)
                        .output().expect("Runtime error");


    let end = String::from_utf8_lossy(&outputExec.stdout).to_string();

    println!("{}", end );
    
}
