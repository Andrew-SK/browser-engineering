use std::{
    collections::HashMap,
    env::args,
    io::{Read, Write},
    net::TcpStream,
    process::exit,
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut args = args();
    args.next(); // skip executable name
    let url = URL::parse(&args.next().expect("missing URL arg"));
    let response_content = url.request()?;

    show(response_content);

    Ok(())
}

#[derive(Debug, PartialEq)]
struct URL {
    scheme: String,
    host: String,
    path: String,
}

impl URL {
    fn parse(unparsed: &str) -> URL {
        let (scheme, rest) = match unparsed.split_once("://") {
            Some((scheme, rest)) => (scheme.to_owned(), rest),
            None => (String::from("http"), unparsed),
        };
        assert!(scheme == "http");

        let (host, path) = match rest.split_once('/') {
            Some((host, path)) => (host.to_owned(), String::from("/") + path),
            None => (rest.to_owned(), String::from("/")),
        };

        URL { scheme, host, path }
    }

    fn request(&self) -> Result<String> {
        let mut sock = TcpStream::connect(format!("{}:80", self.host))?;

        let mut req = String::new();
        req.push_str(&format!("GET {} HTTP/1.0\r\n", self.path));
        req.push_str(&format!("Host: {}\r\n", self.host));
        req.push_str("\r\n");

        sock.write_all(req.as_bytes())?;

        let mut resp = String::new();
        sock.read_to_string(&mut resp)?;
        let mut line_reader = resp.lines();

        // Parse the statusline
        let mut statusline = line_reader
            .next()
            .context("response missing statusline")?
            .splitn(3, ' ');
        let _version = statusline.next().context("statusline missing version")?;
        let _status = statusline.next().context("statusline missing status")?;
        let _explanation = statusline.next().unwrap_or("");

        // Parse headers
        let mut headers = HashMap::new();
        for headerline in &mut line_reader {
            if headerline == "" {
                break;
            }

            match headerline.split_once(':') {
                Some((key, value)) => headers.insert(key.to_lowercase(), value.trim_start()),
                None => continue,
            };
        }

        // confirm we're not getting as yet unsupported response content
        assert!(
            headers.get("transfer-encoding").is_none(),
            "transfer-encoding not supported"
        );
        assert!(
            headers.get("content-encoding").is_none(),
            "content-encoding not supported"
        );
        let content: String = line_reader.collect();
        Ok(content)
    }
}

fn show(content: String) {
    let mut in_tag = false;
    for c in content.chars() {
        match c {
            '<' => {
                in_tag = true;
            }
            '>' => {
                in_tag = false;
            }
            c => {
                if !in_tag {
                    print!("{}", c);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let url = URL::parse("http://example.org/path.html");

        assert_eq!(
            url,
            URL {
                scheme: String::from("http"),
                host: String::from("example.org"),
                path: String::from("/path.html"),
            }
        );
    }

    #[test]
    fn test_parse_without_scheme() {
        let url = URL::parse("example.org/path.html");

        assert_eq!(
            url,
            URL {
                scheme: String::from("http"),
                host: String::from("example.org"),
                path: String::from("/path.html"),
            }
        );
    }

    #[test]
    fn test_parse_without_path() {
        let url = URL::parse("http://example.org");

        assert_eq!(
            url,
            URL {
                scheme: String::from("http"),
                host: String::from("example.org"),
                path: String::from("/"),
            }
        );
    }

    #[test]
    fn test_request() {
        let url = URL::parse("http://example.org");
        url.request();
    }
}
