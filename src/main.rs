use std::path::PathBuf;

use gmailfiltermanager::{download, list_local, list_remote, upload};

use clap::{Parser, Subcommand};
use oauth2_simple::Oauth2Simple;

const DEFAULT_DIR_NAME: &str = "gmailfiltermanager";
const DEFAULT_DATA_FILE_NAME: &str = "filters.json";
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

    #[command(subcommand)]
    command: Command,
    // TODO: support DPoP
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Download filters from your Gmail account and save them in a local file.
    Download {
        /// If the specified file already exists, this parameter controls whether or not to delete
        /// entries in the file that don't appear in your Gmail account.
        #[arg(short, long, default_value_t = true)]
        delete_unlisted: bool,

        /// The file to write your filters to. If unspecified, this defaults to
        /// <your_user_data_dir>/gmailfiltermanager/filters.json. By default, that means
        /// "~/.local/share/gmailfiltermanager/filters.json" on Linux,
        /// "C:\Users\<your_username>\AppData\Roaming\gmailfiltermanager\filters.json" on Windows,
        /// or "~/Library/Application Support/gmailfiltermanager/filters.json" on MacOS.
        #[arg(short, long)]
        output_file: Option<PathBuf>,
    },

    /// Read filters from a local file and upload them to your Gmail account.
    Upload {
        /// If true, this will cause all the filters in your Gmail account that aren't present in
        /// the local file to be deleted.
        #[arg(short, long, default_value_t = false)]
        delete_unlisted: bool,

        /// If true, ALL filters in the file will be overwritten on the Gmail server. Any data
        /// outside of the Gmail API will be lost.
        #[arg(short, long, default_value_t = false)]
        force: bool,

        /// Only upload filters from the local file which don't exist yet on the Gmail account.
        /// That means that any modifications to existing filters will not be applied.
        #[arg(short, long, default_value_t = false)]
        only_new: bool,

        /// The file with your filters. If unspecified, this defaults to
        /// <your_user_data_dir>/gmailfiltermanager/filters.json. By default, that means
        /// "~/.local/share/gmailfiltermanager/filters.json" on Linux,
        /// "C:\Users\<your_username>\AppData\Roaming\gmailfiltermanager\filters.json" on Windows,
        /// or "~/Library/Application Support/gmailfiltermanager/filters.json" on MacOS.
        #[arg(short, long)]
        input_file: Option<PathBuf>,
    },

    /// Prints the filters from your local file.
    ListLocal {
        /// The file with your filters. If unspecified, this defaults to
        /// <your_user_data_dir>/gmailfiltermanager/filters.json. By default, that means
        /// "~/.local/share/gmailfiltermanager/filters.json" on Linux,
        /// "C:\Users\<your_username>\AppData\Roaming\gmailfiltermanager\filters.json" on Windows,
        /// or "~/Library/Application Support/gmailfiltermanager/filters.json" on MacOS.
        #[arg(short, long)]
        input_file: Option<PathBuf>,
    },

    /// Prints the filters from your Gmail account.
    ListRemote,
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

    let data_file = |file: Option<&PathBuf>| -> PathBuf {
        let mut data_dir = PathBuf::new();
        file.unwrap_or_else(|| {
            data_dir = dirs::data_dir()
                .expect("couldn't get data dir - please specify your data file manually.");
            data_dir.push(DEFAULT_DIR_NAME);
            data_dir.push(DEFAULT_DATA_FILE_NAME);
            &data_dir
        })
        .to_path_buf()
    };
    match args.command {
        Command::Download {
            delete_unlisted,
            output_file,
        } => {
            download(
                &tokens.access_token,
                delete_unlisted,
                &data_file(output_file.as_ref()),
            )
            .await
        }
        Command::Upload {
            delete_unlisted,
            force,
            only_new,
            input_file,
        } => upload(
            &tokens.access_token,
            delete_unlisted,
            force,
            only_new,
            &data_file(input_file.as_ref()),
        ),
        Command::ListLocal { input_file } => {
            list_local(&tokens.access_token, &data_file(input_file.as_ref()))
        }
        Command::ListRemote => list_remote(&tokens.access_token).await,
    }
    .expect("unspecified error"); // TODO: better error
}
