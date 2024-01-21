use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(Reqwest::Error);
    }
}

fn main() -> Result<()>{
    let mut res = reqwest::blocking::get()?;
    let mut body = string::new();
    res.read_to_string(&mut body)?;

    println!("status :{}", res.status());
    println!("Header: \n{:#?}", res.Header());
    println!("body: \n{}", body);
    ok(())
}
