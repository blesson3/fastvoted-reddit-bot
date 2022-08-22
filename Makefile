SRC                    = $(shell find src    -type f -name '*.rs') Cargo.toml Cargo.lock
DEB_SRC                = $(shell find debian -type f)              Cargo.toml
DEBUG_BIN_ARMv7          = target/armv7-unknown-linux-musleabihf/debug/fastvoted_reddit_bot
RELEASE_BIN_ARMv7_MAKETS = target/armv7-unknown-linux-musleabihf/release/fastvoted_reddit_bot.makets
RELEASE_DEB_ARMv7        = target/armv7-unknown-linux-musleabihf/debian/fastvoted_reddit_bot_0.1.0_armv7.deb
DEBUG_BIN              = target/debug/fastvoted_reddit_bot
RELEASE_BIN            = target/release/fastvoted_reddit_bot

# debug

.PHONY: build-debug
build-debug: $(DEBUG_BIN)

$(DEBUG_BIN): $(SRC)
	cargo build --bins

# release

.PHONY: build-release
build-release: $(RELEASE_BIN)

$(RELEASE_BIN): $(SRC)
	cargo build --bins --release --locked

# debug-armv7

.PHONY: build-debug-armv7
build-debug-armv7: $(DEBUG_BIN_ARMv7)

$(DEBUG_BIN_ARMv7): $(SRC)
	cross build --bins --target armv7-unknown-linux-musleabihf

# release-armv7

.PHONY: build-release-armv7
build-release-armv7: $(RELEASE_DEB_ARMv7)

$(RELEASE_BIN_ARMv7_MAKETS): $(SRC)
	cross build --bins --target armv7-unknown-linux-musleabihf --release --locked
	touch $(RELEASE_BIN_ARMv7_MAKETS)

$(RELEASE_DEB_ARMv7): $(RELEASE_BIN_ARMv7_MAKETS) $(DEB_SRC)
	cargo deb --target armv7-unknown-linux-musleabihf --no-build --no-strip
