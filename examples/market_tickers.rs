use anyhow::Result;
use okx_api::apikey::OkxClient;
#[tokio::main]
async fn main() -> Result<()> {
    println!("[*] Run {:?}", run().await?);

    Ok(())
}

async fn run() -> Result<()> {
    let web_rest_url_public = "https://www.okx.com";

    let debug = true;
    let testnet = false;

    let okxclient = OkxClient::new(
        debug,
        testnet,
        "XXX",
        "XXX",
        "XXX",
        "XXX",
        web_rest_url_public,
    );

    let result = okxclient.market_tickers("SWAP", None, None).await?;

    Ok(())
}
