
# Why fediback

A Mastodon instance can go down, and can die. This happens.

That's why users should always have essential information backuped on their own computer, most essentially their list of following and of followers.

With those lists, they can start a new account if necessary, even if their old one is unreachable, and follow again the same users, and notify people who were following them of their new account.


# Install

## From source

You must have the Rust environnement installed: https://rustup.rs

Run

```bash
cargo install fediback
```

## From precompiled binaries

Executables for Windows and Linux are included in [the releases published on GitHub](https://github.com/Canop/fediback/releases).

# Usage

![help](doc/help.png)

You first tell Fediback what account(s) you want to save, with `fediback add <user>` where user can be a user url or `@user@instance`:

![add](doc/add.png)

You can add who you want: only public data is fetched.

To backup all selected accounts, run `fediback update`:

![update](doc/update.png)

Accounts are fetched in parallel.

This operation can be done manually but there's no harm in having a cron rule if you can.

For each account 4 JSON files are saved on disk:

* `user.json` with the core account data
* `followers.json` with a list of users
* `following.json` with a list of users too
* `featured.json` with a list of "pinned" messages

![tree](doc/tree.png)

![user.json](doc/user.json.png)

![following.json](doc/following.json.png)

# Licence

MIT - Enjoy
