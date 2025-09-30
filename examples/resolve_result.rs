use busybody::helpers;

#[tokio::main]
async fn main() {
    helpers::resolver(|_| async {
        if rand::random_bool(0.3) {
            Result::<DbConnection, String>::Err(String::from("database is down"))
        } else {
            Result::<DbConnection, String>::Ok(DbConnection)
        }
    })
    .await;

    for id in 1..=20 {
        helpers::resolve_and_call(move |db_result: Result<DbConnection, String>| async move {
            match db_result {
                Ok(db) => db.persist(id),
                Err(e) => println!("{}", e),
            };
        })
        .await
    }
}

#[derive(Clone)]
struct DbConnection;

impl DbConnection {
    fn persist(&self, id: i64) {
        println!("persisting id: {}", id)
    }
}
