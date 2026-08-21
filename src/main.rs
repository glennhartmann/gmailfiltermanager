use std::path::PathBuf;

use clap::Parser;
use oauth2_simple::Oauth2Simple;

const DEFAULT_DIR_NAME: &str = "gmailfiltermanager";
const CLIENT_INFO_FILE_NAME: &str = "client_info.json";
const TOKENS_FILE_NAME: &str = "token.json";
const SCOPE: &str = "https://www.googleapis.com/auth/gmail.settings.basic";

/// Gmail Filter Manager.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port for the OAuth2 http server to listen on.
    #[arg(short, long, default_value_t = 42684)]
    listen_port: u16,

    /// Custom config directory to use instead of the default one. The default one is
    /// <your_local_user_config_dir>/gmailfiltermanager. By default, that means
    /// "~/.config/gmailfiltermanager" on Linux,
    /// "C:\Users\<your_username>\AppData\Local\gmailfiltermanager" on Windows, or
    /// "~/Library/Application Support/gmailfiltermanager" on MacOS.
    #[arg(short, long)]
    config_dir: Option<PathBuf>,
    // TODO: support DPoP
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut config_dir =
        args.config_dir
            .unwrap_or(dirs::config_local_dir().expect(
                "couldn't get config dir - please specify one manually with --config-dir.",
            ));
    config_dir.push(DEFAULT_DIR_NAME);
    let config_dir = config_dir;

    let mut client_info_path = config_dir.clone();
    client_info_path.push(CLIENT_INFO_FILE_NAME);
    let client_info_path = client_info_path;

    let mut tokens_path = config_dir;
    tokens_path.push(TOKENS_FILE_NAME);
    let tokens_path = tokens_path;

    let mut oa2 = Oauth2Simple::with_cfg(oauth2_simple::Cfg {
        client_info_path: client_info_path.to_str().expect("invalid client_info_path"),
        tokens_path: tokens_path.to_str().expect("invalid tokens_path"),
        port: args.listen_port,
        scope: SCOPE,
    })
    .expect("error in Oauth2Simple::with_cfg()");
    let tokens = oa2
        .get_token()
        .await
        .expect("error in Oauth2Simple::get_token()");
    println!("{:?}", tokens);
}
