// use std::default::Default;
// use mysql::*;

use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*; //For date and time


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

pub fn check_user() -> Result<u64> {
		let mut  conn = get_conn().unwrap();
		let res = conn.exec_first("select user_id from test.users where user_name=?;", ("vasya",)).unwrap();
		Ok(res.unwrap())
}
