// use std::default::Default;
// use mysql::*;

use mysql::*;
use mysql::prelude::*;
// use chrono::prelude::*; //For date and time


// use mysql::conn::MyOpts;
// use mysql::conn::pool::MyPool;
// use mysql::value::from_value;

pub fn get_conn() -> Result<PooledConn> {
    // let opts = MyOpts {
    //       user: Some("root".to_string()),
    //       pass: Some("password".to_string()),
    //       ..Default::default()
    // };


    let url = "mysql://ilejn:@localhost:3306/test";

		let pool = Pool::new(url).unwrap();

		let conn = pool.get_conn().unwrap();

		Ok(conn)
}

pub fn check_user(login: &String, pwdhash: &String) -> u64 {
		let mut  conn = get_conn().unwrap();
		let res :std::result::Result<std::option::Option<u64>, mysql::Error>;
		res = conn.exec_first("select id from test.users where login=? and pwdhash=?;", (login, pwdhash));
		match res {
				Ok(id) =>
						match id {
								Some(id) => id,
								None => 0,
						}
        Err(_error) => 0,
		}
		// Ok(res.unwrap())
		// Ok(5)
}

pub fn add_user(login: &String,
								pwdhash: &String,
								name: &String,
								surname: &String,
								birthday: &String,
								city: &String,
								hobby: &String
) -> Result<u64> {
		let mut  conn = get_conn().unwrap();
		conn.exec_drop("insert into test.users (login, pwdhash, name, surname, birthday, city, hobby) values (?, ?, ?, ?, ?, ?, ?);", (login,
																																						 pwdhash,
																																						 name,
																																						 surname,
																																						 birthday,
																																						 city,
																																						 hobby
		)).unwrap();
		Ok(conn.last_insert_id())
}
