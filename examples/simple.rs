extern crate wsdclient;
use wsdclient::{get_diagram};

use std::fs::File;
use std::io::Write;

fn main(){
    let spec = "A->B: text1";
    let rez = get_diagram(spec, &Default::default()).unwrap();
    File::create("simple.png").unwrap()
        .write_all(&rez.diagram).unwrap();
}