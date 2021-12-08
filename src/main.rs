use sqlx::mysql::MySqlPool;

const MESSAGE_SIZE: i64 = 1024 * 1024;
const VARCHAR_COLUMN_MAX_LEN: i64 = 16383;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPool::connect("mysql://root@127.0.0.1:4000/test").await?;
    println!("{}", &gen_create_big_table_sql());
    sqlx::query(&gen_create_big_table_sql())
        .execute(&pool)
        .await?;
    sqlx::query(&gen_insert_sql()).execute(&pool).await?;

    Ok(())
}

fn gen_create_big_table_sql() -> String {
    let mut cols = String::new();
    for i in 0..MESSAGE_SIZE / VARCHAR_COLUMN_MAX_LEN {
        cols.push_str(&format!(", a{} VARCHAR({})", i, VARCHAR_COLUMN_MAX_LEN));
    }

    format!("CREATE TABLE test(id int primary key{});", cols)
}

fn gen_insert_sql() -> String {
    let value = "a".repeat(VARCHAR_COLUMN_MAX_LEN as usize);
    let mut values = String::new();
    for _ in 0..MESSAGE_SIZE / VARCHAR_COLUMN_MAX_LEN {
        values.push_str(&format!(", '{}'", value));
    }

    format!("INSERT INTO test VALUES (1{});", values)
}
