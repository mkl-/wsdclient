extern crate wsdclient;

use wsdclient::config::Config;
use crate::wsdclient::types::WSDEnum;
use wsdclient::client::get_diagram;
use std::fs::File;
use std::io::{Read, Write, stdin};
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    // TODO(mkl): add option to print requests and responses
    let config = Config::from_command_line()
        .map_err(|err| format!("error parsing command line options: {:?}", err))?;

    let mut diagram: Vec<u8> = vec![];
    if let Some(ref input_file) = config.input_file {
        File::open(input_file)
            .map_err(|err| format!("error opening input file {} : {:?}", input_file, err))?
            .read_to_end(&mut diagram)
            .map_err(|err| format!("error reading input file {} : {:?}", input_file, err))?;
    } else {
        stdin()
            .read_to_end(& mut diagram)
            .map_err(|err| format!("error reading from STDIN: {:?}", err))?;
    }

    let diagram_str = String::from_utf8_lossy(&diagram[..]);
    let result = get_diagram(&diagram_str, &config.plot_parameters)
        .map_err(|err| format!("error getting diagram: {:?}", err))?;

    if result.actual_format != config.plot_parameters.format {
        let error_msg = format!("WARNING: Actual format `{}` is different from requested format `{}`\nMaybe you do not provide correct api_key for premium features (like pdf or svg formats)", result.actual_format.wsd_value(), config.plot_parameters.format.wsd_value());
        if config.is_errors_fatal {
            return Err(error_msg.into())
        } else {
            eprintln!("{}", error_msg);
        }
    }

    if !result.errors.is_empty() {
        let lines = diagram_str.split("\n").collect::<Vec<&str>>();
        // There is a bug in websequencediagrams
        // if file starts with empty strings
        // error indexes are less by 1
        let delta =  if has_empty_lines_at_begining(&lines){
            1
        } else {
            0
        };
        for error in &result.errors {
            // TODO(mkl): should I use stderr or stdout ?
            // TODO(mkl): add check if line_numbers are sane
            let inp_file_name = config.input_file.clone().unwrap_or("<STDIN>".to_owned());
            let line_number = error.line_number + delta;
            eprintln!("{}:{} : {}", inp_file_name, line_number, error.description);
            eprintln!("{}\n", lines[(line_number-1) as usize])
        }
        if config.is_errors_fatal {
            return Err(format!("Number of errors in diagram: {}. Exiting.", result.errors.len()).into())
        }
    }

    let mut f = File::create(&config.output_file)
        .map_err(|err| format!("cannot open output file: {} : {:?}", &config.output_file, err))?;
    f.write_all(&result.diagram[..])
        .map_err(|err| format!("cannot write to output file : {} : {:?}", &config.output_file, err))?;
    Ok(())
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