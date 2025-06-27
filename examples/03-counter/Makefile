# EpicChain Rust Smart Contract Makefile
# Simple and reliable build automation

# Project Configuration
PROJECT_NAME := $(shell basename $(CURDIR))
CARGO_TARGET := wasm32-unknown-unknown
BUILD_MODE := release

# Directories and Files
TARGET_DIR := ../../target/$(CARGO_TARGET)/$(BUILD_MODE)
BUILD_DIR := build
WASM_FILE := $(TARGET_DIR)/$(shell grep '^name = ' Cargo.toml | cut -d'"' -f2 | tr '-' '_').wasm
NEF_FILE := $(BUILD_DIR)/$(PROJECT_NAME).nef
MANIFEST_FILE := $(BUILD_DIR)/$(PROJECT_NAME).manifest.json

# Tools
CARGO := cargo
EPICCHAIN_WASM := ../../epicchain-wasm/epicchain-wasm
RUSTUP := rustup

# Build flags
RUSTFLAGS := -Ctarget-feature=+multivalue -Clink-arg=--initial-memory=2097152
CARGO_FLAGS := --target $(CARGO_TARGET) --$(BUILD_MODE)

# Colors
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[1;33m
BLUE := \033[0;34m
NC := \033[0m

.PHONY: all compile wasm nef manifest clean check test help install-deps info

# Default target
all: nef manifest

# Help
help:
	@echo "$(BLUE)EpicChain Rust Smart Contract Build System$(NC)"
	@echo "Available targets: all, compile, wasm, nef, manifest, clean, check, test, help, info"

# Install dependencies
install-deps:
	@echo "$(YELLOW)Installing dependencies...$(NC)"
	@$(RUSTUP) target add $(CARGO_TARGET)
	@echo "$(GREEN)Dependencies installed!$(NC)"

# Check code
check:
	@echo "$(YELLOW)Checking $(PROJECT_NAME)...$(NC)"
	@RUSTFLAGS="$(RUSTFLAGS)" $(CARGO) check $(CARGO_FLAGS)
	@echo "$(GREEN)Check completed!$(NC)"

# Run tests
test:
	@echo "$(YELLOW)Running tests...$(NC)"
	@$(CARGO) test
	@echo "$(GREEN)Tests completed!$(NC)"

# Compile to WASM
compile:
	@echo "$(YELLOW)Compiling $(PROJECT_NAME) to WASM...$(NC)"
	@$(RUSTUP) target add $(CARGO_TARGET) || true
	@RUSTFLAGS="$(RUSTFLAGS)" $(RUSTUP) run nightly $(CARGO) build $(CARGO_FLAGS)
	@echo "$(GREEN)✅ Compilation completed!$(NC)"

# Verify WASM
wasm: compile
	@echo "$(YELLOW)Verifying WASM file...$(NC)"
	@if [ -f "$(WASM_FILE)" ]; then \
		echo "$(GREEN)✅ WASM file found: $(WASM_FILE)$(NC)"; \
		ls -la "$(WASM_FILE)"; \
	else \
		FOUND_WASM=$$(find ../../target -name "*.wasm" -type f | head -1); \
		if [ -n "$$FOUND_WASM" ]; then \
			echo "$(GREEN)✅ WASM file found: $$FOUND_WASM$(NC)"; \
		else \
			echo "$(RED)❌ No WASM file found!$(NC)"; \
			exit 1; \
		fi; \
	fi

# Generate NEF
nef: wasm
	@mkdir -p $(BUILD_DIR)
	@echo "$(YELLOW)Generating NEF file...$(NC)"
	@WASM_PATH="$(WASM_FILE)"; \
	if [ -z "$$WASM_PATH" ]; then \
		echo "$(RED)❌ No WASM file found!$(NC)"; \
		exit 1; \
	fi; \
	echo "$(YELLOW)Using WASM: $$WASM_PATH$(NC)"; \
	if [ -f "$(EPICCHAIN_WASM)" ]; then \
		$(EPICCHAIN_WASM) translate --input "$$WASM_PATH" --output $(NEF_FILE) --source-code src/lib.rs && \
		echo "$(GREEN)✅ NEF generated: $(NEF_FILE)$(NC)" || \
		($(EPICCHAIN_WASM) translate --input "$$WASM_PATH" --output $(NEF_FILE) && \
		echo "$(GREEN)✅ NEF generated: $(NEF_FILE)$(NC)"); \
	else \
		echo "$(RED)❌ epicchain-wasm not found!$(NC)"; \
		exit 1; \
	fi

# Generate manifest
manifest: wasm
	@mkdir -p $(BUILD_DIR)
	@echo "$(YELLOW)Generating manifest file...$(NC)"
	@WASM_PATH="$(WASM_FILE)"; \
	if [ -z "$$WASM_PATH" ]; then \
		echo "$(RED)❌ No WASM file found!$(NC)"; \
		exit 1; \
	fi; \
	echo "$(YELLOW)Using WASM: $$WASM_PATH$(NC)"; \
	if [ -f "$(EPICCHAIN_WASM)" ]; then \
		$(EPICCHAIN_WASM) translate --input "$$WASM_PATH" --manifest $(MANIFEST_FILE) --source-code src/lib.rs && \
		echo "$(GREEN)✅ Manifest generated: $(MANIFEST_FILE)$(NC)" || \
		($(EPICCHAIN_WASM) translate --input "$$WASM_PATH" --manifest $(MANIFEST_FILE) && \
		echo "$(GREEN)✅ Manifest generated: $(MANIFEST_FILE)$(NC)"); \
	else \
		echo "$(RED)❌ epicchain-wasm not found!$(NC)"; \
		exit 1; \
	fi

# Clean
clean:
	@echo "$(YELLOW)Cleaning...$(NC)"
	@$(CARGO) clean
	@rm -rf $(BUILD_DIR)
	@echo "$(GREEN)✅ Clean completed!$(NC)"

# Info
info:
	@echo "$(BLUE)Build Information$(NC)"
	@echo "Project: $(PROJECT_NAME)"
	@echo "Target: $(CARGO_TARGET)"
	@echo "Mode: $(BUILD_MODE)"
	@echo "WASM: $(WASM_FILE)"
	@echo "NEF: $(NEF_FILE)"
	@echo "Manifest: $(MANIFEST_FILE)"
