use crate::tng::app::App;
use crate::tng::cache_info::CacheInfo;
use crate::tng::download::Download;
use crate::tng::file::File;
use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use chrono::Utc;
use isopy_lib::tng::{Context, DownloadOptions, FileNameParts};
use reqwest::header::{ACCEPT, USER_AGENT};
use reqwest::Client;
use reqwest::Url as ReqwestUrl;
use std::collections::HashMap;
use std::fs::{create_dir_all, write};
use std::path::PathBuf;
use url::Url;

const CACHE_FILE_NAME: &str = "cache.json";

pub(crate) struct AppContext<'a> {
    #[allow(unused)]
    app: &'a App,
    cache_dir: PathBuf,
}

impl<'a> AppContext<'a> {
    pub(crate) fn new(app: &'a App, name: &str) -> Self {
        Self {
            app,
            cache_dir: app.cache_dir().join(name),
        }
    }

    fn make_unique_path(&self, url: &Url) -> Result<PathBuf> {
        let file_name_parts = FileNameParts::from_url_safe(url)?;
        for i in 0.. {
            let file_name = if i == 0 {
                format!("{}{}", file_name_parts.prefix, file_name_parts.suffix)
            } else {
                format!(
                    "{}-{i:05}{}",
                    file_name_parts.prefix, file_name_parts.suffix
                )
            };
            let p = self.cache_dir.join(file_name);
            if !p.is_file() {
                return Ok(p);
            }
        }
        unreachable!();
    }
}

#[async_trait]
impl<'a> Context for AppContext<'a> {
    async fn download(&self, url: &Url, options: &DownloadOptions) -> Result<PathBuf> {
        let cache_info = CacheInfo::load(self.cache_dir.join(CACHE_FILE_NAME))?;
        let downloads = cache_info
            .manifest
            .downloads
            .iter()
            .map(|f| (f.url.clone(), f))
            .collect::<HashMap<_, _>>();

        if let Some(download) = downloads.get(url) {
            if download.files.is_empty() {
                bail!("URL {} has no downloaded files", url.as_str());
            }

            let mut files = download.files.iter().collect::<Vec<_>>();
            files.sort_by_cached_key(|f| f.downloaded_at);
            files.reverse();

            let file = files.first().expect("Must be at least one file");
            let path = self.cache_dir.join(&file.file_name);
            if !path.is_file() {
                bail!("File {} is missing from cache", path.display());
            }
            return Ok(path);
        }

        let path = self.make_unique_path(url)?;
        let downloaded_at = Utc::now();

        create_dir_all(
            path.parent().ok_or_else(|| {
                anyhow!("Cannot get parent directory from path {}", path.display())
            })?,
        )?;

        println!("Downloading {url}");

        let mut request = Client::new()
            .get(ReqwestUrl::parse(url.as_str())?)
            .header(USER_AGENT, "isopy-tng");

        if let Some(accept) = &options.accept {
            request = request.header(ACCEPT, accept.as_str())
        };

        let response = request.send().await?;
        response.error_for_status_ref()?;

        let data = response.bytes().await?;
        write(&path, data)?;

        {
            let mut cache_info = CacheInfo::load(self.cache_dir.join(CACHE_FILE_NAME))?;
            let file_name = path
                .file_name()
                .expect("Must have file name")
                .to_str()
                .expect("Must be valid string")
                .to_string();
            let files = vec![File {
                file_name,
                downloaded_at,
            }];
            cache_info.manifest.downloads.push(Download {
                url: url.clone(),
                files,
            });
            cache_info.save()?;
        }

        println!("Downloaded {url}");

        return Ok(path);
    }
}
