pub async fn fetch_yahoo_history(
    symbol: &str,
    start: i64, // unix timestamp
    end: i64,
) -> Result<Vec<StockQuote>, Box<dyn Error>> {
    let url = format!(
        "https://query1.finance.yahoo.com/v7/finance/download/{sym}?period1={start}&period2={end}&interval=1d&events=history",
        sym = symbol,
        start = start,
        end = end
    );

    let resp = Client::new().get(&url).send().await?.text().await?;
    let mut rdr = csv::Reader::from_reader(resp.as_bytes());

    let mut quotes = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let date = NaiveDate::parse_from_str(&record[0], "%Y-%m-%d")?;
        let close: f64 = record[4].parse()?; // "Close" column
        quotes.push(StockQuote { date, close });
    }

    Ok(quotes)
}