use reqwest::{Client, Url};
use tokio::sync::RwLock;

use crate::{
    models::{
        AppBuildInfo, AppPreferences, GlobalTransferInfo, JsonObject, Log, LogParams, PeerLog, SyncMainData, Torrent, TorrentGenericProperties, TorrentListParams, TorrentTracker, TorrentWebSeed
    },
    QBittorrentError
};

pub struct QBittorrentClient {
    http_client: Client,
    base_url: RwLock<Url>
}

impl QBittorrentClient {
    pub async fn new<S: ToString>(url: S) -> Result<Self, QBittorrentError> {
        let http_client = Client::builder()
            .cookie_store(true)
            .build()?;

        let base_url = Url::parse(&url.to_string())?;
        let base_url = RwLock::new(base_url);

        Ok(QBittorrentClient {
            http_client,
            base_url
        })
    }

    async fn build_url(&self, endpoint: &str) -> Result<Url, QBittorrentError> {
        let base_url = self.base_url.read().await;
        Ok(base_url.join(endpoint)?)
    }

    pub async fn login<S: ToString>(&self, username: S, password: S) -> Result<(), QBittorrentError> {
        let base_url = self.base_url.read().await;
        let login_url = base_url.join("/api/v2/auth/login")?;
        
        let res = self.http_client.post(login_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Referer", base_url.to_string())
            .body(format!("username={}&password={}", username.to_string(), password.to_string()))
            .send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(QBittorrentError::AuthFailed)
        }
    }

    pub async fn logout(&self) -> Result<(), QBittorrentError> {
        let logout_url = self.build_url("/api/v2/auth/logout").await?;
        self.http_client.post(logout_url).send().await?;

        Ok(())
    }

    pub async fn application_version(&self) -> Result<String, QBittorrentError> {
        let app_ver_url = self.build_url("/api/v2/app/version").await?;
        let res = self.http_client.get(app_ver_url).send().await?;

        Ok(res.text().await?)
    }

    pub async fn api_version(&self) -> Result<String, QBittorrentError> {
        let api_ver_url = self.build_url("/api/v2/app/webapiVersion").await?;
        let res = self.http_client.get(api_ver_url).send().await?;

        Ok(res.text().await?)
    }

    pub async fn build_info(&self) -> Result<AppBuildInfo, QBittorrentError> {
        let build_info_url = self.build_url("/api/v2/app/buildInfo").await?;
        let res = self.http_client.get(build_info_url).send().await?;

        Ok(res.json::<AppBuildInfo>().await?)
    }

    pub async fn shutdown(&self) -> Result<(), QBittorrentError> {
        let shutdown_url = self.build_url("/api/v2/app/shutdown").await?;
        self.http_client.post(shutdown_url).send().await?;

        Ok(())
    }

    pub async fn preferences(&self) -> Result<AppPreferences, QBittorrentError> {
        let preferences_url = self.build_url("/api/v2/app/preferences").await?;
        let res = self.http_client.get(preferences_url).send().await?;

        if res.status().as_u16() == 400 {
            Err(QBittorrentError::BadRequest)
        } else {
            Ok(res.json::<AppPreferences>().await?)
        }
    }

    pub async fn set_preferences(&self, obj: JsonObject) -> Result<(), QBittorrentError> {
        let set_pref_url = self.build_url("/api/v2/app/setPreferences").await?;

        let res = self.http_client.post(set_pref_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(format!("json={}", obj.0.to_string()))
            .send().await?;

        if res.status().as_u16() == 400 {
            Err(QBittorrentError::BadRequest)
        } else {
            Ok(())
        }
    }

    pub async fn default_save_path(&self) -> Result<String, QBittorrentError> {
        let def_save_path_url = self.build_url("/api/v2/app/defaultSavePath").await?;
        let res = self.http_client.get(def_save_path_url).send().await?;

        Ok(res.text().await?)
    }

    pub async fn logs(&self, params: LogParams) -> Result<Vec<Log>, QBittorrentError> {
        let mut log_url = self.build_url("/api/v2/log/main").await?;
        
        let bool_to_str = |b: bool| -> &str {
            if b { "true" } else { "false" }
        };

        log_url.query_pairs_mut()
            .append_pair("normal", bool_to_str(params.normal))
            .append_pair("info", bool_to_str(params.info))
            .append_pair("warning", bool_to_str(params.warning))
            .append_pair("critical", bool_to_str(params.critical))
            .append_pair("last_known_id", &params.last_known_id.to_string());

        let res = self.http_client.get(log_url).send().await?;
        Ok(res.json::<Vec<Log>>().await?)
    }

    // Exclude messages with "message id" <= last_known_id (default: -1)
    pub async fn peer_logs(&self, last_known_id: Option<usize>) -> Result<Vec<PeerLog>, QBittorrentError> {
        let mut peers_url = self.build_url("/api/v2/log/peers").await?;

        let lki = if last_known_id.is_some() {
            last_known_id.unwrap().to_string()
        } else {
            "-1".to_string()
        };

        peers_url.query_pairs_mut()
            .append_pair("last_known_id", &lki);

        let res = self.http_client.get(peers_url).send().await?;

        Ok(res.json::<Vec<PeerLog>>().await?)
    }

    pub async fn sync_main_data(&self, response_id: Option<usize>) -> Result<SyncMainData, QBittorrentError> {
        let mut sync_url = self.build_url("/api/v2/sync/maindata").await?;
        sync_url.query_pairs_mut()
            .append_pair("rid", &response_id.unwrap_or(0).to_string());

        let res = self.http_client.get(sync_url).send().await?;

        Ok(res.json::<SyncMainData>().await?)
    }

    // is not implemented by qBittorrent yet
    // pub async fn sync_peers_data<S: ToString>(&self, hash: S, rid: Option<usize>) -> Result<, QBittorrentError> {}

    pub async fn global_transfer_info(&self) -> Result<GlobalTransferInfo, QBittorrentError> {
        let info_url = self.build_url("/api/v2/transfer/info").await?;
        let res = self.http_client.get(info_url).send().await?;

        Ok(res.json::<GlobalTransferInfo>().await?)
    }

    pub async fn alternative_speed_limits_enabled(&self) -> Result<bool, QBittorrentError> {
        let limits_url = self.build_url("/api/v2/transfer/speedLimitsMode").await?;

        let res = self.http_client.get(limits_url).send().await?;
        let text = res.text().await?;

        if text == "1" {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn toggle_alternative_speed_limits(&self) -> Result<(), QBittorrentError> {
        let toggle_url = self.build_url("/api/v2/transfer/toggleSpeedLimitsMode").await?;
        self.http_client.post(toggle_url).send().await?;

        Ok(())
    }

    // The response is the value of current global download speed limit in bytes/second; this value will be zero if no limit is applied.
    pub async fn download_limit(&self) -> Result<usize, QBittorrentError> {
        let limit_url = self.build_url("/api/v2/transfer/downloadLimit").await?;
        let res = self.http_client.get(limit_url).send().await?;
        let text = res.text().await?;

        Ok(text.parse::<usize>()?)
    }

    pub async fn set_download_limit(&self, limit: usize) -> Result<(), QBittorrentError> {
        let mut limit_url = self.build_url("/api/v2/transfer/setDownloadLimit").await?;
        limit_url.query_pairs_mut()
            .append_pair("limit", &limit.to_string());

        self.http_client.post(limit_url).send().await?;

        Ok(())
    }

    // The response is the value of current global upload speed limit in bytes/second; this value will be zero if no limit is applied.
    pub async fn upload_limit(&self) -> Result<usize, QBittorrentError> {
        let limit_url = self.build_url("/api/v2/transfer/uploadLimit").await?;
        let res = self.http_client.get(limit_url).send().await?;
        let text = res.text().await?;

        Ok(text.parse::<usize>()?)
    }

    pub async fn set_upload_limit(&self, limit: usize) -> Result<(), QBittorrentError> {
        let mut limit_url = self.build_url("/api/v2/transfer/setUploadLimit").await?;
        limit_url.query_pairs_mut()
            .append_pair("limit", &limit.to_string());

        self.http_client.post(limit_url).send().await?;

        Ok(())
    }

    // The peer to ban, or multiple peers separated by a pipe |. Each peer is a colon-separated host:port
    pub async fn ban_peers<S: ToString>(&self, peers: S) -> Result<(), QBittorrentError> {
        let mut ban_url = self.build_url("/api/v2/transfer/banPeers").await?;
        ban_url.query_pairs_mut()
            .append_pair("peers", &peers.to_string());

        let res = self.http_client.post(ban_url).send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(QBittorrentError::BadRequest)
        }
    }

    pub async fn torrent_list(&self, params: TorrentListParams) -> Result<Vec<Torrent>, QBittorrentError> {
        let mut list_url = self.build_url("/api/v2/torrents/info").await?;
        let mut pairs = list_url.query_pairs_mut();
        pairs.append_pair("filter", &params.filter.to_string());
        pairs.append_pair("category", &urlencoding::encode(&params.category));
        pairs.append_pair("tag", &urlencoding::encode(&params.tag));
        pairs.append_pair("reverse", if params.reverse { "true" } else { "false" });
        
        if let Some(sort) = params.sort {
            pairs.append_pair("sort", &sort);
        }

        if let Some(limit) = params.limit {
            pairs.append_pair("limit", &limit.to_string());
        }

        if let Some(offset) = params.offset {
            pairs.append_pair("offset", &offset.to_string());
        }

        if let Some(hashes) = params.hashes {
            pairs.append_pair("hashes", &hashes);
        }

        drop(pairs);

        let res = self.http_client.get(list_url).send().await?;

        if res.status().is_success() {
            Ok(res.json::<Vec<Torrent>>().await?)
        } else {
            Err(QBittorrentError::BadRequest)
        }
    }

    pub async fn torrent_generic_properties<S: ToString>(&self, hash: S) -> Result<TorrentGenericProperties, QBittorrentError> {
        let mut props_url = self.build_url("/api/v2/torrents/properties").await?;
        props_url.query_pairs_mut().append_pair("hash", &hash.to_string());

        let res = self.http_client.get(props_url).send().await?;

        if res.status().is_success() {
            Ok(res.json::<TorrentGenericProperties>().await?)
        } else {
            Err(QBittorrentError::BadInput("torrent hash was not found".to_string()))
        }
    }

    pub async fn torrent_trackers<S: ToString>(&self, hash: S) -> Result<Vec<TorrentTracker>, QBittorrentError> {
        let mut trackers_url = self.build_url("/api/v2/torrents/trackers").await?;
        trackers_url.query_pairs_mut().append_pair("hash", &hash.to_string());

        let res = self.http_client.get(trackers_url).send().await?;

        if res.status().is_success() {
            Ok(res.json::<Vec<TorrentTracker>>().await?)
        } else {
            Err(QBittorrentError::BadInput("torrent hash was not found".to_string()))
        }
    }

    pub async fn torrent_web_seeds<S: ToString>(&self, hash: S) -> Result<Vec<TorrentWebSeed>, QBittorrentError> {
        let mut seeds_url = self.build_url("/api/v2/torrents/webseeds").await?;
        seeds_url.query_pairs_mut().append_pair("hash", &hash.to_string());

        let res = self.http_client.get(seeds_url).send().await?;

        if res.status().is_success() {
            Ok(res.json::<Vec<TorrentWebSeed>>().await?)
        } else {
            Err(QBittorrentError::BadInput("torrent hash was not found".to_string()))
        }
    }

    // hashes: The hashes of the torrents you want to pause. hashes can contain multiple hashes separated by |, to pause multiple torrents, or set to all, to pause all torrents.
    pub async fn torrent_pause<S: ToString>(&self, hashes: S) -> Result<(), QBittorrentError> {
        let mut pause_url = self.build_url("/api/v2/torrents/pause").await?;
        pause_url.query_pairs_mut().append_pair("hashes", &hashes.to_string());

        let res = self.http_client.post(pause_url).send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(QBittorrentError::BadRequest)
        }
    }

    // hashes: The hashes of the torrents you want to pause. hashes can contain multiple hashes separated by |, to pause multiple torrents, or set to all, to pause all torrents.
    pub async fn torrent_resume<S: ToString>(&self, hashes: S) -> Result<(), QBittorrentError> {
        let mut resume_url = self.build_url("/api/v2/torrents/resume").await?;
        resume_url.query_pairs_mut().append_pair("hashes", &hashes.to_string());

        let res = self.http_client.post(resume_url).send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(QBittorrentError::BadRequest)
        }
    }

    // hashes: The hashes of the torrents you want to delete. hashes can contain multiple hashes separated by |, to delete multiple torrents, or set to all, to delete all torrents.
    // delete_files: If set to true, the downloaded data will also be deleted, otherwise has no effect.
    pub async fn torrent_delete<S: ToString>(&self, hashes: S, delete_files: bool) -> Result<(), QBittorrentError> {
        let mut delete_url = self.build_url("/api/v2/torrents/delete").await?;
        delete_url.query_pairs_mut()
            .append_pair("hashes", &hashes.to_string())
            .append_pair("deleteFiles", if delete_files { "true" } else { "false" });

        let res = self.http_client.post(delete_url).send().await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(QBittorrentError::BadRequest)
        }
    }
}

