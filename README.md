# fastvoted-reddit-bot

this is a reddit bot written in rust that will post to 
[/r/fastvoted](https://www.reddit.com/r/fastvoted/). it's run on a raspberry pi.

## motivation

[fastvoted.com](https://fastvoted.com/) is a slack bot that will post trending 
tech and startup posts from various sources to a slack channel. I wanted a way 
to see this news feed through another medium (like reddit).

I also wanted a way to test my knowledge of rust, deb, syslog, apis, 
cross-compilation, and makefiles.

## how it works

every 15 minutes the bot will hit the fastvoted.com website (where it shows a 
preview of news it would post to a slack channel). it scrapes the data and then
posts it to the subreddit.

## install

the rust code is cross-compiled using the 
[cross](https://github.com/rust-embedded/cross) project. cross must be 
installed. this project uses the nightly toolchain.

```
# install cross (if not already)
$ cargo install cross

# cross compile and generate deb file for linux install
$ make build-release-rpi

# copy the file to the target and install
$ rsync -avr target/aarch64-unknown-linux-gnu/debian/fastvoted_reddit_bot_*_arm64.deb ubuntu@target-ip:/tmp
$ ssh ubuntu@target-ip sudo dpkg -i /tmp/fastvoted_reddit_bot_*_arm64.deb
$ ssh ubuntu@target-ip rm /tmp/fastvoted_reddit_bot_*_arm64.deb
```
