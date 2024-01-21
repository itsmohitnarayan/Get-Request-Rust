error_chain

fn main() -> Result<()>{
    let mut res = reqwest::blocking::get()?;
    let mut body = string::new();
    res.read_to_string(&mut body)?;

    println!("status :{}", res.status());
    println!("Header: \n{:#?}", res.Header());
    println!("body: \n{}", body);
    ok(())
}
