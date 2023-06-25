SOURCE_FILE := ./src/main.rs

VERSION := 0.1.0

APP := kusa

RELEASE_BUILD := cargo build --release --target
CROSS_RELEASE_BUILD := cross build --release --target

release_mac_universal: $(SOURCE_FILE)
	$(RELEASE_BUILD) aarch64-apple-darwin
	$(RELEASE_BUILD) x86_64-apple-darwin
	lipo -create -output ./release/$(APP) ./target/aarch64-apple-darwin/release/$(APP) ./target/x86_64-apple-darwin/release/$(APP)
	tar acvf release/$(APP)_$(VERSION)_mac_universal.tar.gz release/$(APP)

release_linux: $(SOURCE_FILE)
	$(CROSS_RELEASE_BUILD) aarch64-unknown-linux-gnu
	$(CROSS_RELEASE_BUILD) x86_64-unknown-linux-gnu
	cp ./target/aarch64-unknown-linux-gnu/release/$(APP) $(APP)
	tar acvf release/$(APP)_$(VERSION)_aarch64_linux.tar.gz $(APP)
	rm $(APP)
	cp ./target/x86_64-unknown-linux-gnu/release/$(APP) $(APP)
	tar acvf release/$(APP)_$(VERSION)_x86_64_linux.tar.gz $(APP)
	rm $(APP)

release_windows: $(SOURCE_FILE)
	$(CROSS_RELEASE_BUILD) x86_64-pc-windows-gnu
	$(CROSS_RELEASE_BUILD) i686-pc-windows-gnu
	cp ./target/x86_64-pc-windows-gnu/release/$(APP).exe $(APP).exe
	zip release/$(APP)_$(VERSION)_x86_64-windows.zip $(APP).exe
	rm $(APP).exe
	cp ./target/i686-pc-windows-gnu/release/$(APP).exe $(APP).exe
	zip release/$(APP)_$(VERSION)_i686-windows.zip $(APP).exe
	rm $(APP).exe

.PHONY: release
release:
	mkdir -p release
	make release_mac_universal --no-print-directory
	make release_linux --no-print-directory
	make release_windows --no-print-directory

git_add: $(SOURCE_FILE)
	sed -i -e s/$${GITHUB_ACCESS_TOKEN}/GITHUB_ACCESS_TOKEN/ $(SOURCE_FILE)
	rm ./src/main.rs-e
	@if !(cat $(SOURCE_FILE) | grep $${GITHUB_ACCESS_TOKEN}); then \
		git add .;\
		echo changed to be staged; \
	else \
		echo token is included in the code; \
	fi

git_push: $(SOURCE_FILE)
	@if !(cat $(SOURCE_FILE) | grep $${GITHUB_ACCESS_TOKEN}); then \
		git push origin main; \
		sed -i -e s/GITHUB_ACCESS_TOKEN/$${GITHUB_ACCESS_TOKEN}/ $(SOURCE_FILE); \
		rm ./src/main.rs-e; \
	else \
		echo token is included in the code; \
	fi

.PHONY: format
format:
	cargo fmt

.PHONY: clean
clean:
	rm -rf ./target
	rm -rf ./release
	rm Cargo.lock
