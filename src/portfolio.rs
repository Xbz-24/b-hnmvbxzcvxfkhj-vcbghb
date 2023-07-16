#[derive(Debug)]
pub struct PortfolioItem {
    pub title: String,
    pub description: String,
    pub url: String,
}
impl PortfolioItem {
    pub fn new(title: &str, description: &str, url: &str) -> PortfolioItem {
        PortfolioItem {
            title: title.to_string(),
            description: description.to_string(),
            url: url.to_string(),
        }
    }

}