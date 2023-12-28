use super::spec::TodoRepository;
use crate::backend::database::schema::todos;
use crate::backend::model::Todo;
use async_trait::async_trait;
use diesel::{
    prelude::PgConnection,
    query_dsl::methods::{FilterDsl, FindDsl, SelectDsl},
    r2d2::{ConnectionManager, Pool},
    Connection, ExpressionMethods, RunQueryDsl, SelectableHelper,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;
use std::io::{Error, ErrorKind};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct TodoPostgresRespository {
    pool: DbPool,
}

impl TodoPostgresRespository {
    pub fn new() -> Self {
        let database_url = &env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = diesel::r2d2::Pool::builder().build(manager).unwrap();

        let mut conn = PgConnection::establish(&database_url).unwrap();
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
        conn.run_pending_migrations(MIGRATIONS).unwrap();

        return Self { pool };
    }
}

#[async_trait]
impl TodoRepository for TodoPostgresRespository {
    async fn create(&mut self, title: &str) -> Todo {
        let todo = Todo::new(String::from(title));
        diesel::insert_into(todos::table)
            .values(todo.clone())
            .execute(&mut self.pool.get().unwrap())
            .unwrap();
        todo
    }
    async fn get(&mut self) -> Vec<Todo> {
        todos::table
            .load::<Todo>(&mut self.pool.get().unwrap())
            .unwrap()
    }
    async fn get_by_id(&mut self, id: uuid::Uuid) -> Option<Todo> {
        match todos::table
            .filter(todos::id.eq(id))
            .select(Todo::as_select())
            .get_result(&mut self.pool.get().unwrap())
        {
            Ok(todo) => Some(todo),
            Err(_) => None,
        }
    }

    async fn update(
        &mut self,
        id: uuid::Uuid,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<Todo, Error> {
        let title_expression = title.map(|t| todos::title.eq(t));
        let completed_expression = completed.map(|c| todos::completed.eq(c));
        match diesel::update(todos::table.find(id))
            .set((title_expression, completed_expression))
            .returning(Todo::as_returning())
            .get_result(&mut self.pool.get().unwrap())
        {
            Ok(todo) => Ok(todo),
            Err(_) => Err(Error::new(ErrorKind::NotFound, "Todo not found!")),
        }
    }

    async fn delete(&mut self, id: uuid::Uuid) -> Result<Todo, Error> {
        match diesel::delete(todos::table.find(id)).get_result(&mut self.pool.get().unwrap()) {
            Ok(todo) => Ok(todo),
            Err(_) => Err(Error::new(ErrorKind::NotFound, "Todo not found!")),
        }
    }
}
