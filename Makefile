# Makefile for building and testing the Rust version of 9cc

# 実行ファイル名（任意で変更可能）
TARGET=rusty_cc

# Build the Rust project
$(TARGET):
	cargo build --release
	cp target/release/$(TARGET) .

# Run tests
test: $(TARGET)
	./test.sh

# Clean build artifacts
clean:
	cargo clean
	rm -f $(TARGET) *~

.PHONY: test clean

