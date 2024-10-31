use anyhow::bail;
use wordpress_plugins_agregator::models::Plugins;
use reqwest::header::{HeaderMap, USER_AGENT};
use std::process::Command;
// use subversion::client::CheckoutOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // make GET request to target URL and retrieve response
    // let resp = reqwest::get("https://plugins.svn.wordpress.org/")
    //     .await?
    //     .text()
    //     .await?;
    // println!("{resp:#?}");

    let client = reqwest::Client::new();

    // Create a custom User-Agent string
    let custom_user_agent =
        "Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0";

    // Add it to a HeaderMap
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, custom_user_agent.parse()?);

    //
    // Use the HeaderMap in the request
    let res = client
        .get("https://api.wordpress.org/plugins/info/1.2/?action=query_plugins&per_page=10")
        .headers(headers)
        .send()
        .await?;

    let status = res.status();
    if !status.is_success() {
        bail!("Can't get the list of plugins. Status: {}", status);
    }

    let data = res.json::<Plugins>().await?;

    // let mut ctx = subversion::client::Context::new()?;
    // let checkout_default_options = CheckoutOptions::default();
    for plugin in data.plugins {
        if plugin.active_installs < 1000 {
            continue;
        }

        let plugin_name = plugin.name;
        let plugin_slug = plugin.slug;
        let checkout_url = format!("https://plugins.svn.wordpress.org/{plugin_slug}/trunk");
        let checkout_path = format!("/home/ubuntu/plugins/{plugin_slug}");
        // ctx.checkout(
        //     checkout_url,
        //     checkout_path,
        //     &checkout_default_options,
        // );

        let output = Command::new("svn")
            .arg("co")
            .arg(checkout_url)
            .arg(checkout_path)
            .output()
            .expect("failed to execute process");

        println!("{plugin_name}");
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    // Print the response
    // println!("Status: {}", res.status());
    // println!("Body:\n{}", res.text().await?);

    Ok(())
}
