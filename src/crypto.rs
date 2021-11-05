// use argon2::{self, Config, ThreadMode, Variant, Version};

pub fn mk_hash(password: &String) -> String {
		let salt = b"randomsalt";
		// let password = b"password";
		let config = argon2::Config::default();
		let hash = argon2::hash_encoded(password.as_bytes() , salt, &config).unwrap();
    log::info!("hash {}", hash);
		hash
}

// pub fn check_hash(hash: &String, password: &String) -> bool {
// 		let res = argon2::verify_encoded(hash, password.as_bytes()).unwrap();
// 		res
// }
