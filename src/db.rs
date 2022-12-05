use {
    crate::*,
    cli_log::*,
    crossbeam::{channel, thread},
    serde::Serialize,
    std::{
        fs,
        io::{self, Write},
        path::{Path, PathBuf},
    },
    termimad::{
        crossterm::{
            cursor,
            style::{style, Color, Print, PrintStyledContent, Stylize},
            terminal::{Clear, ClearType},
            queue,
        },
        ProgressBar,
    },
};

/// the database
#[derive(Debug)]
pub struct Db<'s> {
    /// where the DB is on disk
    pub dir: PathBuf,
    /// whether to tell everything when we work
    pub verbose: bool,
    /// whether to save on disk
    pub read_only: bool,
    /// skin used to display info on updating process
    skin: &'s MadSkin,
}

impl<'s> Db<'s> {

    pub fn new(skin: &'s MadSkin) -> Result<Self> {
        let dir = app_dirs()?.data_dir().to_path_buf();
        let verbose = false;
        let read_only = false;
        Ok(Self { dir, verbose, read_only, skin })
    }

    pub fn user_dir(&self, user_ref: &UserRef) -> PathBuf {
        self.dir.join("users").join(user_ref.to_string())
    }

    pub fn write<P, O>(&self, path: P, obj: &O) -> Result<()>
        where
            P: AsRef<Path>,
            O: Serialize,
    {
        let mut file = fs::File::create(path.as_ref())?;
        let json = serde_json::to_string_pretty(obj)?;
        write!(&mut file, "{}", json)?;
        if self.verbose {
            mad_print_inline!(
                self.skin,
                "User saved in *$0*\n",
                path.as_ref().to_string_lossy(),
            );
        }
        Ok(())
    }

    pub fn update_user(
        &self,
        user_ref: &UserRef,
        client: &Client,
    ) -> Result<()> {
        debug!("checking user {}", user_ref);
        let user_dir = self.user_dir(user_ref);
        fs::create_dir_all(&user_dir)?;
        let user = user_ref.get_user(client)?;
        self.write(
            user_dir.join("user.json"),
            &user,
        )?;
        if let Some(url) = &user.following {
            let following: Vec<String> = client.get_items(url)?;
            self.write(
                user_dir.join("following.json"),
                &following,
            )?;
        }
        if let Some(url) = &user.followers {
            let followers: Vec<String> = client.get_items(url)?;
            self.write(
                user_dir.join("followers.json"),
                &followers,
            )?;
        }
        if let Some(url) = &user.featured {
            let featured: Vec<Note> = client.get_items(url)?;
            self.write(
                user_dir.join("featured.json"),
                &featured,
            )?;
        }
        Ok(())
    }

    /// Update all users
    pub fn update(
        &self,
        conf: &Conf,
        thread_count: usize,
    ) -> Result<()> {
        let n = conf.watched.len();
        if n == 0 {
            eprintln!("No user followed. Use `fediback add @user@instance` to add one.");
            return Ok(());
        }
        print_progress(0, n)?;

        // a channel for the user_refs to process
        let (s_users, r_users) = channel::bounded(n);
        for user_ref in &conf.watched {
            s_users.send(user_ref).unwrap();
        }

        // a channel to receive progress
        let (s_progress, r_progress) = channel::bounded(n);

        // a thread to display progress
        std::thread::spawn(move || {
            let mut done = 0;
            while r_progress.recv().is_ok() {
                done += 1;
                print_progress(done, n).unwrap();
                if done == n { break; }
            }
        });

        // threads doing the heavy work
        thread::scope(|scope| {
            for _ in 0..thread_count {
                let r_users = r_users.clone();
                let s_progress = s_progress.clone();
                let mut client = match Client::new() {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("error while creating client: {:?}", e);
                        return;
                    }
                };
                client.verbose = self.verbose;
                scope.spawn(move |_| {
                    while let Ok(user_ref) = r_users.try_recv() {
                        self
                            .update_user(user_ref, &client)
                            .unwrap_or_else(|e| {
                                eprintln!("error while updating user: {:?}", e);
                            });
                        s_progress.send(()).unwrap();
                    }
                });
            }
        }).unwrap();

        eprintln!("                                              ");
        println!("{} users fetched and saved in {:?}", n, &self.dir);
        Ok(())
    }

}

fn print_progress(done: usize, total: usize) -> Result<()> {
    let width = 20;
    let p = ProgressBar::new(done as f32 / (total as f32), width);
    let s = format!("{:width$}", p, width=width);
    let mut stderr = io::stderr();
    queue!(stderr, cursor::SavePosition)?;
    queue!(stderr, Clear(ClearType::CurrentLine))?;
    queue!(stderr, Print(format!("{:>4} / {} users ", done, total)))?;
    queue!(stderr, PrintStyledContent(style(s).with(Color::Yellow).on(Color::DarkMagenta)))?;
    queue!(stderr, cursor::RestorePosition)?;
    stderr.flush()?;
    Ok(())
}
