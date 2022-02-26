use kv::*;

pub fn store_set_string(key: &str, value: &str) -> Result<(), Error> {
    let cfg = Config::new("./out/fa_cose_store");
    let store = Store::new(cfg)?;

    let fa_cose_bucket = store.bucket::<String, String>(Some("fa_cose"))?;
    fa_cose_bucket.set(&key.to_string(), &value.to_string())
}

pub fn store_get_string(key: &str) -> Option<String> {
    let cfg = Config::new("./out/fa_cose_store");
    let store = Store::new(cfg);

    if store.is_err(){
        eprintln!("Store error: {:?}", store.err().unwrap());
        return None;
    }

    let fa_cose_bucket = store.unwrap().bucket::<String, String>(Some("fa_cose"));
    if fa_cose_bucket.is_err(){
        eprintln!("Bucket error: {:?}", fa_cose_bucket.err().unwrap());
        return None;
    }

    match fa_cose_bucket.unwrap().get(&key.to_string()) {
        Ok(val) => { val }
        Err(_) => None
    }
}

