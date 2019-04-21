extern crate wsdclient;

use wsdclient::config::Config;
use wsdclient::client::get_diagram;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // TODO(mkl): do not use unwrap
    let config = Config::from_command_line();

    let mut diagram: Vec<u8> = vec![];
    if let Some(input_file) = config.input_file {
        File::open(input_file).unwrap().read_to_end(&mut diagram).unwrap();
    } else {
        unimplemented!("reading from STDIN is not supported");
    }

    let diagram_img = get_diagram(&String::from_utf8_lossy(&diagram[..]), &config.plot_parameters).unwrap();

    let mut f = File::create(config.output_file).unwrap();
    // copy the response body directly to stdout
    f.write_all(&diagram_img[..]).unwrap();
}
