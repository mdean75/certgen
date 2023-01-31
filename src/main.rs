use certgen::{cli, info, cert};

use std::process::exit;
use crate::cli::Commands;


fn main() {
    let args = cli::parse();


    if args.build {
        match args.output_format {
            Some(a) => {info::build_details(a.to_string())},
            None => {info::build_details("".to_string())}
        }
        exit(0);
    }

    match args.command {
        Some(Commands::Gen(g)) => {
            let cert = cert::generate_certs(&g.root_cn, &g.signing_cn, &g.expired);
            match cert {
                Ok(_) => {},
                Err(e) => println!("unable to create certificates: {}", e)
            }
        }
        Some(Commands::Test(t)) => {println!("{:?}", print_type_of(t))}
        None => {}
    }
}
fn print_type_of<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}
