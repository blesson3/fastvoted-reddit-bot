SRC                    = $(shell find src    -type f -name '*.rs') Cargo.toml Cargo.lock
DEB_SRC                = $(shell find debian -type f)              Cargo.toml
DEBUG_BIN_RPI          = target/aarch64-unknown-linux-gnu/debug/fastvoted_reddit_bot
RELEASE_BIN_RPI_MAKETS = target/aarch64-unknown-linux-gnu/release/fastvoted_reddit_bot.makets
RELEASE_DEB_RPI        = target/aarch64-unknown-linux-gnu/debian/fastvoted_reddit_bot_0.1.0_arm64.deb
DEBUG_BIN              = target/debug/fastvoted_reddit_bot
RELEASE_BIN            = target/release/fastvoted_reddit_bot

# .PHONY: build-rpi-debug
# build-rpi-debug: $(SRC) $(RPI_DEBUG_BIN)
# 	cross build --bins --target aarch64-unknown-linux-gnu

# .PHONY: build-rpi-release
# build-rpi-release: $(SRC) $(RPI_RELEASE_BIN)
# 	cross build --bins --target aarch64-unknown-linux-gnu --release
# 	cargo deb --target aarch64-unknown-linux-gnu --no-build --no-strip --output rpi.deb

# .PHONY: build-debug
# build-debug: $(SRC) $(DEBUG_BIN)

# .PHONY: build-release
# build-release: $(SRC) $(RELEASE_BIN)
# 	cargo build --bins --release

# # only build the deb for rpi release
# # $(RPI_RELEASE_BIN):

# $(RPI_DEBUG_BIN): $(SRC)
# 	cargo build --bins

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

# debug-rpi

.PHONY: build-debug-rpi
build-debug-rpi: $(DEBUG_BIN_RPI)

$(DEBUG_BIN_RPI): $(SRC)
	cross build --bins --target aarch64-unknown-linux-gnu

# release-rpi

.PHONY: build-release-rpi
build-release-rpi: $(RELEASE_DEB_RPI)

$(RELEASE_BIN_RPI_MAKETS): $(SRC)
	cross build --bins --target aarch64-unknown-linux-gnu --release --locked
	touch $(RELEASE_BIN_RPI_MAKETS)

$(RELEASE_DEB_RPI): $(RELEASE_BIN_RPI_MAKETS) $(DEB_SRC)
	cargo deb --target aarch64-unknown-linux-gnu --no-build --no-strip
