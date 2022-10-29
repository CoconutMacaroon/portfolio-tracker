use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use comfy_table::TableComponent::*;
use indoc::indoc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::vec;
use text_io::read;

#[derive(Serialize, Deserialize, Debug)]
struct Portfolio {
    assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    ticker: String,
    buy_price_cents: u32,
    // technically we don't care about the current price if it is sold, but it is still a valid property to have, so we include it here, although it isn't displayed
    current_price_cents: u32,
    // if sell price is None, it isn't sold
    sell_price_cents: Option<u32>,
}

fn is_asset_sold(asset: &Asset) -> bool {
    // if there is no sell price, then it isn't sold (i.e., it is currently held)
    match asset.sell_price_cents {
        Some(_x) => true,
        None => false,
    }
}

fn is_asset_held(asset: &Asset) -> bool {
    !is_asset_sold(asset)
}

fn percent_increase(old: u32, new: u32) -> f32 {
    // ensure floating point math
    (new as f32 - old as f32) / old as f32 * 100_f32
}

fn format_money(cents: u32) -> String {
    format!("${:.2}", cents as f32 / 100.0)
}

fn print_assets(assets: &Vec<Asset>) {
    let mut table = Table::new();

    // I prefer a Unicode table with solid lines
    table.load_preset(UTF8_FULL);
    table.set_style(VerticalLines, '│');
    table.set_style(HorizontalLines, '─');

    table.set_header(vec![
        "Ticker",
        "Buy Price",
        "Current Price",
        "Percent Change",
        "Sell Price",
    ]);

    for asset in assets {
        table.add_row(vec![
            // ticker
            asset.ticker.clone(),
            // buy price (formatted as money)
            format_money(asset.buy_price_cents),
            // current price (formatted as money) if held, else the current price is irrelevant
            if is_asset_held(asset) {
                format_money(asset.current_price_cents)
            } else {
                "N/A (sold)".to_string()
            },
            // percent change - calculate on current price if held, calculate on sell price if sold
            format!(
                "{:.2}%",
                percent_increase(
                    asset.buy_price_cents,
                    if is_asset_held(asset) {
                        asset.current_price_cents
                    } else {
                        asset.sell_price_cents.unwrap()
                    }
                )
            ),
            // sell price - show N/A if not sold
            if is_asset_sold(asset) {
                format_money(asset.sell_price_cents.unwrap())
            } else {
                "N/A (currently held)".to_string()
            },
        ]);
    }
    println!("{table}");
}

fn get_current_ticker_price(_ticker: String) -> u32 {
    // TODO: use Yahoo Finance for this
    200
}

fn add_asset() -> Asset {
    print!("Enter ticker: ");
    let symbol: String = read!();

    print!("Enter buy price in total cents: ");
    let buy_price: u32 = read!();

    print!("Enter sell price if sold, otherwise enter 'held': ");
    let sell_price_raw: String = read!();

    // if I access a string twice I have to make it owned for some reason - IDK
    // what that means or if there is a better way
    Asset {
        ticker: symbol.to_owned(),
        buy_price_cents: buy_price,
        current_price_cents: get_current_ticker_price(symbol),
        sell_price_cents: (if sell_price_raw.eq("held") {
            None
        } else {
            Some(sell_price_raw.parse().unwrap())
        }),
    }
}

fn print_help() {
    let help_text = indoc! {"
    assets - prints all assets, both held and sold
    new - adds a new asset
    help - prints this help text
    load - loads assets from a file"};
    println!("{}", help_text);
}

fn prompt(text: &str) -> String {
    print!("{}", text);
    let mut data: String = read!("{}\n");

    // on Windows systems, if we read to \n, a \r character can be appended
    // so we remove it - AFAIK this is a Windows-only isse
    if data.ends_with('\r') {
        data.pop();
    }
    data
}

fn load_portfolio() -> Option<Portfolio> {
    // get the filename and read the file
    let filename = prompt("Enter filename to load: ");
    let data = fs::read_to_string(filename);
    let raw_portfolio: String = if let Ok(x) = data { x } else { return None };

    // convert the read file into an actual Portfolio struct
    let portfolio = serde_json::from_str(&raw_portfolio);

    if let Ok(x) = portfolio {
        Some(x)
    } else {
        None
    }
}

fn dump_portfolio(portfolio: &Portfolio) {
    let json = serde_json::to_string(&portfolio);
    let filename = prompt("Enter filename to dump assets to: ");
    if let Ok(x) = json {
        let result = fs::write(filename, x);
        if result.is_err() {
            println!("Error occurred when dumping. Portfolio not dumped.");
        }
    } else {
        println!("Error occurred when dumping. Portfolio not dumped.");
    }
}

fn main() {
    let mut active_portfolio: Portfolio = Portfolio { assets: vec![] };
    let mut input: String;
    loop {
        input = prompt("» ");

        match input.as_str() {
            "assets" => print_assets(&active_portfolio.assets),
            "new" => active_portfolio.assets.push(add_asset()),
            "help" => print_help(),
            "load" => match load_portfolio() {
                None => println!("An error occurred when loading portfolio. Portfolio not loaded."),
                Some(x) => active_portfolio = x,
            },
            "dump" => dump_portfolio(&active_portfolio),
            "exit" => break,
            "" => continue,
            _ => println!("Unknown command. Enter 'help' for a list of valid commands"),
        }
    }
}
