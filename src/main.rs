use anyhow::{bail, Context};
use indicatif::{MultiProgress, ProgressBar};
use reqwest::header::USER_AGENT;
use std::{
    collections::HashSet,
    fs,
    process::Command,
    sync::{Arc, Mutex},
};
use wordpress_plugins_agregator::models::{Plugin, Plugins};

const PLUGINS_PATH: &str = "/home/oleg/D/Education/Projects/bugbounty/wordpress/plugins/";
// const PLUGINS_PATH: &str = "/home/ubuntu/plugins/";

fn main() -> anyhow::Result<()> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(20).build()?;

    let page = 1;

    let data = get_data(page)?;

    let total_pages = data.info.pages;
    let total_plugins = data.info.results;

    let multiple_progress = MultiProgress::new();

    let progress_plugins = multiple_progress.add(ProgressBar::new(total_plugins));
    progress_plugins.set_position(0);

    let progress_active_plugins = multiple_progress.add(ProgressBar::new(total_plugins));
    progress_active_plugins.set_position(0);

    let progress_inactive_plugins = multiple_progress.add(ProgressBar::new(total_plugins));
    progress_inactive_plugins.set_position(0);

    let valid_plugins: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::with_capacity(
        (total_plugins + 1000)
            .try_into()
            .context("can't convert u64 to usize")?,
    )));

    let valid_plugins_list: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::with_capacity(
        (total_plugins + 1000)
            .try_into()
            .context("can't convert u64 to usize")?,
    )));

    pool.install(|| {
        rayon::scope(|s| {
            for page in 1..=total_pages {
                let progress_plugins = progress_plugins.clone();
                let progress_active_plugins = progress_active_plugins.clone();
                let progress_inactive_plugins = progress_inactive_plugins.clone();
                let multiple_progress = multiple_progress.clone();
                let valid_plugins = valid_plugins.clone();
                let valid_plugins_list = valid_plugins_list.clone();

                s.spawn(move |_| {
                    if let Ok(data) = get_data(page) {
                        let res = process_plugins(
                            &data.plugins,
                            valid_plugins,
                            valid_plugins_list,
                            progress_plugins,
                            progress_active_plugins,
                            progress_inactive_plugins,
                        );

                        if res.is_err() {
                            let _ = multiple_progress
                                .println(format!("spawn page: {page}: error: {res:?}"));
                        }
                    } else {
                        let _ = multiple_progress.println(format!("can't get_data page: {page}"));
                    }
                });
            }
        });
    });

    let downloaded_plugins = fs::read_dir(PLUGINS_PATH)?;

    for path in downloaded_plugins {
        let path = path?;
        let slug = path.file_name();
        let slug = slug
            .to_str()
            .with_context(|| format!("can't read dir namme: {slug:?}"))?;

        if valid_plugins.lock().unwrap().contains(slug) {
            continue;
        }

        fs::remove_dir_all(path.path())?;
    }

    let valid_plugins_number = valid_plugins.lock().unwrap().len();
    let valid_plugins_list_len = valid_plugins_list.lock().unwrap().len();

    let progress_plugins_position = progress_plugins.position();
    let progress_active_plugins_position = progress_active_plugins.position();
    let progress_inactive_plugins_position = progress_inactive_plugins.position();

    multiple_progress.println(format!("valid_plugins_number mp: {valid_plugins_number}"))?;
    multiple_progress.println(format!("valid_plugins_list_len: {valid_plugins_list_len}"))?;
    multiple_progress.println(format!("total mp: {total_plugins}"))?;
    multiple_progress.println(format!("progress_plugins mp: {progress_plugins_position}"))?;
    multiple_progress.println(format!(
        "progress_active_plugins mp: {progress_active_plugins_position}"
    ))?;
    multiple_progress.println(format!(
        "progress_inactive_plugins mp: {progress_inactive_plugins_position}"
    ))?;

    Ok(())
}

fn get_data(page: u64) -> anyhow::Result<Plugins> {
    let client = reqwest::blocking::Client::new();

    let custom_user_agent =
        "Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0";

    let res = client
        .get(format!(
            "https://api.wordpress.org/plugins/info/1.2/?action=query_plugins&per_page=250&page={}",
            page
        ))
        .header(USER_AGENT, custom_user_agent)
        .send()
        .with_context(|| format!("wordpress api doesn't response. page: {page}"))?;

    let status = res.status();
    if !status.is_success() {
        bail!("Can't get the list of plugins. Status: {}", status);
    }

    let data = res
        .json::<Plugins>()
        .with_context(|| format!("couldn't parse page: {page}"))?;

    Ok(data)
}

fn process_plugins(
    plugins: &Vec<Plugin>,
    valid_plugins: Arc<Mutex<HashSet<String>>>,
    valid_plugins_list: Arc<Mutex<Vec<String>>>,
    progress_plugins: ProgressBar,
    progress_active_plugins: ProgressBar,
    progress_inactive_plugins: ProgressBar,
) -> anyhow::Result<()> {
    for plugin in plugins {
        if plugin.active_installs >= 1000 {
            valid_plugins.lock().unwrap().insert(plugin.slug.clone());
            valid_plugins_list.lock().unwrap().push(plugin.slug.clone());
            download_plugin(&plugin.slug)?;
            progress_active_plugins.inc(1);
        } else {
            progress_inactive_plugins.inc(1);
        }

        progress_plugins.inc(1);
    }

    Ok(())
}

fn download_plugin(plugin_slug: &str) -> anyhow::Result<()> {
    let checkout_url = format!("https://plugins.svn.wordpress.org/{plugin_slug}/trunk");
    let checkout_path = format!("{PLUGINS_PATH}{plugin_slug}");

    Command::new("svn")
        .arg("co")
        .arg(checkout_url)
        .arg(checkout_path)
        .output()
        .with_context(|| format!("problems with svn: {plugin_slug}"))?;

    Ok(())
}
