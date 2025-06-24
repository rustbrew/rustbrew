async fn install_package(package: &str) -> Result<(), Box<dyn std::error::Error>> {
    let formula = fetch_formula(package).await?;
    let client = Client::new();
    let response = client.get(&formula.url).send().await?;
    let bytes = response.bytes().await?;
    
    // Save to cache
    std::fs::write(format!("/usr/local/rustbrew/cache/{}-{}.tar.gz", formula.name, formula.version), bytes)?;
    
    // Execute install script (simplified)
    std::process::Command::new("sh")
        .arg("-c")
        .arg(&formula.install_script)
        .status()?;
    
    // Update local database
    update_db(&formula)?;
    Ok(())
}
