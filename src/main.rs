#![feature(decl_macro)]
extern crate comrak;
use comrak::ComrakOptions;
use comrak::{ComrakParseOptions, ComrakExtensionOptions, ComrakRenderOptions};

#[macro_use] extern crate rocket;
use rocket::{Request, State, Rocket};

extern crate rocket_contrib;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

use serde::Serialize;

use std::fs;

#[derive(Serialize)]
struct HtmlMarkup {
    html: String,
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let context = HtmlMarkup {
        html: format!(
            r#"<h1>404</h1>
            <p>That's an error.  I couldn't find "{}".</p>
            <p>If you think you reached this page in error, please reach out to me and let me know!</p>"#,
            req.uri()
        ),
    };
    Template::render("base", context)
}

#[get("/")]
fn index(comrak_options: State<ComrakOptions>) -> Template {
    let markdown = fs::read_to_string("home.md").unwrap();
    let html = comrak::markdown_to_html(markdown.as_str(), comrak_options.inner());
    Template::render("home", HtmlMarkup  { html })
}

fn rocket() -> Rocket {
    // unsafe_ allows rendering of raw HTML, which I need for named anchors
    // probably not a huge issue for my simple portfolio website
    let mut render_options = ComrakRenderOptions::default();
    render_options.github_pre_lang = true;
    render_options.unsafe_ = true;

    let mut extension_options = ComrakExtensionOptions::default();
    extension_options.strikethrough =  true;
    extension_options.table = true;
    extension_options.footnotes = true;
    extension_options.description_lists = true;

    rocket::ignite()
        .register(catchers![not_found])
        .manage(ComrakOptions {
            extension: extension_options,
            parse: ComrakParseOptions::default(),
            render: render_options,
        })
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("static"))
        .mount("/templates", StaticFiles::from("templates"))
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}
