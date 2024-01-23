# GetRequest

A simple Rust project to make HTTP GET requests using the `reqwest` crate, handle errors with `error_chain`, and display the response status, headers, and body.

## 🚀 Quick Start

1. Clone the repository:

    ```bash
    git clone https://github.com/itsmohitnarayan/Get-Request-Rust
    ```

2. Navigate to the project directory:

    ```bash
    cd Get-Request-Rust   
    ```

3. Build and run the project:

    ```bash
    cargo run
    ```

## 🛠️ Dependencies

- [reqwest](https://docs.rs/reqwest): HTTP client for Rust.
- [error_chain](https://docs.rs/error-chain): A library for flexible error handling.

## 📋 Usage

This project demonstrates making a simple HTTP GET request to "http://httpbin.org/get" and printing the response status, headers, and body.

It demonstrates basic error handling using the error_chain crate.

```rust
use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers: \n{:#?}", res.headers());
    println!("Body: \n{}", body);

    Ok(())
}
```

## 🤖 Error Handling

This project utilizes the `error_chain` crate for simple and flexible error handling. The errors that can occur during the execution of the program are `std::io::Error` and `reqwest::Error`.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or create pull requests.

## 📬 Contact

For any questions or suggestions, please feel free to [contact us](mailto:your.email@example.com).

---

Happy coding! 🦀✨

------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
