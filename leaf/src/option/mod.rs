use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use lazy_static::lazy_static;

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "ios")]
pub use ios::*;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "android"))]
mod unix;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "android"))]
pub use unix::*;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::*;

fn get_env_var<T>(key: &str, default: T) -> T
where
    T: FromStr,
{
    if let Ok(v) = env::var(key) {
        if let Ok(v) = v.parse::<T>() {
            return v;
        }
    }
    default
}

lazy_static! {
    /// Uplink timeout after downlink EOF.
    pub static ref TCP_UPLINK_TIMEOUT: u64 = {
        get_env_var("TCP_UPLINK_TIMEOUT", 2)
    };

    /// Downlink timeout after uplink EOF.
    pub static ref TCP_DOWNLINK_TIMEOUT: u64 = {
        get_env_var("TCP_DOWNLINK_TIMEOUT", 4)
    };

    /// Buffer size for uplink and downlink connections, in KB.
    pub static ref LINK_BUFFER_SIZE: usize = {
        get_env_var("LINK_BUFFER_SIZE", 2)
    };

    /// Maximum outbound dial concurrency.
    pub static ref OUTBOUND_DIAL_CONCURRENCY: usize = {
        get_env_var("OUTBOUND_DIAL_CONCURRENCY", 1)
    };

    pub static ref ASSET_LOCATION: String = {
        let mut file = std::env::current_exe().unwrap();
        file.pop();
        get_env_var("ASSET_LOCATION", file.to_str().unwrap().to_string())
    };

    pub static ref CACHE_LOCATION: String = {
        get_env_var("CACHE_LOCATION", "".to_string())
    };

    pub static ref API_LISTEN: String = {
        get_env_var("API_LISTEN", "".to_string())
    };

    pub static ref ENABLE_IPV6: bool = {
        get_env_var("ENABLE_IPV6", false)
    };

    pub static ref PREFER_IPV6: bool = {
        get_env_var("PREFER_IPV6", false)
    };

    pub static ref UNSPECIFIED_BIND_ADDR: SocketAddr = {
        let default =  if *ENABLE_IPV6 {
            "[::]:0".to_string().parse().unwrap()
        } else {
            "0.0.0.0:0".to_string().parse().unwrap()
        };
        get_env_var("UNSPECIFIED_BIND_ADDR", default)
    };

    pub static ref OUTBOUND_INTERFACE: String = {
        get_env_var("OUTBOUND_INTERFACE", "".to_string())
    };

    pub static ref GATEWAY_MODE: bool = {
        get_env_var("GATEWAY_MODE", false)
    };
}

/// UDP session timeout. A UDP session shall be terminated if there are no
/// activities in this period. The timeouts are observed only when a check
/// is happened.
pub static UDP_SESSION_TIMEOUT: u64 = 30;

/// UDP session timeout check interval. The interval to check for UDP session
/// timeouts.
pub static UDP_SESSION_TIMEOUT_CHECK_INTERVAL: u64 = 10;

/// Maximum retries for a specific DNS query for the built-in DNS client.
pub static MAX_DNS_RETRIES: usize = 4;

/// Timeout for a DNS query for the built-in DNS client.
pub static DNS_TIMEOUT: u64 = 4;

pub static DEFAULT_TUN_NAME: &str = "utun233";
pub static DEFAULT_TUN_IPV4_ADDR: &str = "240.255.0.2";
pub static DEFAULT_TUN_IPV4_GW: &str = "240.255.0.1";
pub static DEFAULT_TUN_IPV4_MASK: &str = "255.255.255.0";
pub static DEFAULT_TUN_IPV6_ADDR: &str = "2001:2::2";
pub static DEFAULT_TUN_IPV6_GW: &str = "2001:2::1";
pub static DEFAULT_TUN_IPV6_PREFIXLEN: i32 = 64;
