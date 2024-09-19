// CmapMetrics 数据结构
// 基本功能：增加 (inc)、减少 (dec)、快照 (snapshot)
use anyhow::Result; // 引入 Result 类型，方便错误处理
use std::{sync::Arc, fmt}; // 引入 Arc 用于线程安全的引用计数，和 fmt 用于格式化输出
use dashmap::DashMap; // 引入 DashMap，提供线程安全的 HashMap 实现

// 定义 CmapMetrics 结构体
#[derive(Debug, Clone)] // 自动实现 Debug 和 Clone trait
pub struct CmapMetrics {
    data: Arc<DashMap<String, i64>>, // 使用 Arc 包装的 DashMap，存储字符串键和 i64 值
}

impl CmapMetrics {
    // 构造函数，初始化一个新的 CmapMetrics 实例
    pub fn new() -> Self {
        CmapMetrics {
            data: Arc::new(DashMap::new()), // 创建一个新的 DashMap 实例
        }
    }

    // 增加指定键的计数器
    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        // 获取或插入键对应的计数器
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1; // 增加计数器的值
        Ok(()) // 返回成功
    }

    // 减少指定键的计数器（注释掉的部分）
    // pub fn dec(&self, key: impl Into<String>) -> Result<()> {
    //     let mut data = self.data.lock()
    //         .map_err(|e| anyhow!(e.to_string()))?; // 锁定数据，处理可能的错误
    //     let counter = data.entry(key.into()).or_insert(0); // 获取或插入键对应的计数器
    //     *counter -= 1; // 减少计数器的值
    //     Ok(()) // 返回成功
    // }
}

// 为 CmapMetrics 实现 Display trait，以便自定义输出格式
impl fmt::Display for CmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 遍历 DashMap，格式化每个键值对
        for entry in self.data.iter() {
            writeln!(f, "{}:{}", entry.key(), entry.value())?; // 写入每个计数器的当前值
        }
        Ok(()) // 返回成功
    }
}
