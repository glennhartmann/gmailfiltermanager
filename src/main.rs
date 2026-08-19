use std::path::PathBuf;

use clap::Parser;
use oauth2_simple::Oauth2Simple;

const XDG_BASE_PREFIX: &str = "gmailfiltermanager";
const CLIENT_INFO_FILE: &str = "client_info.json";
const TOKENS_FILE: &str = "token.json";
const SCOPE: &str = "https://www.googleapis.com/auth/gmail.settings.basic";

/// Gmail Filter Manager.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Port for the OAuth2 http server to listen on.
    #[arg(short, long, default_value_t = 42684)]
    listen_port: u16,

    /// Custom config directory to use instead of the standard XDG one.
    #[arg(short, long)]
    config_dir: Option<PathBuf>,
    // TODO: support DPoP
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (client_info_path, tokens_path) = match args.config_dir.as_ref() {
        None => {
            const ERROR_MSG: &str =
                "couldn't get config dir - please specify one manually with --config-dir.";

            let xdg_dirs = xdg::BaseDirectories::with_prefix(XDG_BASE_PREFIX);

            (
                xdg_dirs.get_config_file(CLIENT_INFO_FILE).expect(ERROR_MSG),
                xdg_dirs.get_config_file(TOKENS_FILE).expect(ERROR_MSG),
            )
        }
        Some(path) => {
            let mut client_info_path = path.clone();
            client_info_path.push(CLIENT_INFO_FILE);

            let mut tokens_path = path.clone();
            tokens_path.push(TOKENS_FILE);

            (client_info_path, tokens_path)
        }
    };

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
