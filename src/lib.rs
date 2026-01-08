use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Subtitle {
    pub id: u32,
    pub cnname: String,          // 中文名
    pub enname: Option<String>,  // 英文名
    pub segment: Option<String>, // 对应片源
    pub segment_num: u8,         // 第几段
    pub source: String,          // 来源 original 原创 trans 转载 official 官方字幕
    pub lang: String,            // 字幕语种
    pub format: String,          // 字幕格式
    pub file: Option<String>,    // 字幕文件
    pub views: u32,              // 浏览数
    pub downloads: u32,          // 下载次数
    pub dateline: i32,           // 时间戳
}
