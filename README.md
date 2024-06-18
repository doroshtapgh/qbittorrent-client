# qbittorrent-client
qbittorrent-client is a wrapper of qBittorrent WebAPI(https://github.com/qbittorrent/qBittorrent/wiki/WebUI-API-(qBittorrent-4.1)), written in Rust.

# Implemented:
1. Authentication
    - [x] Login
    - [x] Logout
2. Application
    - [x] Get application version
    - [x] Get API version
    - [x] Get build info
    - [x] Shutdown application
    - [x] Get application preferences
    - [x] Set application preferences
    - [x] Get default save path
3. Log
    - [x] Get log
    - [x] Get peer log
4. Sync
    - [ ] Get main data
    - [ ] Get torrent peers data
5. Transfer info
    - [ ] Get global transfer info
    - [ ] Get alternative speed limits state
    - [ ] Toggle alternative speed limits
    - [ ] Get global download limit
    - [ ] Set global download limit
    - [ ] Get global upload limit
    - [ ] Set global upload limit
    - [ ] Ban peers
6. Torrent management
    - [ ] Get torrent list
    - [ ] Get torrent generic properties
    - [ ] Get torrent trackers
    - [ ] Get torrent web seeds
    - [ ] Get torrent contents
    - [ ] Get torrent pieces' states
    - [ ] Get torrent pieces' hashes
    - [ ] Pause torrents
    - [ ] Resume torrents
    - [ ] Delete torrents
    - [ ] Recheck torrents
    - [ ] Reannounce torrents
    - [ ] Edit trackers
    - [ ] Remove trackers
    - [ ] Add peers
    - [ ] Add new torrent
    - [ ] Add trackers to torrent
    - [ ] Increase torrent priority
    - [ ] Decrease torrent priority
    - [ ] Maximal torrent priority
    - [ ] Minimal torrent priority
    - [ ] Set file priority
    - [ ] Get torrent download limit
    - [ ] Set torrent download limit
    - [ ] Set torrent share limit
    - [ ] Get torrent upload limit
    - [ ] Set torrent upload limit
    - [ ] Set torrent location
    - [ ] Set torrent name
    - [ ] Set torrent category
    - [ ] Get all categories
    - [ ] Add new category
    - [ ] Edit category
    - [ ] Remove categories
    - [ ] Add torrent tags
    - [ ] Remove torrent tags
    - [ ] Get all tags
    - [ ] Create tags
    - [ ] Delete tags
    - [ ] Set automatic torrent management
    - [ ] Toggle sequential download
    - [ ] Set first/last piece priority
    - [ ] Set force start
    - [ ] Set super seeding
    - [ ] Rename file
    - [ ] Rename folder
7. RSS
    - [ ] Add folder
    - [ ] Add feed
    - [ ] Remove item
    - [ ] Move item
    - [ ] Get all items
    - [ ] Mark as read
    - [ ] Refresh item
    - [ ] Set auto-downloading rule
    - [ ] Rename auto-downloading rule
    - [ ] Remove auto-downloading rule
    - [ ] Get all auto-downloading rules
    - [ ] Get all articles matching a rule
8. Search
    - [ ] Start search
    - [ ] Stop search
    - [ ] Get search status
    - [ ] Get search results
    - [ ] Delete search
    - [ ] Get search plugins
    - [ ] Install search plugin
    - [ ] Uninstall search plugin
    - [ ] Enable search plugin
    - [ ] Update search plugins