#[cfg(test)]
mod test {
    use crate::config::ENV;
    #[test]
    fn test() {
        println!("{:?}", ENV.database_url);
    }
}
