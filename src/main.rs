use anyhow::bail;
use indicatif::{MultiProgress, ProgressBar};
use reqwest::header::USER_AGENT;
use std::process::Command;
use wordpress_plugins_agregator::models::{Plugin, Plugins};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let page = 1;

    let data = get_data(page).await?;

    let total_pages = data.info.pages;

    let multiple_progress = MultiProgress::new();
    let progress_pages = multiple_progress.add(ProgressBar::new(total_pages));
    progress_pages.set_position(0);
    let progress_page = multiple_progress.add(ProgressBar::new(data.plugins.len().try_into()?));
    progress_page.set_position(0);
    let progress_plugins = multiple_progress.add(ProgressBar::new(data.info.results));
    progress_plugins.set_position(0);

    process_plugins(&data.plugins, &progress_page, &progress_plugins)?;
    progress_pages.inc(1);

    for page in 2..=total_pages {
        let data = get_data(page).await?;
        process_plugins(&data.plugins, &progress_page, &progress_plugins)?;
        progress_pages.inc(1);
    }

    progress_plugins.finish_with_message("downloaded");
    multiple_progress.clear()?;
    Ok(())
}

async fn get_data(page: u64) -> anyhow::Result<Plugins> {
    let client = reqwest::Client::new();

    let custom_user_agent =
        "Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0";

    let res = client
        .get(format!(
            "https://api.wordpress.org/plugins/info/1.2/?action=query_plugins&per_page=100&page={}",
            page
        ))
        .header(USER_AGENT, custom_user_agent)
        .send()
        .await?;

    let status = res.status();
    if !status.is_success() {
        bail!("Can't get the list of plugins. Status: {}", status);
    }

    let data = res.json::<Plugins>().await?;

    Ok(data)
}

fn process_plugins(
    plugins: &Vec<Plugin>,
    progress_page: &ProgressBar,
    progress_plugins: &ProgressBar,
) -> anyhow::Result<()> {
    progress_page.set_length(plugins.len().try_into()?);
    progress_page.set_position(0);
    for plugin in plugins {
        if plugin.active_installs < 1000 {
            continue;
        }

        download_plugin(&plugin.slug)?;
        progress_page.inc(1);
        progress_plugins.inc(1);
    }

    Ok(())
}

fn download_plugin(plugin_slug: &str) -> anyhow::Result<()> {
    let checkout_url = format!("https://plugins.svn.wordpress.org/{plugin_slug}/trunk");
    let checkout_path = format!("/home/ubuntu/plugins/{plugin_slug}");

    Command::new("svn")
        .arg("co")
        .arg(checkout_url)
        .arg(checkout_path)
        .output()?;

    Ok(())
}
