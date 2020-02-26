use serde_json::Value;
use std::io::Cursor;
use zip::ZipArchive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let body = reqwest::get("http://dev.azure.com/reitermarkus/serverless/_apis/build/builds?definitions=2&$top=1&api-version=5.0-preview.5")
    .await?
    .text()
    .await?;

  let json = serde_json::from_str::<Value>(&body)?;
  let id = json["value"][0]["id"].as_i64().unwrap();

  let url = format!("https://dev.azure.com/reitermarkus/9f00b2ca-5e57-4700-aee5-5e7c454ffc52/_apis/build/builds/{}/artifacts?artifactName=thesis&api-version=5.1&%24format=zip", id);

  let archive_bytes = reqwest::get(&url)
      .await?
      .bytes()
      .await?;

  let mut pdf_archive = ZipArchive::new(Cursor::new(archive_bytes))?;

  let mut buffer: Vec<u8> = Vec::new();

  match pdf_archive.by_name("thesis/thesis.pdf") {
    Ok(mut f) => std::io::copy(&mut f, &mut buffer)?,
    Err(..) =>  panic!("\"thesis/thesis.pdf\" not found")
  };

  let _ = base64::encode(&buffer);

  Ok(())
}