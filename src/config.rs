use serde::{Serialize, Deserialize};

use clap::{App, Arg, AppSettings, SubCommand};

use crate::types::{WSDEnum, Format, Style, PaperSize, PaperOrientation, PlotParameters};

use std::error::Error;

fn split_list(s: Option<&str>) -> Vec<String> {
    match s {
        Some(s) => s.split(',').map(ToOwned::to_owned).collect(),
        None => vec![],
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub input_file: Option<String>,
    pub output_file: String,
    pub plot_parameters: PlotParameters,
    pub is_errors_fatal: bool
}


impl Config {
    // TODO(mkl): add function for exporting app and args
    // TODO(mkl): add function for parsing command line
    // TODO(mkl): add verbose option. Like write request and response to website
    pub fn from_command_line() -> Result<Config, Box<Error>> {
        let matches = App::new("wsdclient")
            .version(option_env!("CARGO_PKG_VERSION").unwrap_or("<version unknown>"))
            .author("Mykola Sakhno <mykola.sakhno@bitfury.com>")
            .about("wsdclient is a tool for creating diagrams from their textual representation using websequencediagrams public API. Note: errors are not fatal by default.")
            .arg(
                Arg::with_name("input-file")
                    .help("set the input file to use. If not specified STDIN is read.")
                    .index(1)
            )
            .arg(
                Arg::with_name("output-file")
                    .help("Output file for diagram. By default out.<format> is used. E.g. out.png")
                    .long("output")
                    .short("o")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("api-key")
                    .help("websequencediagram api key. For security reason it is better to use environmental variable WEBSEQUENCEDIAGRAM_API_KEY. Command line option has higher precedence over environment variable. Api key can be obtained by going to http://www.websequencediagrams.com/users/getapikey while logged in.")
                    .long("api-key")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("format")
                    .help(&format!("Format of the output file. Some formats are premium. Possible values: {}. Default value is png", Format::help_str()))
                    .long("format")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("style")
                    .help(&format!("Style to use. Possible styles are {}. Default value: {}", Style::help_str(), Style::Default.human_readable_value()))
                    .long("style")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("paper-size")
                    .help(&format!("Paper size to use. Useful only for pdf output format. Possible values: {}. By default it is not included into request.", PaperSize::help_str()))
                    .long("paper-size")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("paper-orientation")
                    .help(&format!("Paper orientation to use. Useful only for pdf output format. Possible values: {}. By default it is not included into request.", PaperOrientation::help_str()))
                    .long("paper-orientation")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("scale")
                    .help("Scale. Default value is 100. High res is 200. It seems it only useful for png format. By default it is not included into request.")
                    .long("scale")
                    .takes_value(true)
            )
            .arg(
                Arg::with_name("errors-fatal")
                    .help("Treat all errors as fatal. By default some errors: like incorrect lines in diagram are ignored")
                    .long("errors-fatal")
            )
            .get_matches();


            let mut api_key: Option<String> = None;
            if let Some(api_key_arg) = matches.value_of("api-key") {
                api_key = Some(api_key_arg.to_owned())
            } else if let Ok(api_key_env) = std::env::var("WEBSEQUENCEDIAGRAM_API_KEY") {
                api_key = Some(api_key_env);
            }

            let mut format = Format::Png;
            if let Some(format_arg_str) = matches.value_of("format") {
                if let Some(format_arg) = Format::from_str(format_arg_str) {
                    format = format_arg;
                } else {
                    let error_msg = format!(
                        "incorrect format value. Possible values are: {}. Got: {}",
                        Format::help_str(),
                        format_arg_str
                    );
                    return Err(error_msg.into());
                }
            }

            let mut style = Style::Default;
            if let Some(style_arg_str) = matches.value_of("style") {
                if let Some(style_arg) = Style::from_str(style_arg_str) {
                    style = style_arg;
                } else {
                    let error_msg = format!(
                        "ERROR: incorrect style value. Possible values are: {}. Got: {}",
                        Style::help_str(),
                        style_arg_str
                    );
                    return Err(error_msg.into());
                }
            }

            let mut paper_size: Option<PaperSize> = None;
            if let Some(paper_size_arg_str) = matches.value_of("paper-size") {
                if let Some(paper_size_arg) = PaperSize::from_str(paper_size_arg_str) {
                    paper_size = Some(paper_size_arg)
                } else {
                    let error_msg = format!(
                        "ERROR: incorrect paper-size value. Possible values are: {}. Got: {}",
                        PaperSize::help_str(),
                        paper_size_arg_str
                    );
                    return Err(error_msg.into());
                }
            }

            let mut paper_orientation: Option<PaperOrientation> = None;
            if let Some(paper_orientation_arg_str) = matches.value_of("paper-orientation") {
                if let Some(paper_orientation_arg) =
                PaperOrientation::from_str(paper_orientation_arg_str)
                {
                    paper_orientation = Some(paper_orientation_arg)
                } else {
                    let error_msg = format!("ERROR: incorrect paper-orientation value. Possible values are: {}. Got: {}", PaperOrientation::help_str(), paper_orientation_arg_str);
                    return Err(error_msg.into());
                }
            }

            let mut scale: Option<u32> = None;
            if let Some(scale_arg_str) = matches.value_of("scale") {
                use std::str::FromStr;
                if let Ok(scale_arg) = u32::from_str(scale_arg_str) {
                    scale = Some(scale_arg)
                } else {
                    let error_msg = format!(
                        "ERROR: incorrect scale value. It should be positive integer. Got: {}",
                        scale_arg_str
                    );
                    return Err(error_msg.into());
                }
            }

            let output_file: String =
                if let Some(output_file_arg) = matches.value_of("output-file") {
                    output_file_arg.to_owned()
                } else {
                    format!("out.{}", format.wsd_value())
                };

            let input_file: Option<String> =
                if let Some(input_file_arg) = matches.value_of("input-file") {
                    Some(input_file_arg.to_owned())
                } else {
                    None
                };

            let is_errors_fatal = matches.occurrences_of("errors-fatal") > 0;

            let plot_parameters = PlotParameters {
                style,
                format,
                paper_size,
                paper_orientation,
                scale,
                api_key,
            };
            Ok(Config {
                input_file,
                output_file,
                plot_parameters,
                is_errors_fatal
            })
        }
    }
