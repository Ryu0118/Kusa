SOURCE_FILE = ./src/main.rs

release_mac_universal: $(SOURCE_FILE)
	cargo build --release --target aarch64-apple-darwin
	cargo build --release --target x86_64-apple-darwin
	lipo -create -output ./release/kusa ./target/aarch64-apple-darwin/release/kusa ./target/x86_64-apple-darwin/release/kusa
	tar acvf release/kusa_mac_universal.tar.gz release/kusa

release_linux: $(SOURCE_FILE)
	cross build --release --target aarch64-unknown-linux-gnu
	cross build --release --target x86_64-unknown-linux-gnu
	cp ./target/aarch64-unknown-linux-gnu/release/kusa kusa
	tar acvf release/kusa_aarch64_linux.tar.gz kusa
	rm kusa
	cp ./target/x86_64-unknown-linux-gnu/release/kusa kusa
	tar acvf release/kusa_x86_64_linux.tar.gz kusa
	rm kusa

release_windows: $(SOURCE_FILE)
	cross build --release --target x86_64-pc-windows-gnu
	cross build --release --target i686-pc-windows-gnu
	cp ./target/x86_64-pc-windows-gnu/release/kusa.exe kusa.exe
	zip release/kusa_x86_64-windows.zip kusa.exe
	rm kusa.exe
	cp ./target/i686-pc-windows-gnu/release/kusa.exe kusa.exe
	zip release/kusa_i686-windows.zip kusa.exe
	rm kusa.exe

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

.PHONY: clean
clean:
	rm -rf ./target
	rm -rf ./release