extern crate clap;

use clap::{App, Arg};

fn main() {

    let matches = App::new("MyApp")
                        .arg(Arg::with_name("foo")
                                    .help("FOO")
                                    .short("f")
                                    .required(true)
                                    .takes_value(true)
                                    //.conflicts_with("bar")      
                                    .requires("bar")
                                    .conflicts_with("x")
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
    {
    let matches = matches.get_matches();
    }
                        //.verify()
                        //.get_matches();
                        //;


    println!("Test");

}
