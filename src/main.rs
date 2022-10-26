use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use comfy_table::TableComponent::*;

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

fn print_assets(assets: &[Asset]) {
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

    println!("Assets");
    println!("{table}");
}

fn main() {
    let assets: [Asset; 2] = [
        Asset {
            ticker: "MSFT".to_string(),
            buy_price_cents: 1000,
            current_price_cents: 25000,
            sell_price_cents: None,
        },
        Asset {
            ticker: "APPL".to_string(),
            buy_price_cents: 30000,
            current_price_cents: 10000000,
            sell_price_cents: Some(40000),
        },
    ];

    print_assets(&assets);
    println!();
}
