#[tokio::main]
async fn main() {
    use openssl::ssl::{SslConnector, SslMethod};
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect("google.com:443").unwrap();
    let mut stream = connector.connect("google.com", stream).unwrap();

    stream.write_all(b"GET / HTTP/1.0\r\n\r\n").unwrap();
    let mut res = vec![];
    stream.read_to_end(&mut res).unwrap();
    println!("{}", String::from_utf8_lossy(&res));
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test() {
        assert!(true);
    }
}
