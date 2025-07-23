use thirtyfour::prelude::*;
use tokio;

#[tokio::test]
async fn test_ui_loads() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--headless")?;
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    async fn check_page(driver: &WebDriver, url: &str, title: &str) -> WebDriverResult<()> {
        driver.goto(url).await?;
        assert!(driver.find(By::Tag("header")).await.is_ok());
        assert!(driver.find(By::Tag("main")).await.is_ok());
        assert!(driver.find(By::Tag("footer")).await.is_ok());
        let h1 = driver.find(By::Tag("h1")).await?;
        assert_eq!(h1.text().await?, title);
        Ok(())
    }

    check_page(&driver, "http://localhost:8081", "Welcome to the Slope.Ski").await?;
    check_page(&driver, "http://localhost:8081/swap", "Swap").await?;
    check_page(&driver, "http://localhost:8081/farm", "Farm").await?;
    check_page(&driver, "http://localhost:8081/pool", "Pool").await?;

    driver.quit().await?;

    Ok(())
}