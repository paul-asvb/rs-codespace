use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, FieldResult, Object, Schema,
    SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, SimpleObject)]
struct CityComment {
    city: String,
    comment: String,
}

type AppState = CityComment;
pub struct QueryRoot;

type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

/// Define GraphQL query schema.
#[Object]
impl QueryRoot {
    async fn get_city(&self, ctx: &'_ Context<'_>) -> FieldResult<CityComment> {
        let bla = r#"
            {
                "city": "bla",
                "comment": "bla2"
            }"#;

        serde_json::from_str(bla).map_err(|error| error.into())
    }
}

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let state = CityComment {
        city: "city123".to_string(),
        comment: "comment123".to_string(),
    };

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
