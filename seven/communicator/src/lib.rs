#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod network {
    pub mod client;
    pub mod server;

    pub fn connect() {
    }
}
