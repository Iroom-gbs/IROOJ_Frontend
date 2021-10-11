pub mod FileIO;
const BACKIP : &str = "127.0.0.1";

///백엔드의 IP 주소를 Stirng으로 반환합니다.
///
/// # Example
/// ```
/// let ip = DataManager::GetBackendIP();
/// ```
///
pub fn GetBackendIP() -> String {String::from(BACKIP)}

///백엔드의 소켓을 포함한 IP 주소를 String으로 반환합니다.
///
/// # Example
/// ```
/// let ip = DataManager::GetBackendSocket(5000);
/// ```
///
pub fn GetBackendSocket(socket: i32) -> String {format!("{}:{}",GetBackendIP(),socket)}