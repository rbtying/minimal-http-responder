use http::status::StatusCode;
use std::env;
use std::io::{Error, Write};
use std::net::{TcpListener, TcpStream};

struct Configuration {
    status_code: StatusCode,
    text: Option<String>,
}

fn main() {
    let status_code = env::var("STATUS_CODE")
        .map_err(|_| ())
        .and_then(|s| StatusCode::from_bytes(s.as_bytes()).map_err(|_| ()))
        .unwrap_or(StatusCode::OK);
    let configuration = Configuration {
        status_code,
        text: env::var("TEXT").ok(),
    };

    let listener = TcpListener::bind("0.0.0.0:2020").unwrap();

    for stream in listener.incoming() {
        if let Ok(s) = stream {
            let _ = handle_request(&configuration, s);
        }
    }
}

fn handle_request(configuration: &Configuration, mut stream: TcpStream) -> Result<(), Error> {
    stream.write(
        format!(
            "HTTP/1.1 {} {}\r\n\r\n",
            configuration.status_code.as_u16(),
            configuration
                .status_code
                .canonical_reason()
                .unwrap_or("Unknown")
        )
        .as_bytes(),
    )?;
    match (
        configuration.text.as_ref(),
        configuration.status_code.canonical_reason(),
    ) {
        (Some(t), _) => stream.write(t.as_bytes())?,
        (None, Some(r)) => {
            stream.write(format!("{} {}", configuration.status_code.as_u16(), r).as_bytes())?
        }
        (None, None) => {
            stream.write(format!("{}", configuration.status_code.as_u16()).as_bytes())?
        }
    };
    stream.flush()?;
    Ok(())
}
