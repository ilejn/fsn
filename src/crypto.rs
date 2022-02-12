// use argon2::{self, Config, ThreadMode, Variant, Version};
use rand::Rng;

pub fn mk_hash(password: &String) -> String {
		let salt = b"randomsalt";
		// let password = b"password";
		let config = argon2::Config::default();
		let hash = argon2::hash_encoded(password.as_bytes() , salt, &config).unwrap();
    log::info!("hash {}", hash);
		hash
}

pub fn mk_random_string() -> String {
		let salt = b"randomsalt";
		let mut rng = rand::thread_rng();
		let rnd_num: u32 = rng.gen();
		let config = argon2::Config::default();
		let rnd = argon2::hash_encoded(rnd_num.to_string().as_bytes() , salt, &config).unwrap();
    log::info!("rnd {}", rnd);
		rnd

}

// pub fn check_hash(hash: &String, password: &String) -> bool {
// 		let res = argon2::verify_encoded(hash, password.as_bytes()).unwrap();
// 		res
// }
