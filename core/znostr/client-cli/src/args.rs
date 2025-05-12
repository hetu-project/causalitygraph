use std::net::SocketAddr;
use std::path::PathBuf;
use clap::Parser;
use nostr_sdk::Url;
use crate::{Credentials, POW_DIFFICULTY_DEFAULT};
use crate::log::LogLevel;
use crate::output::Output;
use crate::version::Version;
use crate::util;
// A public struct with private fields to keep the command line arguments from
// library `clap`.
/// Welcome to "nostr-commander-rs", a Nostr CLI client. ───
/// On the first run use --create-user to create a user.
/// On further runs you can publish notes, send private DM messages,
/// etc.  ───
/// Have a look at the repo "https://github.com/8go/nostr-commander-rs/"
/// and see if you can contribute code to improve this tool.
/// Safe!
use clap::{ColorChoice, CommandFactory, ValueEnum};
#[derive(Clone, Debug, Parser)]
#[command(author, version,
next_line_help = true,
bin_name = util::get_prog_without_ext(),
color = ColorChoice::Always,
term_width = 79,
after_help = "PS: Also have a look at scripts/nostr-commander-tui.",
disable_version_flag = true,
disable_help_flag = true,
)]
pub struct Args {
    // This is an internal field used to store credentials.
    // The user is not setting this in the CLI.
    // This field is here to simplify argument passing.
    #[arg(skip)]
    pub creds: Credentials,

    /// Please contribute.
    #[arg(long, default_value_t = false)]
    pub contribute: bool,

    /// Print version number or check if a newer version exists on crates.io.
    /// Details::
    /// If used without an argument such as '--version' it will
    /// print the version number. If 'check' is added ('--version check')
    /// then the program connects to https://crates.io and gets the version
    /// number of latest stable release. There is no "calling home"
    /// on every run, only a "check crates.io" upon request. Your
    /// privacy is protected. New release is neither downloaded,
    /// nor installed. It just informs you.
    #[arg(short, long, value_name = "CHECK")]
    pub version: Option<Option<Version>>,

    /// Prints a very short help summary.
    /// Details:: See also --help, --manual and --readme.
    #[arg(long)]
    pub usage: bool,

    /// Prints short help displaying about one line per argument.
    /// Details:: See also --usage, --manual and --readme.
    #[arg(short, long)]
    pub  help: bool,

    /// Prints long help.
    /// Details:: This is like a man page.
    /// See also --usage, --help and --readme.
    #[arg(long)]
    pub manual: bool,

    /// Prints README.md file, the documenation in Markdown.
    /// Details:: The README.md file will be downloaded from
    /// Github. It is a Markdown file and it is best viewed with a
    /// Markdown viewer.
    /// See also --usage, --help and --manual.
    #[arg(long)]
    pub readme: bool,

    /// Overwrite the default log level.
    /// Details::
    /// If not used, then the default
    /// log level set with environment variable 'RUST_LOG' will be used.
    /// If used, log level will be set to 'DEBUG' and debugging information
    /// will be printed.
    /// '-d' is a shortcut for '--log-level DEBUG'.
    /// See also '--log-level'. '-d' takes precedence over '--log-level'.
    /// Additionally, have a look also at the option '--verbose'.
    #[arg(short, long,  action = clap::ArgAction::Count, default_value_t = 0u8, )]
    pub debug: u8,

    /// Set the log level by overwriting the default log level.
    /// Details::
    /// If not used, then the default
    /// log level set with environment variable 'RUST_LOG' will be used.
    /// See also '--debug' and '--verbose'.
    // Possible values are
    // '{trace}', '{debug}', '{info}', '{warn}', and '{error}'.
    #[arg(long, value_enum, default_value_t = LogLevel::default(), ignore_case = true, )]
    pub log_level: LogLevel,

    /// Set the verbosity level.
    /// Details::
    /// If not used, then verbosity will be
    /// set to low. If used once, verbosity will be high.
    /// If used more than once, verbosity will be very high.
    /// Verbosity only affects the debug information.
    /// So, if '--debug' is not used then '--verbose' will be ignored.
    #[arg(long,  action = clap::ArgAction::Count, default_value_t = 0u8, )]
    pub verbose: u8,

    // /// Disable encryption for a specific action. By default encryption is
    // /// turned on wherever possible. E.g. rooms created will be created
    // /// by default with encryption enabled. To turn encryption off for a
    // /// specific action use --plain. Currently --plain is supported by
    // /// --room-create and --room-dm-create. See also --room-enable-encryption
    // /// which sort of does the opossite for rooms.
    // #[arg(long, default_value_t = false)]
    // plain: bool,
    //
    //
    /// Specify a path to a file containing credentials.
    /// Details::
    /// At --create-user, information about the user, in particular
    /// its keys, will be written to a credentials file. By
    /// default, this file is "credentials.json". On further
    /// runs the credentials file is read to permit acting
    /// as this established Nostr user.
    /// If this option is provided,
    /// the provided path to a file will be used as credentials
    /// file instead of the default one.
    // e.g. /home/user/.local/share/nostr-commander-rs/credentials.json
    #[arg(short, long,
    value_name = "PATH_TO_FILE",
    value_parser = clap::value_parser!(PathBuf),
    default_value_os_t = util::get_credentials_default_path(),
    )]
    pub(crate) credentials: PathBuf,

    /// Create a new user, i.e. a new key pair.
    /// Details::
    /// This is usually
    /// done only once at the beginning. If you ever want to wipe
    /// this user, use '--delete-user' which deletes the key
    /// pair. Use this option in combination with --name,
    ///  --display_name, --about, --picture, and --nip05.
    /// Also highly recommended that you use this option
    /// together with --add-relay.
    #[arg(long, alias = "create-key", default_value_t = false)]
    pub create_user: bool,

    /// Delete the current user, i.e. delete the current key pair.
    /// Details::
    /// This will erase the key pair and other associated information
    /// like user name, display name, etc. Afterwards one can create
    /// a new user with '--create-user'.
    #[arg(long, alias = "delete-key", default_value_t = false)]
    pub delete_user: bool,

    /// Specify an optional user name.
    /// Details::
    /// Used together with
    /// '--create-user'.
    /// If this option is not set during '--create-user', the information
    /// will be queried via the keyboard. If you want to set it to empty
    /// and not be queried, provide an empty string ''.
    #[arg(long, value_name = "USER_NAME")]
    pub  name: Option<String>,

    /// Specify an optional display name.
    /// Details::
    /// Used together with
    /// '--create-user'.
    /// If this option is not set during '--create-user', the information
    /// will be queried via the keyboard. If you want to set it to empty
    /// and not be queried, provide an empty string ''.
    #[arg(long, value_name = "DISPLAY_NAME")]
    pub display_name: Option<String>,

    /// Specify an optional description.
    /// Details::
    /// Used together with
    /// '--create-user'.
    /// If this option is not set during '--create-user', the information
    /// will be queried via the keyboard. If you want to set it to empty
    /// and not be queried, provide an empty string ''.
    #[arg(long, value_name = "DESCRIPTION")]
    pub  about: Option<String>,

    /// Specify an optional picture or avatar.
    /// Details:: Used together with
    /// '--create-user'. Provide a URL like 'https://example.com/avatar.png'.
    // or a local file like 'file://somepath/someimage.jpg'.
    /// If this option is not set during '--create-user', the information
    /// will be queried via the keyboard. If you want to set it to empty
    /// and not be queried, provide this URL 'none:'.
    #[arg(long, value_name = "URL")]
    pub  picture: Option<Url>,

    /// Specify an optional nip05 name.
    /// Details::
    /// Used together with
    /// '--create-user'. Provide a nip05 name like 'john@example.org'.
    /// If this option is not set during '--create-user', the information
    /// will be queried via the keyboard. If you want to set it to empty
    /// and not be queried, provide an empty string ''.
    #[arg(long, value_name = "NIP05_ID")]
    pub  nip05: Option<String>,

    /// Publish one or multiple notes.
    /// Details::
    /// Notes data must not be binary data, it
    /// must be text.
    /// Input piped via stdin can additionally be specified with the
    /// special character '-'.
    /// If you want to feed a text message into the program
    /// via a pipe, via stdin, then specify the special
    /// character '-'.
    /// If your message is literally a single letter '-' then use an
    /// escaped '\-' or a quoted "\-".
    /// Depending on your shell, '-' might need to be escaped.
    /// If this is the case for your shell, use the escaped '\-'
    /// instead of '-' and '\\-' instead of '\-'.
    /// However, depending on which shell you are using and if you are
    /// quoting with double quotes or with single quotes, you may have
    /// to add backslashes to achieve the proper escape sequences.
    /// If you want to read the message from
    /// the keyboard use '-' and do not pipe anything into stdin, then
    /// a message will be requested and read from the keyboard.
    /// Keyboard input is limited to one line.
    /// The stdin indicator '-' may appear in any position,
    /// i.e. --publish 'start' '-' 'end'
    /// will send 3 messages out of which the second one is read from stdin.
    /// The stdin indicator '-' may appear only once overall in all arguments.
    /// '-' reads everything that is in the pipe in one swoop and
    /// sends a single message.
    /// Similar to '-', another shortcut character
    /// is '_'. The special character '_' is used for
    /// streaming data via a pipe on stdin. With '_' the stdin
    /// pipe is read line-by-line and each line is treated as
    /// a separate message and sent right away. The program
    /// waits for pipe input until the pipe is closed. E.g.
    /// Imagine a tool that generates output sporadically
    /// 24x7. It can be piped, i.e. streamed, into
    /// nostr-commander, and nostr-commander stays active, sending
    /// all input instantly. If you want to send the literal
    /// letter '_' then escape it and send '\_'. '_' can be
    /// used only once. And either '-' or '_' can be used.
    #[arg(short, long, value_name = "NOTE", num_args(0..), )]
    pub  publish: Vec<String>,

    /// Publish one or multiple notes with proof-of-work (POW).
    /// Details::
    /// Use also '--pow-difficulty' to specify difficulty.
    /// See also '--publish' to see how shortcut characters
    /// '-' (pipe) and '_' (streamed pipe) are handled.
    /// Disabled since version nostr-commander-rs 0.2.0 (nostr-sdk 0.21).
    #[arg(long, alias = "pow", value_name = "NOTE", num_args(0..), )]
    pub  publish_pow: Vec<String>, // ToDo: remove this option

    /// Send one or multiple DMs to one given user.
    /// Details::
    /// DM messages will be encrypted and preserve privacy.
    /// The single recipient is specified via its public key, a
    /// string in the form of 'npub1...', a Hex key, or an alias from
    /// one of your contacts. The first argument
    /// is the recipient, all further arguments are texts to be
    /// sent. E.g. '-dm "npub1SomeStrangeNumbers" "First msg" "Second msg"'
    /// or '--dm joe "How about pizza tonight?"'.
    /// See also '--publish' to see how shortcut characters
    /// '-' (pipe) and '_' (streamed pipe) are handled.
    #[arg(long, alias = "direct", value_name = "KEY+MSGS", num_args(0..), )]
    pub  dm: Vec<String>,

    /// Send one or multiple messages to one given channel.
    /// Details::
    /// The single destination channel is specified via its hash.
    /// See here for a channel list: https://damus.io/channels/.
    /// The first argument
    /// is the channel hash, all further arguments are texts to be
    /// sent. E.g.
    /// '-send_channel_message "SomeChannelHash" "First msg" "Second msg"'.
    // or '--send_channel_message joe "How about pizza tonight?"'.
    /// See also '--publish' to see how shortcut characters
    /// '-' (pipe) and '_' (streamed pipe) are handled.
    /// Optionally you can provide a relay to be used for the channel send
    /// by using --relay. See --relay. If --relay has values the first value
    /// from --relay will be used as relay. If --relay is not used, then
    /// the first relay in the relay list in the credentials configuration
    /// file will be used.
    #[arg(long, alias = "chan", value_name = "HASH+MSGS", num_args(0..), )]
    pub  send_channel_message: Vec<String>,


    /// Add one or multiple relays.
    /// Details::
    /// A relay is specified via a URI
    /// that looks like 'wss://some.relay.org'. You can find relays
    /// by looking at https://github.com/aljazceru/awesome-nostr#instances.
    /// Sampler relay registries are: https://nostr-registry.netlify.app/,
    /// https://nostr.info/, or https://nostr.watch/.
    /// Examples: "wss://relay.damus.io", "wss://nostr.openchain.fr".
    /// See also '--proxy'.
    #[arg(long, value_name = "RELAY_URI", num_args(0..), )]
    pub add_relay: Vec<Url>,

    /// Specify a proxy for relays.
    /// Details:: Used by --add-relay.
    /// Note that this proxy will be applied to all of the relays specified
    /// with --add-relay. If you have 3 relays with 3 different proxies, then
    /// run the --add-relay command 3 times with 1 relay and 1 proxy each time.
    /// An example proxy for the Tor network looks something like "127.0.0.1:9050".
    /// If you want to use Tor via a proxy, to assure that no information
    /// leaks you must use only one relay, i.e. the Tor relay.
    /// If more then one relays are configured, data will be communicated to
    /// and from all relays.
    /// A possible relay that you can use together with a Tor proxy is
    /// "ws://jgqaglhautb4k6e6i2g34jakxiemqp6z4wynlirltuukgkft2xuglmqd.onion".
    #[arg(long)]
    pub proxy: Option<SocketAddr>,

    /// Remove one or multiple relays from local config file.
    /// Details:: See --add-relay.
    #[arg(long, value_name = "RELAY_URI", num_args(0..), )]
    pub  remove_relay: Vec<Url>,

    // todo tag
    /// Specify one or multiple tags to attach to notes or DMs.
    /// Details:: Not yet implemented.
    #[arg(long)]
    pub tag: Vec<String>,

    /// Display current metadata.
    /// Details:: shows data in your config file.
    #[arg(long, default_value_t = false)]
    pub show_metadata: bool,

    /// Modify existing metadata of the user.
    /// Details::
    /// Use this option in combination with --name,
    ///  --display_name, --about, --picture, and --nip05.
    #[arg(long, default_value_t = false)]
    pub  change_metadata: bool,

    /// Specify optional proof-of-work (POW) difficulty.
    /// Details::
    /// Use with '--publish_pow' to specify difficulty.
    /// If not specified the default will be used.
    #[arg(long, value_name = "DIFFICULTY", default_value_t = POW_DIFFICULTY_DEFAULT, )]
    pub pow_difficulty: u8,

    /// Show public key.
    /// Details:: Displays your own public key. You can share this
    /// with your friends or the general public.
    #[arg(long, default_value_t = false)]
    pub show_public_key: bool,

    /// Show private, secret key.
    /// Details:: Protect this key. Do not share this with anyone.
    #[arg(long, default_value_t = false)]
    pub show_secret_key: bool,

    /// Print the user name used by "nostr-commander-rs".
    /// Details::
    /// One can get this information also by looking at the
    /// credentials file or by using --show-metadata.
    #[arg(long)]
    pub whoami: bool,

    /// Select an output format.
    /// Details:: This option decides on how the output is presented.
    /// Currently offered choices are: 'text', 'json', 'json-max',
    /// and 'json-spec'. Provide one of these choices.
    /// The default is 'text'. If you want to use the default,
    /// then there is no need to use this option. If you have
    /// chosen 'text', the output will be formatted with the
    /// intention to be consumed by humans, i.e. readable
    /// text. If you have chosen 'json', the output will be
    /// formatted as JSON. The content of the JSON object
    /// matches the data provided by the nostr-sdk SDK. In
    /// some occassions the output is enhanced by having a few
    /// extra data items added for convenience. In most cases
    /// the output will be processed by other programs rather
    /// than read by humans. Option 'json-max' is practically
    /// the same as 'json', but yet another additional field
    /// is added.
    /// In most cases the output will
    /// be processed by other programs rather than read by
    /// humans. Option 'json-spec' only prints information
    /// that adheres 1-to-1 to the Nostr Specification.
    /// Currently this type is not supported.
    /// If no data is available that corresponds exactly with
    /// the Nostr Specification, no data will be printed.
    #[arg(short, long, value_enum,
    value_name = "OUTPUT_FORMAT",
    default_value_t = Output::default(), ignore_case = true, )]
    pub output: Output,

    /// Listen to events, notifications and messages.
    /// Details::
    /// This option listens to events and messages forever. To stop, type
    /// Control-C on your keyboard. You want to listen if you want
    /// to get the event ids for published notices.
    /// Subscriptions do not automatically turn listening on.
    /// If you want to listen to your subscriptions, you must use
    /// --listen.
    #[arg(short, long, default_value_t = false)]
    pub listen: bool,

    /// Add one or more contacts.
    /// Details:: Must be used in combination with
    /// --alias, --key, --relay. If you want to add N new contacts,
    /// use --add-contact and provide exactly N entries in each
    /// of the 3 extra arguments. E.g. --add-contact --alias jane joe
    /// --key npub1JanesPublicKey npub1JoesPublicKey
    /// --relay "wss://janes.relay.org" "wss://joes.relay.org".
    /// Aliases must be unique. Alias can be seen as a nickname.
    #[arg(long, default_value_t = false)]
    pub  add_contact: bool,

    /// Remove one or more contacts.
    /// Details:: Must be used in combination with
    /// --alias. For each entry in --alias the corresponding contact will
    /// be removed. E.g. --remove-contact --alias jane joe.
    #[arg(long, default_value_t = false)]
    pub remove_contact: bool,

    /// Display current contacts.
    /// Details:: Prints your contact list.
    #[arg(long, default_value_t = false)]
    pub show_contacts: bool,

    /// Provide one or multiple aliases (nicknames).
    /// Details:: This is used in combination with arguments
    /// --add-contact and --remove-contact.
    #[arg(long, value_name = "ALIAS", num_args(0..), )]
    pub alias: Vec<String>,

    /// Provide one or multiple public keys.
    /// Details:: This is used in combination with argument
    /// --add-contact. They have the form 'npub1SomeStrangeString'.
    /// Alternatively you can use the Hex form of the public key.
    #[arg(long, value_name = "KEY", num_args(0..), )]
    pub key: Vec<String>,

    /// Provide one or multiple relays.
    /// Details:: This is used in combination with arguments
    /// --add-contact and --send_channel_message.
    /// Relays have the form 'wss://some.relay.org'.
    #[arg(long, value_name = "RELAY", num_args(0..), )]
    pub  relay: Vec<Url>,

    /// Convert one or multiple public keys from Npub to Hex.
    /// Details:: Converts public keys in Bech32 format ('npub1...') into
    /// the corresponding 'hex' format.
    /// See also --hex-to-npub.
    #[arg(long, value_name = "KEY", num_args(0..), )]
    pub  npub_to_hex: Vec<String>,

    /// Convert one or multiple public keys from Hex to Npub.
    /// Details:: Converts public keys in 'hex' format into
    /// the corresponding Bech32 ('npub1...') format.
    /// See also --npub-to-hex.
    #[arg(long, value_name = "KEY", num_args(0..), )]
    pub  hex_to_npub: Vec<String>,

    /// Get the entity of one or multiple public keys.
    /// Details:: This will show you
    /// for every public key given if the key represents a Nostr account
    /// (usually an individual) or a public Nostr channel. It might also
    /// return "Unknown" if the entity of the key cannot be determined.
    /// E.g. this can be helpful to determine if you want to use
    /// --subscribe-author or --subscribe-channel.
    #[arg(long, value_name = "KEY", num_args(0..), )]
    pub get_pubkey_entity: Vec<String>,

    /// Subscribe to one or more public keys.
    /// Details:: Specify each
    /// public key in form of 'npub1SomePublicKey'.
    /// Alternatively you can use the Hex form of the public key.
    /// Use this option to subscribe to an account, i.e. the key of
    /// an individual.
    /// See also --subscribe-channel which are different.
    #[arg(long, value_name = "KEY", num_args(0..), )]
    pub  subscribe_pubkey: Vec<String>,

    /// Subscribe to authors with to one or more public keys of accounts.
    /// Details:: Specify each
    /// public key in form of 'npub1SomePublicKey'.
    /// Alternatively you can use the Hex form of the public key.
    /// Use this option to subscribe to a Nostr accounts (usually individuals).
    /// Provide keys that represent accounts (see --get-pubkey-entity).
    /// See also --subscribe-pubkey and --subscribe-channel which are different.
    #[arg(long, value_name = "KEY", num_args(0..), )]
    pub  subscribe_author: Vec<String>,

    /// Subscribe to public channels with one or more hashes of channels.
    /// Details:: Specify each
    /// hash in form of 'npub1SomePublicKey'.
    /// Alternatively you can use the Hex form of the public key.
    /// Sometimes the hash of a public channel is referred to as
    /// channel id, sometimes also as public channel key.
    /// See here for a channel list: https://damus.io/channels/.
    /// Provide hashes that represent public channels (see --get-pubkey-entity).
    /// See also --subscribe-pubkey and --subscribe-author which are different.
    #[arg(long, value_name = "HASH", num_args(0..), )]
    pub  subscribe_channel: Vec<String>,

    /// Unsubscribe from public key.
    /// Details:: Removes one or multiple public keys from the
    /// public key subscription list.
    /// See --subscribe-pubkey.
    #[arg(long, value_name = "KEY", num_args(0..), )]
    pub  unsubscribe_pubkey: Vec<String>,

    /// Unsubscribe from author.
    /// Details:: Removes one or multiple public keys from the
    /// author subscription list.
    /// See --subscribe-author.
    #[arg(long, value_name = "KEY", num_args(0..), )]
    pub  unsubscribe_author: Vec<String>,

    /// Unsubscribe from public channel.
    /// Details:: Removes one or multiple public keys from the
    /// public channel subscription list.
    /// See --subscribe-channel.
    #[arg(long, value_name = "KEY", num_args(0..), )]
    pub  unsubscribe_channel: Vec<String>,

    /// Limit the number of past messages to receive when subscribing.
    /// Details:: By default there is no limit (0), i.e. all old messages
    /// available to the relay will be received.
    #[arg(long, value_name = "NUMBER", default_value_t = 0)]
    pub  limit_number: usize,

    /// Limit the messages received to the last N days when subscribing.
    /// Details:: By default there is no limit (0), i.e. all old messages
    /// available to the relay will be received.
    #[arg(long, alias = "since-days", value_name = "DAYS", default_value_t = 0)]
    pub limit_days: u64,

    /// Limit the messages received to the last N hours when subscribing.
    /// Details:: By default there is no limit (0), i.e. all old messages
    /// available to the relay will be received.
    #[arg(long, alias = "since-hours", value_name = "HOURS", default_value_t = 0)]
    pub  limit_hours: u64,

    /// Limit the messages received to the next N days when subscribing.
    /// Details:: Stop receiving N days in the future.
    /// By default there is no limit (0), i.e. you will receive events forever.
    #[arg(long, alias = "until-days", value_name = "DAYS", default_value_t = 0)]
    pub limit_future_days: u64,

    /// Limit the messages received to the last N hours when subscribing.
    /// Details:: Stop receiving N hours in the future.
    /// By default there is no limit (0), i.e. you will receive events forever.
    #[arg(long, alias = "until-hours", value_name = "HOURS", default_value_t = 0)]
   pub limit_future_hours: u64,


    ///show the credential file path
    #[arg(long, default_value_t = false)]
    pub show_cred_path:bool,

    #[arg(long, default_value_t = false)]
    pub publish_poll: bool,

    #[arg(long, default_value_t = false)]
    pub vote: bool,

    #[arg(long, default_value_t = false)]
    pub query_poll_state: bool,

    #[arg(long, default_value_t = false)]
    pub get_eids_poll: bool,
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    pub fn new() -> Args {
        Args {
            creds: Credentials::new(),
            usage: false,
            help: false,
            manual: false,
            readme: false,
            contribute: false,
            version: None,
            debug: 0u8,
            log_level: LogLevel::None,
            verbose: 0u8,
            // plain: false,
            // credentials file path
            credentials: util::get_credentials_default_path(),
            create_user: false,
            delete_user: false,
            name: None,
            display_name: None,
            about: None,
            picture: None,
            nip05: None,
            publish: Vec::new(),
            publish_pow: Vec::new(),
            dm: Vec::new(),
            send_channel_message: Vec::new(),
            add_relay: Vec::new(),
            remove_relay: Vec::new(),
            tag: Vec::new(),
            show_metadata: false,
            change_metadata: false,
            pow_difficulty: POW_DIFFICULTY_DEFAULT,
            proxy: None,
            show_public_key: false,
            show_secret_key: false,
            whoami: false,
            output: Output::default(),
            listen: false,
            add_contact: false,
            remove_contact: false,
            show_contacts: false,
            alias: Vec::new(),
            key: Vec::new(),
            relay: Vec::new(),
            npub_to_hex: Vec::new(),
            hex_to_npub: Vec::new(),
            get_pubkey_entity: Vec::new(),
            subscribe_pubkey: Vec::new(),
            subscribe_author: Vec::new(),
            subscribe_channel: Vec::new(),
            unsubscribe_pubkey: Vec::new(),
            unsubscribe_author: Vec::new(),
            unsubscribe_channel: Vec::new(),
            limit_number: 0,
            limit_days: 0,
            limit_hours: 0,
            limit_future_days: 0,
            limit_future_hours: 0,
            show_cred_path:false,
            publish_poll:false,
            vote:false,
            query_poll_state:false,
            get_eids_poll:false
        }
    }
}
