use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio_postgres::{NoTls, Client};

#[derive(Deserialize, Serialize)]
struct Package {
    name: String,
    version: String,
    functions: Vec<Function>,
}

#[derive(Deserialize, Serialize)]
struct Function {
    name: String,
    params: Vec<String>,
    body: Vec<String>,
}

async fn get_package(path: web::Path<String>, client: web::Data<Arc<Client>>) -> impl Responder {
    let package_name: String = path.into_inner();
    let package_query: String = format!("SELECT * FROM packages WHERE name = '{}'", package_name);

    match client.query(&package_query, &[]).await {
        Ok(package_rows) => {
            if package_rows.is_empty() {
                return HttpResponse::NotFound().body("Package not found");
            }

            let package_id: i32 = package_rows[0].get("id");
            let functions_query: String = format!("SELECT * FROM functions WHERE package_id = {}", package_id);

            match client.query(&functions_query, &[]).await {
                Ok(function_rows) => {
                    let functions: Vec<Function> = function_rows.iter().map(|row| Function {
                        name: row.get("name"),
                        params: row.get("params"),
                        body: row.get("body"),
                    }).collect();

                    let package: Package = Package {
                        name: package_rows[0].get("name"),
                        version: package_rows[0].get("version"),
                        functions,
                    };

                    HttpResponse::Ok().json(package)
                }
                Err(err) => HttpResponse::InternalServerError().body(format!("Error fetching functions: {}", err)),
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error fetching package: {}", err)),
    }
}

async fn contribute_package(package: web::Json<Package>, client: web::Data<Arc<Client>>) -> impl Responder {
    let package = package.into_inner();
    let package_query = format!("SELECT * FROM packages WHERE name = '{}'", package.name);

    match client.query(&package_query, &[]).await {
        Ok(package_rows) => {
            if !package_rows.is_empty() {
                return HttpResponse::BadRequest().body("Package already exists");
            }

            let insert_package_query = format!(
                "INSERT INTO packages (name, version) VALUES ('{}', '{}') RETURNING id",
                package.name, package.version
            );

            match client.query_one(&insert_package_query, &[]).await {
                Ok(row) => {
                    let package_id: i32 = row.get("id");

                    for function in package.functions {
                        let insert_function_query: String = format!(
                            "INSERT INTO functions (package_id, name, params, body) VALUES ({}, '{}', '{:?}', '{:?}')",
                            package_id, function.name, function.params, function.body
                        );

                        if let Err(err) = client.execute(&insert_function_query, &[]).await {
                            return HttpResponse::InternalServerError().body(format!("Error inserting function: {}", err));
                        }
                    }

                    HttpResponse::Ok().body("Package contributed successfully")
                }
                Err(err) => HttpResponse::InternalServerError().body(format!("Error inserting package: {}", err)),
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error checking package existence: {}", err)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connection_string: String = env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set");

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await.unwrap();
    let client: Arc<Client> = Arc::new(client);

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/packages/{name}", web::get().to(get_package))
            .route("/package/contribute", web::post().to(contribute_package))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}