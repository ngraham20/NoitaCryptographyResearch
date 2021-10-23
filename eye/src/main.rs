#![recursion_limit = "1024"]
#![allow(warnings)]

mod errors;
use errors::*;
mod cipher;
mod prelude {
    pub use crate::cipher::{Alberti, Substitution};
}
mod gene;
use gene::*;

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }
        
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let a = String::from("abcdefg");
    let mut g = gene::Gene { genomes: a.as_bytes().to_vec(), age: 0 };
    g.mutate()?;
    
    println!("{:?}", std::str::from_utf8(&g.genomes));
    Ok(())
}