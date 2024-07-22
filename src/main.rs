//! Stream stats for all running Docker containers asynchronously
#![type_length_limit = "2097152"]

mod leptos_axum;

use std::env;

use disk::analyse_disk_space_at;
use leptos_axum::LeptosHtml;

mod disk;

use axum::{routing::get, Router};
use leptos::view;
use tower_http::services::ServeDir;

async fn index() -> LeptosHtml {
    let paths = env::var("PATHS").unwrap_or("./".into());
    println!("reading paths: {}", paths);
    let paths_to_check: Vec<&str> = paths.split(",").collect();

    let results = analyse_disk_space_at(paths_to_check).await;

    if let Err(_) = results {
        return "Error".to_owned().into();
    }

    let mut results = results.unwrap();
    results.sort_by(|a, b| a.path.cmp(&b.path));

    return view! {
        <html lang="en">
            <head>
                <title>Disk Stats</title>
                <meta charset="UTF-8"></meta>
                <meta name="viewport" content="width=device-width, initial-scale=1"></meta>
                <link href="/assets/index.css" rel="stylesheet"></link>
            </head>
            <body>
            <h1>Disk Stats</h1>
            <table>
                <tr>
                    <th>
                        Path
                    </th>
                    <th>
                        Availiable
                    </th>
                    <th>
                        Used
                    </th>
                    <th>
                        Total
                    </th>
                </tr>
                    {results.into_iter().map(|result| view! {
                        <tr style=("--used", format!("{}%", result.percent))>
                            <td> {result.path} </td>
                            <td> {human_readable_bytes(result.avail)} </td>
                            <td> {human_readable_bytes(result.used)} </td>
                            <td> {human_readable_bytes(result.total)} </td>
                        </tr>
                    }).collect::<Vec<_>>()}
            </table>
        </body>
    </html>
    }
    .into();
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/assets", ServeDir::new("assets"));

    println!("Listening on: http://localhost:5609");

    axum::Server::bind(&"0.0.0.0:5609".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn human_readable_bytes(kib: u64) -> String {
    if kib == 0 {
        return "Error".to_string();
    }

    // let kb = bytes as f64 / 1024.0;
    let mib = kib as f64 / 1024.0;
    let gib = mib / 1024.0;
    let tib = gib / 1024.0;

    if tib >= 1.0 {
        format!("{:.2} TB", tib)
    } else if gib >= 1.0 {
        format!("{:.2} GB", gib)
    } else if mib >= 1.0 {
        format!("{:.2} MB", mib)
    } else {
        format!("{:.2} KB", kib)
    }
}
