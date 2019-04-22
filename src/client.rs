use regex::Regex;
use serde::{Serialize, Deserialize};
use crate::types::{PlotParameters, DiagramError, Format};

use std::error::Error;
use crate::types::WSDEnum;

// Represent response from websequence diagram website
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WebSequenceDiagramResponse {
    img: String,
    errors: Vec<String>,

    // TODO(mkl): add aditional fields
}

pub struct WSDResult {
    // Content of the diagram
    pub diagram: Vec<u8>,

    // Errors are not fatal. Even if there are errors
    // rest lines may be plotted.
    pub errors: Vec<DiagramError>,

    // Actual format may be different from requested. For example when pdf is requested
    // but no api key are provided.
    // Format is determined from returned url
    // "?png=mscKTO107" for png
    // "?pdf=mscKTO107" for pdf
    // "?svg=mscKTO107" for svg
    pub actual_format: Format
}

// it can plot diagrams even if there are errors
pub fn get_diagram(spec: &str, parameters: &PlotParameters) -> Result<WSDResult, Box<Error>> {
    // TODO(mkl): correct handling of incorrect API key
    // if send request for pdf but key is incorrect png in returned
    let mut params = vec![
        ("message".to_owned(), spec.to_owned()),
        ("style".to_owned(), parameters.style.wsd_value()),
        ("format".to_owned(), parameters.format.wsd_value()),
        ("apiVersion".to_owned(), "1".to_owned()),
    ];
    if let Some(ref api_key) = parameters.api_key {
        params.push(("apikey".to_owned(), api_key.clone()));
    }
    if let Some(ref paper_size) = parameters.paper_size {
        params.push(("paper".to_owned(), paper_size.wsd_value()));
    }
    if let Some(ref paper_orientation) = parameters.paper_orientation {
        params.push(("landscape".to_owned(), paper_orientation.wsd_value()));
    }
    if let Some(ref scale) = parameters.scale {
        params.push(("scale".to_owned(), format!("{}", scale)));
    }

    // URL for first request
    let first_request_url = "http://www.websequencediagrams.com/index.php";
    let first_response = reqwest::Client::new()
        .post(first_request_url)
        .form(&params)
        .send();

    let first_response: WebSequenceDiagramResponse = match first_response {
        Ok(mut r) => {
            let mut v = vec![];
            // Save the response, so we can check it if something going wrong
            std::io::copy(&mut r, &mut v)
                .map_err(|err| format!("error reading response from server {} : {:?}", first_request_url, err))?;

            if !r.status().is_success() {
                return Err(format!(
                    "Error response from server: {} HTTP code={:?} response={}",
                    first_request_url,
                    r.status(),
                    String::from_utf8_lossy(&v)
                ).into())
            }

            serde_json::from_reader(&v[..])
                .map_err(|err|
                    format!(
                        "Cannot deserialize websequencegiagram response: {:?} Response: {}",
                        err,
                        String::from_utf8_lossy(&v)
                    )
                )
        }
        Err(err) => {
            Err(format!("error sending request to {} : {}", first_request_url, err))
        }
    }?;

    let second_request_url = format!("http://www.websequencediagrams.com/index.php{}", first_response.img);
    // Second request contains actual diagram
    let mut second_response = reqwest::Client::new()
        .get(&second_request_url)
        .send()
        .map_err(|err| format!("Error sending request for diagram to {} : {:?}", second_request_url, err))?;

    if !second_response.status().is_success() {
        return Err(format!("Request for diagram was unsuccesfull url: {} code: {:?}", second_request_url, second_response.status()).into());
    }

    let mut data = vec![];
    std::io::copy(&mut second_response, &mut data)
        .map_err(|err|
                     format!("Error reading diagram from {} : {:?}", second_request_url, err)
        )?;

    let errors_parsed = first_response.errors
        .iter()
        .map(|error| DiagramError::from_wsd_error_str(error));
    let mut errors = vec![];
    for error in errors_parsed {
        match error {
            Ok(error) => errors.push(error),
            Err(err) => return Err(format!("cannot parse wsd error message  {:?}",err).into())
        }
    }

    Ok(WSDResult {
        diagram: data,
        errors,
        actual_format: Default::default(),
    })
}

fn determine_actual_format(url: &str) -> Result<Format, Box<Error>> {
    let re = Regex::new(r"(?ix)
\?
(?P<format>\w+)  # format
=
.*
")?;

    let caps = if let Some(caps) = re.captures(url) {
        caps
    } else {
        return Err("Error parsing diagram url.".into())
    };
    let format_str = if let Some(format_match) = caps.name("format"){
        format_match.as_str()
    } else {
        return Err("Error parsing diagram url. Group `format` not found".into())
    };
    match Format::from_str(format_str) {
        Some(x) => Ok(x),
        None => {
            Err(format!("unknown format in diagram url. Known formats are: {}. Got: {}", Format::help_str(), format_str).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{DiagramError, Format};
    use crate::client::determine_actual_format;

    #[test]
    fn determine_actual_format_test() {
        // "?png=mscKTO107" for png
        // "?pdf=mscKTO107" for pdf
        // "?svg=mscKTO107" for svg
        assert_eq!(determine_actual_format("?png=mscKTO107").unwrap(), Format::Png);
        assert_eq!(determine_actual_format("?pdf=mscKTO107").unwrap(), Format::Pdf);
        assert_eq!(determine_actual_format("?svg=mscKTO107").unwrap(), Format::Svg);
        assert!(determine_actual_format("?xxx=mscKTO107").is_err());
    }
}