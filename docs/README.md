# te-server

## Getting started

> For the following instructions, please don't use integrated terminals in IDEs, as they may not work as expected.

### Requirements

- [Rust](https://www.rust-lang.org/tools/install), version 1.65 or later.
- Sqlite3: See [here](#install-sqlite3) for installation instructions.
- `.env` file: create a `.env` file in the root of the project with the `DATABASE_URL` variable set to the path of the sqlite database file. Example: `DATABASE_URL=file:./data/sqlite.db`.
- Diesel: `cargo install diesel_cli --no-default-features --features sqlite`
- Initialize the database: follow the instructions in the [Diesel documentation](https://diesel.rs/guides/getting-started/). Replace `SERIAL PRIMARY KEY` in front of `id` with `INTEGER PRIMARY KEY AUTOINCREMENT`.

### Contributing

> IMPORTANT: When you choose a folder for your database file, please make sure it is in the `.gitignore`, or otherwise, add it. We recommend using the `data` folder.

## Appendix

### Install sqlite3

#### On Windows

In any shell, run (normally, elevated won't be necessary):

```shell
choco install sqlite
```

If you don't have `choco`, you can install the package from the [official website](https://www.sqlite.org/download.html).

It will print the path where `sqlite3` was installed. Copy this path. If the path doesn't end with the folder `tools`, please add it to the given path.

Then, open `Developper Command Prompt for Visual Studio` (not powershell) and go to the path where sqlite3 was installed, using `cd`. After that, find `vcvars64.bat` and run it. It should look like something like this:

```shell
C:\ProgramData\chocolatey\lib\SQLite\tools> call "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
```

Then, run

```shell
lib /machine:x64 /def:sqlite3.def /out:sqlite3.lib
```

and at the current path to the `SQLITE3_LIB_DIR` environment variable and the copied path to then `ENV` environement variable. Then close the terminal app and open another shell (any other shell) to continue.

#### Others

Please add your instructions here.
