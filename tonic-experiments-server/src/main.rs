use chrono::{SecondsFormat, Utc};
use serde_json::json;
use tonic_experiments_server::{init_tracing, run};
use tracing::error;

const ERROR_MSG: &str = "hello-tonic exited with ERROR";

#[tokio::main]
async fn main() {
    // Initialize tracing
    if let Err(e) = init_tracing() {
        let e = format!("{e:#}");
        let now = Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true);
        let msg = json!({
            "timestamp": now,
            "level": "ERROR",
            "fields": {
                "message": ERROR_MSG,
                "error": e,
            },
            "target": "tonic_experiments_server",
        });
        eprintln!("{msg}");
        return;
    }

    // Run
    if let Err(ref e) = run().await {
        error!(message = ERROR_MSG, error = display(format!("{e:#}")));
    };
}
