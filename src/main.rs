mod types;

use headless_chrome::{Browser, LaunchOptions};

fn main() {

    const url: &str = "https://lining.studio/sales/guest/form/";

    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .build()
            .unwrap(),
    ).unwrap();

    let tab = browser.wait_for_initial_tab().unwrap();

    println!("Hello, world!");
}
