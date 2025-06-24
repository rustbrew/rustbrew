use reqwest::Client;

async fn fetch_formula(name: &str) -> Result<Formula, reqwest::Error> {
    let url = format!("https://example.com/taps/formulae/{}.json", name);
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let formula: Formula = response.json().await?;
    Ok(formula)
}
