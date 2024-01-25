#[cfg(test)]
mod tests {
    use googletest::assert_that;
    use googletest::matchers::eq;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn insert(pool: PgPool) {
        sqlx::query!("INSERT INTO users (id, name) VALUES ($1, $2)", 1, "Alice")
            .execute(&pool)
            .await
            .unwrap();
        let n_rows: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
            .fetch_one(&pool)
            .await
            .unwrap()
            .unwrap();
        assert_that!(n_rows, eq(1));
    }
}
