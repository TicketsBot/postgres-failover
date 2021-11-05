use crate::{Config, Error, Result};
use tokio::process::Command;

const ALREADY_MASTER_ERR: &'static str =
    "pg_ctl: cannot promote server; server is not in standby mode";

pub async fn promote(config: &Config) -> Result<()> {
    let child = Command::new("pg_ctlcluster")
        .arg(format!("{}", config.cluster.version))
        .arg(config.cluster.name.as_str())
        .arg("promote")
        .output()
        .await?;

    let success = match child.status.code() {
        Some(0) => true,
        _ => false,
    };

    if success {
        Ok(())
    } else {
        let message = String::from_utf8_lossy(&child.stderr).trim_end().to_string();
        if message == ALREADY_MASTER_ERR {
            Error::AlreadyMaster.into()
        } else {
            Error::PgCtlError(message).into()
        }
    }
}
