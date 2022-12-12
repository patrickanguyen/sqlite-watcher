# SQLite Watcher

A simple TUI application used to view live updates of SQLite Database.

This application reads the [SQLite Schema Table](https://www.sqlite.org/schematab.html), so it can read any SQLite database without the user specifying the schema.

## How to use

To start SQLite Watcher,
```
cargo run <SQLITE_DB_PATH>
```

You can click the Left Arrow and Right Arrow to switch tables, and you can click the Up and Down Arrows to scroll through the table rows.

To quit, you can click Q or the ESC button.

## How to cross-compile

### Install Cross

Install [cross](https://github.com/cross-rs/cross), which allows you to easily cross-compile Rust applications to many different platforms.

### Compile SQLite Watcher

Compile SQLite Watcher with cross by specifying the [target triplet](https://github.com/cross-rs/cross#supported-targets).

From [OSDev.org](https://wiki.osdev.org/Target_Triplet), "Target Triplets describe a platform on which code runs and are a core concept in the GNU build system. They contain three fields: the name of the CPU family/model, the vendor, and the operating system name."

```
cross build --release --target <TARGET_TRIPLET>
```

For `aarch64-linux-android`, you can compile the application with the command

```
cross build --release --target aarch64-linux-android
```

Then, you can push SQLite Watcher to the target

```
adb push target/aarch64-linux-android/release/sqlite-watcher
```