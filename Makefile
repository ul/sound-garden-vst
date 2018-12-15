bundle:
	cargo build --release
	./osx_vst_bundler.sh SoundGarden target/release/libsoundgarden.dylib
