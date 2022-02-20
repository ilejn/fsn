// use std::default::Default;
// use mysql::*;

use std::env;
use mysql::*;
use mysql::prelude::*;
use chrono::{NaiveDate, Utc};

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

pub fn add_subscription(subscriber_id: u32, author_id: u32) -> std::result::Result<(), &'static str>{
		let mut  conn = get_conn().unwrap();
		let ret :std::result::Result<(), mysql::Error>;
		ret = conn.exec_drop("insert into test.subscriptions (subscriber_id, author_id) values (?, ?);",
									 (subscriber_id,
										author_id));
		match ret {
				Ok(ok) => Ok(ok),
				Err(_error) => Err("Some SQL error"),
		}
}

struct Person {
		id: u32,
		name: String,
		surname: String,
}


pub fn get_subscriptions(subscriber_id: u32) -> std::result::Result<String, &'static str> {
		let mut conn = get_conn().unwrap();
		// let retvec :std::result::Result<Vec<i32>, mysql::Error>;
		let retvec = conn.exec_map("select author_id, name, surname  from test.subscriptions, test.users where subscriber_id=? and test.users.id=authod_id", (subscriber_id,),
													 |(author_id, name, surname)| Person {
															 id : author_id,
															 name : name,
															 surname : surname
													 }
		);
		match retvec {
				Ok(vec) => {
						let mut res_str: String = "".to_string();
						for element in vec.iter() {
								res_str += &element.id.to_string();
								res_str += "  ";
								res_str += &element.name;
								res_str += "  ";
								res_str += &element.surname;
								res_str += "\n";
						}
						Ok(res_str)
				}
				Err(_error) => Err("Some SQL error"),
		}
}

struct ExtPerson {
		id: u32,
		name: String,
		surname: String,
		city: String,
		birthday: NaiveDate,
		hobby: String,
}

pub fn lookup_users(name: &String,
									 surname: &String

									 // https://mobiarch.wordpress.com/2020/06/02/access-mysql-from-rust-part-i/
									 // https://docs.rs/mysql/latest/mysql/

) -> std::result::Result<String, &'static str> {
		let mut conn = get_conn().unwrap();
		let retvec = conn.exec_map("select id, name, surname, city, birthday, hobby from extusers where name like ? and surname like ? order by id limit 10000", (format!("{}%", name), format!("{}%", surname)),
												|(id, name, surname, city, birthday, hobby)| ExtPerson {
														id, name, surname, city, birthday, hobby
        }
		);

		match retvec {
				Ok(vec) => {
						let mut res_str: String = "".to_string();
						for element in vec.iter(){
								res_str += &format!("{}, {}, {}, {}, {}, {}\n", element.name, element.surname, element.id, element.city, element.birthday, element.hobby);
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

pub fn set_perspage(user_id: u32, pers_page: &String
) -> Result<u64> {
		let mut  conn = get_conn().unwrap();
		conn.exec_drop("upsert into test.perspages (user_id, pers_page) values (?, ?);",
									 (user_id,
										pers_page
	  )).unwrap();
		Ok(conn.last_insert_id())

}
