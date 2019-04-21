extern crate wsdclient;

use wsdclient::config::Config;
use wsdclient::client::get_diagram;
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // TODO(mkl): add option to treat all errors/warnings as fatal
    // TODO(mkl): add option to print requests and responses
    // TODO(mkl): do not use unwrap
    let config = Config::from_command_line();

    let mut diagram: Vec<u8> = vec![];
    if let Some(ref input_file) = config.input_file {
        File::open(input_file).unwrap().read_to_end(&mut diagram).unwrap();
    } else {
        // TODO(mkl): add support for STDIN
        unimplemented!("reading from STDIN is not supported");
    }

    let diagram_str = String::from_utf8_lossy(&diagram[..]);

    let (diagram_img, errors) = get_diagram(&diagram_str, &config.plot_parameters).unwrap();
    if !errors.is_empty() {
        let lines = diagram_str.split("\n").collect::<Vec<&str>>();
        // There is a bug in websequencediagrams
        // if file starts with empty strings
        // error indexes are less by 1
        let delta =  if has_empty_lines_at_begining(&lines){
            1
        } else {
            0
        };
        for error in errors {
            // TODO(mkl): should I use stderr or stdout ?
            // TODO(mkl): add check if line_numbers are sane
            let inp_file_name = config.input_file.clone().unwrap_or("<STDIN>".to_owned());
            let line_number = error.line_number + delta;
            println!("{}:{} : {}", inp_file_name, line_number, error.description);
            println!("{}\n", lines[(line_number-1) as usize])
        }
    }

    let mut f = File::create(config.output_file).unwrap();
    // copy the response body directly to stdout
    f.write_all(&diagram_img[..]).unwrap();
}

fn is_empty(s: &str) -> bool {
    s
        .replace("\n", "")
        .replace(" ", "")
        .replace("\t", "")
        .is_empty()
}

fn has_empty_lines_at_begining(lines: &Vec<&str>) -> bool {
    !lines.is_empty() && is_empty(lines[0])
}