use {
    std::path::PathBuf,
    tide::{prelude::*, Request},
};

#[derive(Deserialize)]
struct Info {
    name: String,
    address: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Starting the web server on port 8080");

    let mut app = tide::new();
    let static_dir = PathBuf::from("static/");

    app.at("/").serve_file(static_dir.join("index.html"))?;
    app.at("/").serve_dir(static_dir);

    app.at("/hello")
        .get(|_| async { Ok("Hello, server is up!") });

    app.at("/form").post(form_handler);

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

async fn form_handler(mut req: Request<()>) -> tide::Result {
    let info: Info = req.body_form().await?;

    Ok(format!(
        "Form value name = {} and address = {}",
        info.name, info.address
    )
    .into())
}
