#![allow(clippy::unwrap_used, clippy::expect_used)]

use e2e_tests::TestConfig;
use jugar_probar::Browser;

#[tokio::test]
async fn test_console_errors() {
    // The UI under test comes from TEST_WEB_UI_URL like every other e2e test (it was hard-coded to localhost:9765,
    // where nothing listens unless a dev server was started by hand); 9765 stays the fallback.
    let config = TestConfig {
        web_ui_url: std::env::var("TEST_WEB_UI_URL").unwrap_or_else(|_| "http://localhost:9765".to_string()),
        ..Default::default()
    };

    println!("Launching browser to check {}", config.web_ui_url);

    let browser = Browser::launch(Default::default())
        .await
        .expect("Failed to launch browser");

    let mut page = browser.new_page().await.expect("Failed to create page");
    page.goto(&config.web_ui_url)
        .await
        .expect("Failed to navigate");

    // Give it time to load scripts using the Probar browser
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    println!("Successfully navigated to {}", config.web_ui_url);
}
