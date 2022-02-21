# fivebot 

## Database changes
```sh 
$ sqlx migrate revert
$ psql -U postgres -d dev
> DROP table _sqlx_migrations;
> \q
$ sqlx migrate run
```

## Cross Compilation for Raspberry PI
Change database, change global/client commands
```sh
$ sudo /etc/init.d/postgresql start
$ export OPUS_LIB_DIR=/mnt/c/Users/5joshi/libopus/
$ export OPUS_NO_PKG=1 
$ export OPUS_STATIC=1
$ cargo build --release --features vendored-openssl --target arm-unknown-linux-gnueabihf
$ scp /target/arm-unknown-linux-gnueabihf/release/fivebot pi@192.168.1.103:/home/pi/Desktop
```

## Running on Raspberry PI
```sh
$ ssh pi@192.168.1.103
$ tmux new -s fivebot
$ cd Desktop
$ ./fivebot
# ctrl+B - D -- to leave tmux session
# tmux attach -t fivebot -- to enter tmux session
```