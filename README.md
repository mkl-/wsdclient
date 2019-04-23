 Crate for working with [https://www.websequencediagrams.com/](https://www.websequencediagrams.com/)
 public [RESTful API](https://www.websequencediagrams.com/embedding.html)
 This service allows to create sequence diagrams from simple text.
 Note: this library is a simple wrapper around the API.
 Some features of this service require premium subscription (like export to pdf format)
 Features supported by this library
 * statically typed library. Different options are represented as enums
 * multiple output formats: png, pdf (premium), svg (premium)
 * detection of actual output format. E.g. trying to get pdf with wrong API key leads to png output
 * allows specification of scale, paper size, paper orientation, style
 * parse returned errors

 This crate contains command line tool for accessing websequencediagram API

 `$ wsdclient my_diag.wsd -o my.png`

 Example:
 ```
 use wsdclient::{get_diagram};

use std::fs::File;
use std::io::Write;

fn main(){
    let spec = "A->B: text1";
    let rez = get_diagram(spec, &Default::default()).unwrap();
    File::create("simple.png").unwrap()
        .write_all(&rez.diagram).unwrap();
}
 ``` 
 