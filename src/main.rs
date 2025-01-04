fn main() {
    println!("Hello, world!");
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
}
