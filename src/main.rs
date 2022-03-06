mod types;

use std::sync::Arc;

use std::time;

use std::process::Command;

use headless_chrome::{
    protocol::cdp::types::Event, protocol::cdp::Page::CaptureScreenshotFormatOption, Browser,
    LaunchOptions,
};

macro_rules! element {
    ($t: expr,$($v: literal),*) => {
    ($(
    $t.find_element($v).expect(&format!("element for this selector {} was not found",$v)),
    )*)
    };

    ($t: expr,$v: literal) => {
    $t.find_element($v).expect(&format!("element for this selector {} was not found",$v))
    };
    }

const URL: &str = "https://lining.studio/sales/guest/form/";

const WEBHOOK_URL: &str = "https://discord.com/api/webhooks/949634126942715924/LQ2Es081IC-EKqh3RVg_caUKm3G4ALZIZJWPpcuQMT5YnrU04NowH73_A9xIQAQ1VITE";

fn screenshot_shipment(order_id: &String, last_name: &String, email: &String) {
    let browser = Browser::new(
        LaunchOptions::default_builder()
            .headless(false)
            .build()
            .unwrap(),
    )
    .unwrap();

    let tab = browser.wait_for_initial_tab().unwrap();

    tab.navigate_to(URL).unwrap();

    tab.wait_until_navigated().unwrap();

    tab.add_event_listener(Arc::new(move |e: &Event| match e {
        Event::PageWindowOpen(p) => {
            std::thread::sleep_ms(1500);

            let tabs = browser.get_tabs();

            let shipment_tab = &tabs.lock().unwrap()[0];

            shipment_tab.bring_to_front().unwrap();

            let orders = shipment_tab.wait_for_elements(".order-info").unwrap();

            if (orders.len() > 0) {
                let buffer = shipment_tab
                    .capture_screenshot(CaptureScreenshotFormatOption::Png, Some(85), None, true)
                    .expect("couldnt capure screenshot");

                let time = time::SystemTime::now()
                    .duration_since(time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis();

                std::fs::write(format!("./screenshots/{}.png", time), buffer).unwrap();

                Command::new("curl")
                    .args(vec![
                        "-F",
                        r#"'payload_json={"content": "Shipment Status"}'"#,
                        "-F",
                        &format!("file1=@screenshots/{}.png", time),
                        WEBHOOK_URL,
                    ])
                    .spawn()
                    .unwrap()
                    .wait()
                    .unwrap();

                shipment_tab.close(false).unwrap();
            }
        }
        _ => (),
    }))
    .unwrap();

    let (order_id_input, last_name_input, email_input, submit_button) = element!(
        tab,
        "#oar-order-id",
        "#oar-billing-lastname",
        "#oar_email",
        ".submit"
    );

    order_id_input.type_into(&order_id).unwrap();
    last_name_input.type_into(&last_name).unwrap();
    email_input.type_into(&email).unwrap();

    submit_button.click().unwrap();

    tab.wait_until_navigated().unwrap();

    let nav = &tab.wait_for_elements(".nav").unwrap()[2];

    nav.call_js_fn("(function() {this.children[0].click()})", vec![], false)
        .unwrap();

    let track = tab.wait_for_element(".track").unwrap();

    track.click().unwrap();

    tab.close(false).unwrap();
}

fn main() {
    let mut args = std::env::args();

    args.next();

    // get order id name from program arguments
    let order_id = args.next().expect("order_id is empty");

    // get billing last name for login from program arguments
    let last_name = args.next().expect("billing last name is empty");

    // get email for login from program arguments
    let email = args.next().expect("email is empty");

    screenshot_shipment(&order_id, &last_name, &email);

    loop {
        // every 5 hours
        std::thread::sleep(time::Duration::from_secs(60 * 60 * 5));

        screenshot_shipment(&order_id, &last_name, &email);
    }
}
