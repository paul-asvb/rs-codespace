use async_graphql::{
    http::GraphiQLSource, EmptyMutation, EmptySubscription, QueryRoot, Schema, SimpleObject,
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

type TypeOne = Option<Vec<CityComment>>;

async fn graphql_handler(schema: Extension<CityComment>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let type_one = r#"
  {
      "Berlin": "bla"
      "Paris": "bla2"
  },

"#;

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(type_one)
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
