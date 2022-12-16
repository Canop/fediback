mod app;
mod args;
mod client;
mod conf;
mod db;
mod error;
mod note;
mod page;
mod public_key;
mod skin;
mod user;
mod user_ref;

pub use {
    app::*,
    args::*,
    client::*,
    conf::*,
    db::*,
    error::*,
    note::*,
    page::*,
    public_key::*,
    user::*,
    user_ref::*,
};

use {
    cli_log::*,
    termimad::{MadSkin, mad_print_inline},
};

fn main() -> Result<()> {
    init_cli_log!();
    let args: Args = argh::from_env();
    debug!("args: {:#?}", &args);
    if args.version {
        println!("fediback {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if args.paths {
        println!("configuration file: {:?}", Conf::standard_location()?);
        println!("data directory: {:?}", Db::standard_location()?);
        return Ok(());
    }
    let conf_path = match args.conf {
        Some(path) => path,
        None => Conf::standard_location()?,
    };
    let mut conf = Conf::read(&conf_path)?;
    let skin = skin::make_skin();
    match args.command {
        Some(ArgsCommand::Check(CheckCommand { user })) => {
            check(&user, &skin)?;
        }
        Some(ArgsCommand::Add(AddCommand { user })) => {
            check(&user, &skin)?;
            conf.add(user);
            skin.print_text(
                "Run `fediback update` to fetch the complete profile."
            );
            conf.save(&conf_path, &skin)?;
        }
        Some(ArgsCommand::Remove(RemoveCommand { user })) => {
            conf.remove(&user);
            conf.save(&conf_path, &skin)?;
        }
        Some(ArgsCommand::List(_)) => {
            let mut users: Vec<&UserRef> = conf
                .watched
                .iter()
                .collect();
            if users.is_empty() {
                skin.print_text(
                    "No user watched. Use `fediback add @name@instance` to add one.",
                );
            } else {
                users.sort();
                skin.print_text("Watched accounts:");
                for user in users {
                    mad_print_inline!(&skin, " **$0**\n", user);
                }
            }
        }
        Some(ArgsCommand::Update(_)) | None => {
            let db_path = match args.data {
                Some(path) => path,
                None => Db::standard_location()?,
            };
            let db = Db::new(db_path, &skin)?;
            db.update(&conf, 8)?;
        }
    }
    info!("bye");
    Ok(())
}

fn check(user_ref: &UserRef, skin: &MadSkin) -> Result<()> {
    let client = Client::new()?;
    let user: User = user_ref.get_user(&client)?;
    mad_print_inline!(
        skin,
        "user **$0** successfully checked.\n",
        &user.preferred_username,
    );
    Ok(())
}
