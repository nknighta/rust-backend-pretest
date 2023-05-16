#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

// customerテーブルの日付型カラム(reg_date)を扱うのに必要
use chrono::NaiveDate;

// "src/schema.rs"で定義したマクロを使えるようにする
mod schema;
use schema::customer::dsl::*;

// テーブルcustomerの各行の情報を格納する構造体
// （Web上のサンプルプロブラムでは別ソースmodels.rsに書かれていることが多い）
// ageはNULLを取ることがあり、Option型で受ける必要あり
#[derive(Queryable, Debug)]
struct Customer {
    name: String,
    age: Option<i32>,
    reg_date: NaiveDate,
}

/// DBの接続を確立
fn establish_connection() -> PgConnection {
    dotenv().ok();

    // .envファイルに定義された環境変数DATABASE_URLを取得してDBに接続する
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    // DBの接続を確立
    let mut conn:PgConnection = establish_connection();

    // customerテーブルの各情報を取得
    // SQLで「SELECT * FROM customer;」をやっているのと同じ
    let results = customer
        .load::<Customer>(&mut conn )
        .expect("Error loading customer");

    // 結果を表示
    for r in results {
        println!("{:?}", r);
    }
}