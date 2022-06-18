#[tokio::main]
async fn main() {
    println!("Check");
}

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn test() {
        assert!(true);
    }
}
