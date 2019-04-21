use serde::{Serialize, Deserialize};
use crate::options::PlotParameters;

use std::error::Error;
use crate::options::WSDEnum;

// Represent response from websequence diagram website
#[derive(Debug, Clone, Serialize, Deserialize)]
struct WebSequenceDiagramResponse {
    img: String,
    errors: Vec<String>, // TODO(mkl): add aditional fields
}

pub fn get_diagram(spec: &str, parameters: &PlotParameters) -> Result<Vec<u8>, Box<Error>> {
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

    let resp = reqwest::Client::new()
        .post("http://www.websequencediagrams.com/index.php")
        .form(&params)
        .send();

    let wr: WebSequenceDiagramResponse = match resp {
        Ok(mut r) => {
            let mut v = vec![];
            // Save the response, so we can check it if something going wrong
            std::io::copy(&mut r, &mut v).unwrap();

            if !r.status().is_success() {
                return Err(format!(
                    "Error response from wsd code={:?} response={}",
                    r.status(),
                    String::from_utf8_lossy(&v)
                )
                    .into());
            }

            println!("response: {}", String::from_utf8_lossy(&v));
            match serde_json::from_reader(&v[..]) {
                Ok(r) => r,
                Err(err) => {
                    println!(
                        "Error deserializing websequencegiagram response: {:?}",
                        &err
                    );
                    println!("Response: {}", String::from_utf8_lossy(&v));
                    return Err(format!(
                        "Error deserializing websequencegiagram response: {:?}",
                        err
                    )
                        .into());
                }
            }
        }
        Err(err) => {
            return Err(format!("ERROR: {}", err).into());
        }
    };

    let mut resp2 = reqwest::Client::new()
        .get(("http://www.websequencediagrams.com/index.php".to_owned() + &wr.img).as_str())
        .send()
        .unwrap();

    if !resp2.status().is_success() {
        return Err("Error getting image from size".to_string().into());
    }

    let mut data = vec![];
    // copy the response body directly to stdout
    std::io::copy(&mut resp2, &mut data).unwrap();
    Ok(data)
}