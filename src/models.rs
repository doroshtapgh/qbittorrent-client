use std::collections::HashMap;

use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct AppBuildInfo {
    pub qt: String,
    pub libtorrent: String,
    pub boost: String,
    pub openssl: String,
    pub bitness: usize
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppPreferences {
    pub locale: Option<String>,
    pub create_subfolder_enabled: Option<bool>,
    pub start_paused_enabled: Option<bool>,
    // not implemented yet (by qBittorrent): pub auto_delete_mode: isize(?),
    pub preallocate_all: Option<bool>,
    pub incomplete_files_ext: Option<bool>,
    pub auto_tmm_enabled: Option<bool>,
    pub torrent_changed_tmm_enabled: Option<bool>,
    pub save_path_changed_tmm_enabled: Option<bool>,
    pub category_changed_tmm_enabled: Option<bool>,
    pub save_path: Option<String>,
    pub temp_path_enabled: Option<bool>,
    pub temp_path: Option<String>,
    pub scan_dirs: Option<HashMap<String, IntOrString>>,
    pub export_dir: Option<String>,
    pub export_dir_fin: Option<String>,
    pub mail_notification_enabled: Option<bool>,
    pub mail_notification_sender: Option<String>,
    pub mail_notification_email: Option<String>,
    pub mail_notification_smtp: Option<String>,
    pub mail_notification_ssl_enabled: Option<bool>,
    pub mail_notification_auth_enabled: Option<bool>,
    pub mail_notification_username: Option<String>,
    pub mail_notification_password: Option<String>,
    pub autorun_enabled: Option<bool>,
    pub autorun_program: Option<String>,
    pub queueing_enabled: Option<bool>,
    pub max_active_downloads: Option<usize>,
    pub max_active_torrents: Option<usize>,
    pub max_active_uploads: Option<usize>,
    pub dont_count_slow_torrents: Option<bool>,
    pub slow_torrent_dl_rate_threshold: Option<usize>,
    pub slow_torrent_ul_rate_threshold: Option<usize>,
    pub slow_torrent_inactive_timer: Option<usize>,
    pub max_ratio_enabled: Option<bool>,
    pub max_ratio: Option<f64>,
    pub max_ratio_act: Option<usize>,
    pub listen_port: Option<u16>,
    pub upnp: Option<bool>,
    pub random_port: Option<bool>,
    pub dl_limit: Option<isize>,
    pub up_limit: Option<isize>,
    pub max_connec: Option<usize>,
    pub max_connec_per_torrent: Option<usize>,
    pub max_uploads: Option<usize>,
    pub max_uploads_per_torrent: Option<usize>,
    pub stop_tracker_timeout: Option<usize>,
    pub enable_piece_extent_affinity: Option<bool>,
    pub bittorrent_protocol: Option<usize>,
    pub limit_utp_rate: Option<bool>,
    pub limit_tcp_overhead: Option<bool>,
    pub limit_lan_peers: Option<bool>,
    pub alt_dl_limit: Option<usize>,
    pub alt_up_limit: Option<usize>,
    pub scheduler_enabled: Option<bool>,
    pub schedule_from_hour: Option<usize>,
    pub schedule_from_min: Option<usize>,
    pub schedule_to_hour: Option<usize>,
    pub schedule_to_min: Option<usize>,
    pub scheduler_days: Option<usize>,
    pub dht: Option<bool>,
    pub pex: Option<bool>,
    pub lsd: Option<bool>,
    pub encryption: Option<usize>,
    pub anonymous_mode: Option<bool>,
    pub proxy_type: Option<IntOrString>,
    pub proxy_ip: Option<String>,
    pub proxy_port: Option<u16>,
    pub proxy_peer_connections: Option<bool>,
    pub proxy_auth_enabled: Option<bool>,
    pub proxy_username: Option<String>,
    pub proxy_password: Option<String>,
    pub proxy_torrents_only: Option<bool>,
    pub ip_filter_enabled: Option<bool>,
    pub ip_filter_path: Option<String>,
    pub ip_filter_trackers: Option<bool>,
    pub web_ui_domain_list: Option<String>,
    pub web_ui_address: Option<String>,
    pub web_ui_port: Option<u16>,
    pub web_ui_upnp: Option<bool>,
    pub web_ui_username: Option<String>,
    pub web_ui_password: Option<String>,
    pub web_ui_csrf_protection_enabled: Option<bool>,
    pub web_ui_clickjacking_protection_enabled: Option<bool>,
    pub web_ui_secure_cookie_enabled: Option<bool>,
    pub web_ui_max_auth_fail_count: Option<usize>,
    pub web_ui_ban_duration: Option<usize>,
    pub web_ui_session_timeout: Option<usize>,
    pub web_ui_host_header_validation_enabled: Option<bool>,
    pub bypass_local_auth: Option<bool>,
    pub bypass_auth_subnet_whitelist_enabled: Option<bool>,
    pub bypass_auth_subnet_whitelist: Option<String>,
    pub alternative_webui_enabled: Option<bool>,
    pub alternative_webui_path: Option<String>,
    pub use_https: Option<bool>,
    pub ssl_key: Option<String>,
    pub ssl_cert: Option<String>,
    pub web_ui_https_key_path: Option<String>,
    pub web_ui_https_cert_path: Option<String>,
    pub dyndns_enabled: Option<bool>,
    pub dyndns_service: Option<usize>,
    pub dyndns_username: Option<String>,
    pub dyndns_password: Option<String>,
    pub dyndns_domain: Option<String>,
    pub rss_refresh_interval: Option<usize>,
    pub rss_max_articles_per_feed: Option<usize>,
    pub rss_processing_enabled: Option<bool>,
    pub rss_auto_downloading_enabled: Option<bool>,
    pub rss_download_repack_proper_episodes: Option<bool>,
    pub rss_smart_episode_filters: Option<String>,
    pub add_trackers_enabled: Option<bool>,
    pub add_trackers: Option<String>,
    pub web_ui_use_custom_http_headers_enabled: Option<bool>,
    pub web_ui_custom_http_headers: Option<String>,
    pub max_seeding_time_enabled: Option<bool>,
    pub max_seeding_time: Option<isize>,
    // not implemented yet (by qBittorrent): pub announce_ip: String,
    pub announce_to_all_tiers: Option<bool>,
    pub announce_to_all_trackers: Option<bool>,
    pub async_io_threads: Option<usize>,
    #[serde(rename = "banned_IPs")]
    pub banned_ips: Option<String>,
    pub checking_memory_use: Option<usize>,
    pub current_interface_address: Option<String>,
    pub current_network_interface: Option<String>,
    pub disk_cache: Option<isize>,
    pub disk_cache_ttl: Option<usize>,
    pub embedded_tracker_port: Option<u16>,
    pub enable_coalesce_read_write: Option<bool>,
    pub enable_embedded_tracker: Option<bool>,
    pub enable_multi_connections_from_same_ip: Option<bool>,
    pub enable_os_cache: Option<bool>,
    pub enable_upload_suggestions: Option<bool>,
    pub file_pool_size: Option<usize>,
    pub outgoing_ports_max: Option<usize>,
    pub outgoing_ports_min: Option<usize>,
    pub recheck_completed_torrents: Option<bool>,
    pub resolve_peer_countries: Option<bool>,
    pub save_resume_data_interval: Option<usize>,
    pub send_buffer_low_watermark: Option<usize>,
    pub send_buffer_watermark: Option<usize>,
    pub send_buffer_watermark_factor: Option<usize>,
    pub socket_backlog_size: Option<usize>,
    pub upload_choking_algorithm: Option<usize>,
    pub upload_slots_behavior: Option<usize>,
    pub upnp_lease_duration: Option<usize>,
    pub utp_tcp_mixed_mode: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum IntOrString {
    Int(i64),
    Str(String)
}

impl<'de> Deserialize<'de> for IntOrString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;
        match v {
            Value::Number(num) => {
                if let Some(i) = num.as_i64() {
                    Ok(IntOrString::Int(i))
                } else {
                    Err(serde::de::Error::custom("Expected an integer"))
                }
            }
            Value::String(s) => Ok(IntOrString::Str(s)),
            _ => Err(serde::de::Error::custom("Expected a string or an integer")),
        }
    }
}

pub struct JsonObject(pub(crate) Value);

impl TryFrom<Value> for JsonObject {
    type Error = std::io::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Object(..) = value {
            Ok(JsonObject(value))
        } else {
            Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, ""))
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Log {
    pub id: usize,
    pub message: String,
    pub timestamp: usize,
    #[serde(rename = "type")]
    pub log_type: usize
}

#[derive(Debug, Clone)]
pub struct LogParams {
    pub normal: bool,
    pub info: bool,
    pub warning: bool,
    pub critical: bool,
    pub last_known_id: isize
}

impl Default for LogParams {
    fn default() -> Self {
        LogParams {
            normal: true,
            info: true,
            warning: true,
            critical: true,
            last_known_id: -1
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PeerLog {
    pub id: usize,
    pub ip: String,
    pub timestamp: usize,
    pub blocked: bool,
    pub reason: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct SyncMainData {
    pub rid: usize,
    pub full_update: bool,
    pub torrents: HashMap<String, Torrent>,
    pub torrents_removed: Vec<String>,
    pub categories: HashMap<String, Category>,
    pub categories_removed: Vec<String>,
    pub tags: Vec<String>,
    pub tags_removed: Vec<String>,
    pub server_state: GlobalTransferInfo
}

#[derive(Debug, Clone, Deserialize)]
pub struct GlobalTransferInfo {
    pub dl_info_speed: usize,
    pub dl_info_data: usize,
    pub up_info_speed: usize,
    pub up_info_data: usize,
    pub dl_rate_limit: usize,
    pub up_rate_limit: usize,
    pub dht_nodes: usize,
    pub connection_status: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct Category {
    pub name: String,
    #[serde(rename = "savePath")]
    pub save_path: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    pub added_on: usize,
    pub amount_left: usize,
    pub auto_tmm: bool,
    pub availability: f64,
    pub category: String,
    pub completed: usize,
    pub completion_on: usize,
    pub content_path: String,
    pub dl_limit: isize,
    pub dlspeed: usize,
    pub downloaded: usize,
    pub downloaded_session: usize,
    pub eta: usize,
    pub f_l_piece_prio: bool,
    pub force_start: bool,
    pub hash: String,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub last_activity: usize,
    pub magnet_uri: String,
    pub max_ratio: f64,
    pub max_seeding_time: usize,
    pub name: String,
    pub num_complete: usize,
    pub num_incomplete: usize,
    pub num_leechs: usize,
    pub num_seeds: usize,
    pub priority: isize,
    pub progress: f64,
    pub ratio: f64,
    // not implemented yet (by qBittorrent): pub ratio_limit: f64,
    pub save_path: String,
    pub seeding_time: usize,
    // not implemented yet (by qBittorrent): pub seeding_time_limit: isize,
    pub seen_complete: usize,
    pub seq_dl: bool,
    pub size: usize,
    pub state: String,
    pub super_seeding: bool,
    pub tags: String,
    pub time_active: usize,
    pub tracker: String,
    pub up_limit: isize,
    pub uploaded: usize,
    pub uploaded_session: usize,
    pub upspeed: usize
}

#[derive(Debug, Clone)]
pub struct TorrentListParams {
    pub filter: TorrentListFilter,
    pub category: String,
    pub tag: String,
    pub sort: Option<String>,
    pub reverse: bool,
    pub limit: Option<usize>,
    pub offset: Option<isize>,
    pub hashes: Option<String>
}

impl Default for TorrentListParams {
    fn default() -> Self {
        TorrentListParams {
            filter: TorrentListFilter::All,
            category: String::new(),
            tag: String::new(),
            sort: None,
            reverse: false,
            limit: None,
            offset: None,
            hashes: None
        }
    }
}

#[derive(Debug, Clone)]
pub enum TorrentListFilter {
    All,
    Downloading,
    Seeding,
    Completed,
    Paused,
    Active,
    Inactive,
    Resumed,
    Stalled,
    StalledUploading,
    StalledDownloading,
    Errored
}

impl ToString for TorrentListFilter {
    fn to_string(&self) -> String {
        match *self {
            TorrentListFilter::All => String::from("all"),
            TorrentListFilter::Downloading => String::from("downloading"),
            TorrentListFilter::Seeding => String::from("seeding"),
            TorrentListFilter::Completed => String::from("completed"),
            TorrentListFilter::Paused => String::from("paused"),
            TorrentListFilter::Active => String::from("active"),
            TorrentListFilter::Inactive => String::from("inactive"),
            TorrentListFilter::Resumed => String::from("resumed"),
            TorrentListFilter::Stalled => String::from("stalled"),
            TorrentListFilter::StalledUploading => String::from("stalled_uploading"),
            TorrentListFilter::StalledDownloading => String::from("stalled_downloading"),
            TorrentListFilter::Errored => String::from("errored")
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TorrentGenericProperties {
    pub save_path: String,
    pub creation_date: usize,
    pub piece_size: usize,
    pub comment: String,
    pub total_wasted: usize,
    pub total_uploaded: usize,
    pub total_uploaded_session: usize,
    pub total_downloaded: usize,
    pub total_downloaded_session: usize,
    pub up_limit: isize,
    pub dl_limit: isize,
    pub time_elapsed: usize,
    pub seeding_time: usize,
    pub nb_connections: usize,
    pub nb_connections_limit: usize,
    pub share_ratio: f64,
    pub addition_date: usize,
    pub completion_date: usize,
    pub created_by: String,
    pub dl_speed_avg: usize,
    pub dl_speed: usize,
    pub eta: usize,
    pub last_seen: usize,
    pub peers: usize,
    pub peers_total: usize,
    pub pieces_have: usize,
    pub pieces_num: usize,
    pub reannounce: usize,
    pub seeds: usize,
    pub seeds_total: usize,
    pub total_size: usize,
    pub up_speed_avg: usize,
    pub up_speed: usize,
    #[serde(rename = "isPrivate")]
    pub is_private: bool
}

#[derive(Debug, Clone, Deserialize)]
pub struct TorrentTracker {
    pub url: String,
    pub status: usize,
    pub tier: isize,
    pub num_peers: usize,
    pub num_seeds: usize,
    pub num_leeches: usize,
    pub num_downloaded: usize,
    pub msg: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct TorrentWebSeed {
    pub url: String
}
