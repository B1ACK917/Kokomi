use std::env;
use once_cell::sync::Lazy;


#[derive(Debug)]
pub enum KKMError {
    ModelNotSpecifiedError(&'static str)
}

pub static DEBUG: Lazy<bool> = Lazy::new(|| {
    match env::var("KKM_DEBUG") {
        Ok(_) => true,
        Err(_) => false
    }
});
