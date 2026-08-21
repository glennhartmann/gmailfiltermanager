mod gmail_filters_api;

#[cfg(unix)]
use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
use std::{
    fs::{DirBuilder, File, create_dir_all},
    io::Write,
    path::PathBuf,
};

pub async fn download(
    token: &str,
    _delete_unlisted: bool,
    output_file: &PathBuf,
) -> anyhow::Result<()> {
    let app_data_dir = output_file
        .parent()
        .ok_or(anyhow::anyhow!("couldn't get app data dir"))?;
    let user_data_dir = app_data_dir.parent().ok_or(anyhow::anyhow!(
        "couldn't get app data dir's parent. Do not store your data directly at '/'."
    ))?;

    #[cfg(unix)]
    {
        create_dir_all(user_data_dir)?;
        DirBuilder::new()
            .mode(0o700)
            .recursive(true)
            .create(app_data_dir)?;
    }
    #[cfg(not(unix))]
    create_dir_all(app_data_dir)?;

    // TODO: merge data if !delete_unlisted

    let filters = gmail_filters_api::list(token).await?;
    let js = serde_json::to_string_pretty(&filters)?;

    let mut f = File::create(output_file)?;

    #[cfg(unix)]
    {
        let md = f.metadata()?;
        let mut perms = md.permissions();
        perms.set_mode(0o600);
        f.set_permissions(perms)?;
    }

    f.write_all(&js.into_bytes())?;

    Ok(())
}

pub fn upload(
    _token: &str,
    _delete_unlisted: bool,
    _force: bool,
    _only_new: bool,
    _input_file: &PathBuf,
) -> anyhow::Result<()> {
    Ok(())
}

pub fn list_local(_token: &str, _input_file: &PathBuf) -> anyhow::Result<()> {
    Ok(())
}

pub async fn list_remote(token: &str) -> anyhow::Result<()> {
    let filters = gmail_filters_api::list(token).await?;
    println!("{}", serde_json::to_string_pretty(&filters)?);
    Ok(())
}
