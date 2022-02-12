// use std::default::Default;
// use mysql::*;

use std::env;
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

    let url = env::var("MYSQL").unwrap_or_else(|_| "mysql://ilejn:@localhost:3306/test".into());

		let pool = Pool::new(url).unwrap();

		let conn = pool.get_conn().unwrap();

		Ok(conn)
}

pub fn get_user_by_session(session: &str) -> std::result::Result<u32, &'static str> {
		let mut  conn = get_conn().unwrap();


		// let res :std::result::Result<std::option::Option<i32>, mysql::Error>;
		let res = conn.exec_first("select id from test.users where session=?;", (session, ));

		match res {
				Ok(id) =>
						match id {
								Some(id) => Ok(id),
								None => Err("Not found"),
						}
        Err(_error) => Err("Some SQL error"),
		}


		// let mut id = 0;

		// row.unwrap().map(|(i)| {id = i;});

		// id
}

pub fn add_subscription(subscriber_id: u32, author_id: u32) {
		let mut  conn = get_conn().unwrap();
		conn.exec_drop("insert into test.subscriptions (subscriber_id, author_id) values (?, ?);",
									 (subscriber_id,
										author_id)).unwrap();
}

pub fn get_subscriptions(subscriber_id: u32) -> std::result::Result<String, &'static str> {
		let mut conn = get_conn().unwrap();
		let retvec :std::result::Result<Vec<i32>, mysql::Error>;
		retvec = conn.exec("select author_id from test.subscriptions where subscriber_id=?", (subscriber_id,));
		match retvec {
				Ok(vec) => {
						let mut res_str: String = "".to_string();
						for element in vec.iter() {
								res_str += &element.to_string();
								res_str += "\n";
						}
						Ok(res_str)
				}
				Err(_error) => Err("Some SQL error"),
		}
}

pub fn check_user(login: &str, pwdhash: &str) -> (u64, String) {
		let mut  conn = get_conn().unwrap();
		// let res :std::result::Result<std::option::Option<u64>, mysql::Error>;
		let row = conn.exec_first("select id, session from test.users where login=? and pwdhash=?;", (login, pwdhash));

		let mut id = 0;
		let mut session = "".to_string();

		row.unwrap().map(|(i, s)| {id = i; session = s;});

		(id, session)

}

pub fn add_user(login: &str,
								pwdhash: &str,
								name: &str,
								surname: &str,
								birthday: &str,
								city: &str,
								hobby: &str,
								session: &str
) -> Result<u64> {
		let mut  conn = get_conn().unwrap();
		conn.exec_drop("insert into test.users (login, pwdhash, name, surname, birthday, city, hobby, session) values (?, ?, ?, ?, ?, ?, ?, ?);",
									 (login,
										pwdhash,
										name,
										surname,
										birthday,
										city,
										hobby,
										session
		)).unwrap();
		Ok(conn.last_insert_id())
}
