use reqwest::{Client, Url};
use tokio::sync::RwLock;

use crate::{
    models::{
        AppBuildInfo, AppPreferences, GlobalTransferInfo, JsonObject, Log, LogParams, PeerLog, SyncMainData
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
}

