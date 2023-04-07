use crate::storage::{PackageInfo, PackageStorage, PublishInfo};

pub struct SqlitePackageStorage;

impl PackageStorage for SqlitePackageStorage {
    async fn load_packages(&self) -> anyhow::Result<Vec<PackageInfo>> {
        todo!()
    }

    async fn load_package(&self, package: &str) -> anyhow::Result<Option<PackageInfo>> {
        todo!()
    }

    async fn store_package(&self, info: &PackageInfo) -> anyhow::Result<()> {
        todo!()
    }

    async fn load_publish(&self) -> anyhow::Result<Option<PublishInfo>> {
        todo!()
    }

    async fn store_publish(&self, info: Option<&PublishInfo>) -> anyhow::Result<()> {
        todo!()
    }
}