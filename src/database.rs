use rocksdb::{Options, DB};

// @dev Initialize the RocksDB instance
pub fn init_db(path: &str) -> std::io::Result<DB> {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    DB::open(&opts, path).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

// @dev Load the last processed block height from RocksDB
pub fn load_last_processed_block(db: &Arc<Mutex<DB>>) -> std::io::Result<u64> {
    let db = db.lock().unwrap();
    match db.get(LAST_PROCESSED_BLOCK_KEY) {
        Ok(Some(value)) => {
            let height = u64::from_le_bytes(value.try_into().expect("Invalid block height format"));
            Ok(height)
        }
        Ok(None) => Ok(0), // If no value, start from block 0
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}

// @dev Save the last processed block height to RocksDB
pub fn save_last_processed_block(db: &Arc<Mutex<DB>>, block_height: u64) -> std::io::Result<()> {
    let db = db.lock().unwrap();
    db.put(LAST_PROCESSED_BLOCK_KEY, &block_height.to_le_bytes())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
