use std::collections::HashMap; // 引入 HashMap，用于存储计数器
use std::sync::atomic::AtomicI64; // 引入原子类型 AtomicI64，用于线程安全的整数计数
use std::sync::Arc; // 引入 Arc，提供线程安全的引用计数
use anyhow::Result; // 引入 Result 类型，方便错误处理
use std::sync::atomic::Ordering; // 引入原子操作的排序选项
use std::fmt; // 引入格式化功能

// 定义 AmapMetrics 结构体，表示一个用于计数的指标集合
#[derive(Debug)] // 为结构体自动实现 Debug trait
pub struct AmapMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>, // 使用 Arc 包装的 HashMap，存储指标名称与其计数器的映射
}

impl AmapMetrics {
    // 构造函数，接收一个字符串切片数组作为指标名称
    pub fn new(metric_names: &[&'static str]) -> Self {
        // 遍历指标名称，创建一个 HashMap，键为指标名称，值为 AtomicI64（初始值为 0）
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0))) // 每个计数器初始为 0
            .collect();
        // 返回新的 AmapMetrics 实例
        AmapMetrics {
            data: Arc::new(map), // 将 HashMap 包装在 Arc 中，支持线程安全的共享
        }
    }

    // 增加指定键的计数器
    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref(); // 获取键的引用
        // 从 HashMap 中获取对应的计数器
        let counter = self.data.get(key)
            .ok_or_else(|| anyhow::anyhow!("key {} not found", key))?; // 如果找不到，返回错误
        counter.fetch_add(1, Ordering::Relaxed); // 以原子方式增加计数器
        Ok(()) // 返回成功
    }
}

// 为 AmapMetrics 实现 Clone trait，以便复制实例
impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: Arc::clone(&self.data), // 克隆 Arc，保持引用计数
        }
    }
}

// 为 AmapMetrics 实现 Display trait，以便自定义输出格式
impl fmt::Display for AmapMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 遍历 HashMap，格式化每个键值对
        for (key, value) in self.data.iter() {
            // 将每个计数器的当前值写入格式化器
            writeln!(f, "{}:{}", key, value.load(Ordering::Relaxed))?; // 读取计数器的值并输出
        }
        Ok(()) // 返回成功
    }
}
