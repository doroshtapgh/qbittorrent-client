mod error;
mod client;
pub mod models;
pub use error::QBittorrentError;
pub use client::QBittorrentClient;

/*pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
