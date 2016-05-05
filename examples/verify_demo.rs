extern crate clap;

use clap::{App, Arg};

fn main() {
    
    //Demonstration of use case of App::verify() function
    //If required arg 'foo' both requires and conflicts with 'bar',
    // verify() (which returns an Option) contains an error because 
    // the program will be unusable.
    //Something similar can be done for testing specific arguments by
    // making them required and then calling verify() to check compatibility.
    //This probably shouldn't be used in production code, as it only needs 
    // to be run once after the args are written.
    //This feature will also error-out if there is a .requires or 
    // .conflicts parameter that references an argument that is absent. 
    //This is another handy runtime check that can be useful in the 
    // debugging stage but probably doesn't belong in the release.
    

    let matches = App::new("MyApp")
                        .arg(Arg::with_name("foo")
                                    .help("FOO")
                                    .short("f")
                                    .required(true)
                                    .takes_value(true)
                                    //.conflicts_with("bar")      
                                    .requires("bar")
                                    //.conflicts_with("x")
                            )
                        .arg(Arg::with_name("bar")
                                    .short("b")
                                    .takes_value(true)
                                    .help("BAR")
                                    .requires("foo")
                                    .required(true)
                            );
    {
        match matches.verify(){
            Err(e) => panic!("Error: {}", e),
            Ok(_)  => ( )
        }
    }
    let matches = matches.get_matches();

}
