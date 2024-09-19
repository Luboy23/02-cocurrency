// metrics data structure
// 基本功能： inc/dec/snapshot
use anyhow::Result;
use std::{sync::Arc, fmt};
use dashmap::DashMap;


#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            data:Arc::new(DashMap::new()),
         }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    // pub fn dec(&self, key: impl Into<String>) -> Result<()> {
    //     let mut data = self.data.lock()
    //     .map_err(|e| anyhow!(e.to_string()))?;
    //     let counter = data.entry(key.into()).or_insert(0);
    //     *counter -= 1;
    //     Ok(())
    // }
    

}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}:{}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}