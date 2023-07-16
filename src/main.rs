#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use rocket::response::content::Html;
use rocket::State;
use std::sync::Arc;
mod portfolio;
use portfolio::PortfolioItem;
mod routes {
    use super::*;
    #[get("/")]
    pub(crate) fn index(portfolio: State<Vec<PortfolioItem>>) -> Html<String> {
        let mut html = String::new();
        html.push_str("<html><head><title>Portfolio</title></head><body>");
        for item in portfolio.iter() {
            html.push_str("<h2>");
            html.push_str(&item.title);
            html.push_str("</h2>");
            html.push_str("<p>");
            html.push_str(&item.description);
            html.push_str("</p>");
            html.push_str("<a href=\"");
            html.push_str(&item.url);
            html.push_str("\">Visit</a>");
        }
        Html(html)
    }
}
use routes::index;
fn main() {
    let portfolio_items = vec![
        PortfolioItem {
            title: "Project 1".to_string(),
            description: "Description 1".to_string(),
            url: "https://www.example.com/project1".to_string(),
        },
        PortfolioItem {
            title: "Project 2".to_string(),
            description: "Description 2".to_string(),
            url: "https://www.example.com/project2".to_string(),
        },
        PortfolioItem {
            title: "Project 3".to_string(),
            description: "Description 3".to_string(),
            url: "https://www.example.com/project3".to_string(),
        },
    ];
    rocket::ignite()
        .manage(portfolio_items)
        .mount("/", routes![routes::index])
        .launch();
}