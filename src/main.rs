/// ログの出力制御
pub async fn init_logger() -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
  let subscriber = tracing_subscriber::fmt()
    .with_max_level(tracing::Level::INFO)
    .finish();
  tracing::subscriber::set_global_default(subscriber)?;
  Ok(())
}

#[tokio::main]
async fn main() {
  println!("Hello, world!");
}
