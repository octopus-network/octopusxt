
#[subxt::subxt(runtime_metadata_path = "metadata/substrate.scale")]
pub mod substrate {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
