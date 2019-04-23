use serde::{Serialize, Deserialize};

use regex::Regex;

use std::error::Error;

use std::str::FromStr;

fn normalise_str(s: &str) -> String {
    s.to_lowercase().replace("-", "").replace("_", "")
}

/// Trait with useful methods for working with websequencediagrams (wsd) enums (style, ...).
pub trait WSDEnum
    where
        Self: std::marker::Sized,
{
    /// is this value premium? Premium features requires correct API key
    fn premium_feature(&self) -> bool;

    /// value used in request to API
    fn wsd_value(&self) -> String;

    /// returns list of all values of type
    fn all() -> Vec<Self>;

    fn human_readable_value(&self) -> String {
        self.wsd_value()
    }

    fn from_str(s: &str) -> Option<Self> {
        for x in Self::all() {
            if normalise_str(&x.human_readable_value()) == normalise_str(s) {
                return Some(x);
            }
        }
        None
    }

    fn all_wsd_values() -> Vec<String> {
        Self::all().iter().map(WSDEnum::wsd_value).collect()
    }

    fn all_human_readable_values() -> Vec<String> {
        Self::all()
            .iter()
            .map(WSDEnum::human_readable_value)
            .collect()
    }

    fn help_str() -> String {
        Self::all()
            .iter()
            .map(|x| {
                if x.premium_feature() {
                    format!("{} (premium)", x.human_readable_value())
                } else {
                    x.human_readable_value()
                }
            })
            .collect::<Vec<String>>()
            .join(", ")
    }
}

/// represent output format. Note: some formats (Pdf, Svg) are premium features.
///
/// high-res png is png but with scale=200
///
/// Note: websequencediagram API may return result in other format then requested
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Format {
    Png,
    Pdf,
    Svg,
}

// By default PNG image is created
impl Default for Format {
    fn default() -> Format {
        Format::Png
    }
}

impl WSDEnum for Format {
    fn premium_feature(&self) -> bool {
        match self {
            Format::Png => false,
            Format::Pdf => true,
            Format::Svg => true,
        }
    }

    // Converts to value used in WSD API
    fn wsd_value(&self) -> String {
        match self {
            Format::Png => "png".to_owned(),
            Format::Pdf => "pdf".to_owned(),
            Format::Svg => "svg".to_owned(),
        }
    }

    fn all() -> Vec<Format> {
        use Format::*;
        vec![Png, Pdf, Svg]
    }
}

/// Represent style used to plot diagram. List of styles: default, earth, magazine,
/// modern-blue, mscgen, napkin, omegapple, patent, qsd, rose, roundgreen
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Style {
    Default,
    Earth,
    Magazine,
    ModernBlue,
    Mscgen,
    Napkin,
    Omegapple,
    Patent,
    Qsd,
    Rose,
    Roundgreen,
}

impl Default for Style {
    fn default() -> Style {
        Style::Default
    }
}

impl WSDEnum for Style {
    fn premium_feature(&self) -> bool {
        false
    }

    // Converts to value used in WSD API
    fn wsd_value(&self) -> String {
        match self {
            Style::Default => "default".to_owned(),
            Style::Earth => "earth".to_owned(),
            Style::Magazine => "magazine".to_owned(),
            Style::ModernBlue => "modern-blue".to_owned(),
            Style::Mscgen => "mscgen".to_owned(),
            Style::Napkin => "napkin".to_owned(),
            Style::Omegapple => "omegapple".to_owned(),
            Style::Patent => "patent".to_owned(),
            Style::Qsd => "qsd".to_owned(),
            Style::Rose => "rose".to_owned(),
            Style::Roundgreen => "roundgreen".to_owned(),
        }
    }

    fn all() -> Vec<Style> {
        use Style::*;
        vec![
            Default, Earth, Magazine, ModernBlue, Mscgen, Napkin, Omegapple, Patent, Qsd, Rose,
            Roundgreen,
        ]
    }
}

/// Represent paper size of result. Only useful with pdf output format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaperSize {
    None,
    Letter,
    A4,
    _11x17,
    A1,
    A2,
    A3,
    Legal,
}

impl Default for PaperSize {
    fn default() -> PaperSize {
        PaperSize::None
    }
}

impl WSDEnum for PaperSize {
    fn premium_feature(&self) -> bool {
        // It seems paper size is not premium feature by itself.
        // However it is only useful for pdf format which is premium feature.
        false
    }

    fn wsd_value(&self) -> String {
        match self {
            PaperSize::None => "none".to_owned(),
            PaperSize::Letter => "lettter".to_owned(),
            PaperSize::A4 => "a4".to_owned(),
            PaperSize::_11x17 => "11x17".to_owned(),
            PaperSize::A1 => "a1".to_owned(),
            PaperSize::A2 => "a2".to_owned(),
            PaperSize::A3 => "a3".to_owned(),
            PaperSize::Legal => "legal".to_owned(),
        }
    }

    fn all() -> Vec<PaperSize> {
        use PaperSize::*;
        vec![None, Letter, A4, _11x17, A1, A2, A3, Legal]
    }
}

/// Represent paper orientation of output. Useful only with pdf output format
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaperOrientation {
    Portrait,
    Landscape,
}

impl Default for PaperOrientation {
    fn default() -> PaperOrientation {
        PaperOrientation::Portrait
    }
}

impl WSDEnum for PaperOrientation {
    fn premium_feature(&self) -> bool {
        // It seems paper orientation is not premium feature by itself.
        // However it is only useful for pdf format which is premium feature.
        false
    }

    // in request Portrait is encoded as landscape=0
    // Landscape is encoded as landscape=1
    fn wsd_value(&self) -> String {
        match self {
            PaperOrientation::Portrait => "0".to_owned(),
            PaperOrientation::Landscape => "1".to_owned(),
        }
    }

    fn all() -> Vec<PaperOrientation> {
        vec![PaperOrientation::Portrait, PaperOrientation::Landscape]
    }

    fn human_readable_value(&self) -> String {
        match self {
            PaperOrientation::Portrait => "portrait".to_owned(),
            PaperOrientation::Landscape => "landscape".to_owned(),
        }
    }
}

/// represent parameters for plotting
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlotParameters {
    /// style to used. By default `default` style is used
    pub style: Style,

    /// output format. By default `png` is used
    pub format: Format,

    /// paper size of output. Only useful with pdf output format
    pub paper_size: Option<PaperSize>,

    /// paper orientation of output. Only useful with pdf output format
    pub paper_orientation: Option<PaperOrientation>,

    /// scale of output image. Useful with png output format. To obtain high-res png use scale=200
    pub scale: Option<u32>,

    /// API key. Some features requires premium account.
    pub api_key: Option<String>,
}

impl Default for PlotParameters {
    fn default() -> PlotParameters {
        PlotParameters {
            style: Style::default(),
            format: Format::default(),
            paper_size: None,
            paper_orientation: None,
            scale: None,
            api_key: None,
        }
    }
}

/// Represent an error during diagram creation
///
/// Example of raw errors from API:
///
/// "Line 1: Syntax error."
///
/// "Line 3: Deactivate: A was not activated."
///
/// Errors may be not fatal. So you may still obtain diagram and errors at the same time.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagramError {
    /// Parsed description
    pub description: String,

    /// Line number where the error occurred. Note: it may be incorrect due to bug in API if
    /// input starts from empty lines
    pub line_number: i32,

    /// description returned from API
    pub raw_description: String,
}

impl DiagramError {
    // "Line 1: Syntax error."
    pub fn from_wsd_error_str(error: &str) -> Result<DiagramError, Box<Error>> {
        // TODO(mkl): maybe use lazy_static ?
        let re = Regex::new(r"(?ix)
\s*Line\s+
(?P<line_number>\d+)  # the line number
\s* : \s*
(?P<description>.*) # the description
")?;

        let caps = if let Some(caps) = re.captures(error) {
            caps
        } else {
            return Err("Error parsing error response.".into())
        };
        let line_number = if let Some(line_number_match) = caps.name("line_number"){
            match  i32::from_str(line_number_match.as_str()) {
                Ok(x) => x,
                Err(err) => return Err(format!("Error parsing error response. Cannot convert line_number into int: {} . String: `{}`", err, line_number_match.as_str()).into()),
            }
        } else {
            return Err("Error parsing error response. Group `line_number` not found".into())
        };

        let description = if let Some(description_match) = caps.name("description") {
            description_match.as_str().to_owned()
        } else {
            return Err("Error parsing error response. Group `description` not found".into())
        };
        Ok(DiagramError {
            line_number,
            description,
            raw_description: error.to_owned()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::DiagramError;

    #[test]
    fn from_wsd_error_1_test() {
        match DiagramError::from_wsd_error_str("Line 1: Syntax error.") {
            Ok(rez) => {
                assert_eq!(
                    rez,
                    DiagramError {
                        line_number: 1,
                        description: "Syntax error.".to_owned(),
                        raw_description: "Line 1: Syntax error.".to_owned()
                    }
                );
            },
            Err(err) => {
                panic!("Result expected. Instead got an error: {}", err);
            }
        }
    }

    #[test]
    fn from_wsd_error_2_test() {
        match DiagramError::from_wsd_error_str("Line 3: Deactivate: A was not activated.") {
            Ok(rez) => {
                assert_eq!(
                    rez,
                    DiagramError {
                        line_number: 3,
                        description: "Deactivate: A was not activated.".to_owned(),
                        raw_description: "Line 3: Deactivate: A was not activated.".to_owned()
                    }
                );
            },
            Err(err) => {
                panic!("Result expected. Instead got an error: {}", err);
            }
        }
    }


}