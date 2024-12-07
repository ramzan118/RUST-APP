use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn home() -> impl Responder {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>My Rust App</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                margin: 0;
                padding: 0;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100vh;
                background-color: #f0f0f0;
            }
            header {
                background-color: #4CAF50;
                color: white;
                padding: 1em;
                text-align: center;
                width: 100%;
            }
            main {
                padding: 2em;
                text-align: center;
            }
            footer {
                background-color: #4CAF50;
                color: white;
                padding: 1em;
                text-align: center;
                width: 100%;
                position: absolute;
                bottom: 0;
            }
            button {
                background-color: #4CAF50;
                color: white;
                border: none;
                padding: 1em;
                cursor: pointer;
                margin-top: 1em;
            }
            button:hover {
                background-color: #45a049;
            }
            nav a {
                margin: 0 1em;
                text-decoration: none;
                color: #4CAF50;
            }
        </style>
    </head>
    <body>
        <header>
            <h1>Welcome to My Rust App</h1>
            <nav>
                <a href="/">Home</a>
                <a href="/about">About</a>
                <a href="/contact">Contact Us</a>
            </nav>
        </header>
        <main>
            <h2>rustc version: 1.83.0</h2>
            <h2>cargo version: 1.83.0</h2>
            <button onclick="alert('Hello, world!')">Click Me</button>
        </main>
        <footer>
            <p>© 2024 My Rust App</p>
        </footer>
    </body>
    </html>
    "#;

    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn about() -> impl Responder {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>About Us</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                margin: 0;
                padding: 0;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100vh;
                background-color: #f0f0f0;
            }
            header {
                background-color: #4CAF50;
                color: white;
                padding: 1em;
                text-align: center;
                width: 100%;
            }
            main {
                padding: 2em;
                text-align: center;
            }
            footer {
                background-color: #4CAF50;
                color: white;
                padding: 1em;
                text-align: center;
                width: 100%;
                position: absolute;
                bottom: 0;
            }
            nav a {
                margin: 0 1em;
                text-decoration: none;
                color: #4CAF50;
            }
        </style>
    </head>
    <body>
        <header>
            <h1>About Us</h1>
            <nav>
                <a href="/">Home</a>
                <a href="/about">About</a>
                <a href="/contact">Contact Us</a>
            </nav>
        </header>
        <main>
            <p>Welcome to the About Us page. Here you can learn more about our application and team.</p>
        </main>
        <footer>
            <p>© 2024 My Rust App</p>
        </footer>
    </body>
    </html>
    "#;

    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn contact() -> impl Responder {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Contact Us</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                margin: 0;
                padding: 0;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                height: 100vh;
                background-color: #f0f0f0;
            }
            header {
                background-color: #4CAF50;
                color: white;
                padding: 1em;
                text-align: center;
                width: 100%;
            }
            main {
                padding: 2em;
                text-align: center;
            }
            footer {
                background-color: #4CAF50;
                color: white;
                padding: 1em;
                text-align: center;
                width: 100%;
                position: absolute;
                bottom: 0;
            }
            nav a {
                margin: 0 1em;
                text-decoration: none;
                color: #4CAF50;
            }
            form {
                display: flex;
                flex-direction: column;
                align-items: center;
            }
            input, textarea {
                margin: 0.5em 0;
                padding: 0.5em;
                width: 300px;
            }
            button {
                background-color: #4CAF50;
                color: white;
                border: none;
                padding: 1em;
                cursor: pointer;
                margin-top: 1em;
            }
            button:hover {
                background-color: #45a049;
            }
        </style>
    </head>
    <body>
        <header>
            <h1>Contact Us</h1>
            <nav>
                <a href="/">Home</a>
                <a href="/about">About</a>
                <a href="/contact">Contact Us</a>
            </nav>
        </header>
        <main>
            <form action="/submit_contact" method="post">
                <input type="text" name="name" placeholder="Your Name" required>
                <input type="email" name="email" placeholder="Your Email" required>
                <textarea name="message" placeholder="Your Message" required></textarea>
                <button type="submit">Send</button>
            </form>
        </main>
        <footer>
            <p>© 2024 My Rust App</p>
        </footer>
    </body>
    </html>
    "#;

    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn submit_contact(form: web::Form<ContactForm>) -> impl Responder {
    let response = format!(
        "Thank you, {}! We have received your message: {}",
        form.name, form.message
    );
    HttpResponse::Ok().body(response)
}

#[derive(serde::Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
            .route("/about", web::get().to(about))
            .route("/contact", web::get().to(contact))
            .route("/submit_contact", web::post().to(submit_contact))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

