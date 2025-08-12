use worker::{durable_object, Env, Request, Response, Result, RouteContext, SqlStorage, State};

#[durable_object]
pub struct SqlCounter {
    sql: SqlStorage,
}

impl DurableObject for SqlCounter {
    fn new(state: State, _env: Env) -> Self {
        let sql = state.storage().sql();
        // Create table if it does not exist
        sql.exec("CREATE TABLE IF NOT EXISTS counter(value INTEGER);", None)
            .expect("create table");
        Self { sql }
    }

    async fn fetch(&self, _req: Request) -> Result<Response> {
        #[derive(serde::Deserialize)]
        struct Row {
            value: i32,
        }

        // Read current value
        let rows: Vec<Row> = self
            .sql
            .exec("SELECT value FROM counter LIMIT 1;", None)?
            .to_array()?;
        let current = rows.get(0).map(|r| r.value).unwrap_or(0);
        let next = current + 1;

        // Update counter
        self.sql.exec("DELETE FROM counter;", None)?;
        self.sql
            .exec("INSERT INTO counter(value) VALUES (?);", vec![next.into()])?;

        Response::ok(format!("SQL counter is now {}", next))
    }
}

impl SqlCounter {
    pub async fn count(request: Request, ctx: RouteContext<()>) -> Result<Response> {
        let namespace = ctx.env.durable_object("SQL_COUNTER")?;
        let ud = namespace.id_from_name(request.url()?.path())?;
        let stub = ud.get_stub()?;
        let response = stub.fetch_with_request(request).await?;
        Ok(response)
    }
}
