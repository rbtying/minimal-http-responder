use http::status::StatusCode;
use std::env;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;

struct Configuration {
    status_code: StatusCode,
    text: Option<String>,
}

fn main() {
    ctrlc::set_handler(|| {
        println!("Received SIGTERM");
        std::process::exit(0);
    })
    .unwrap();

    let status_code = env::var("STATUS_CODE")
        .map_err(|_| ())
        .and_then(|s| StatusCode::from_bytes(s.as_bytes()).map_err(|_| ()))
        .unwrap_or(StatusCode::OK);
    let configuration = Arc::new(Configuration {
        status_code,
        text: env::var("TEXT").ok(),
    });

    let listener = TcpListener::bind("0.0.0.0:2020").unwrap();

    for stream in listener.incoming().flatten() {
        let c = Arc::clone(&configuration);
        thread::spawn(move || {
            let _ = handle_request(&c, stream);
        });
    }
}

fn handle_request(configuration: &Configuration, mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0u8; 1024];
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 || buf.windows(4).any(|w| w == b"\r\n\r\n") {
            break;
        }
    }
    stream.write_all(
        format!(
            "HTTP/1.1 {} {}\r\n",
            configuration.status_code.as_u16(),
            configuration
                .status_code
                .canonical_reason()
                .unwrap_or("Unknown")
        )
        .as_bytes(),
    )?;
    let contents = match (
        configuration.text.as_ref(),
        configuration.status_code.canonical_reason(),
    ) {
        (Some(t), _) => t.to_string(),
        (None, Some(r)) => format!("{} {}", configuration.status_code.as_u16(), r),

        (None, None) => format!("{}", configuration.status_code.as_u16()),
    };
    stream
        .write_all(format!("Content-Length: {}\r\n\r\n", contents.as_bytes().len()).as_bytes())?;
    stream.write_all(contents.as_bytes())?;
    stream.flush()?;
    Ok(())
}
