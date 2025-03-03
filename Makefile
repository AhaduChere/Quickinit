# Try to detect the git root directory; if not found, default to current directory.
GIT_ROOT := $(shell git rev-parse --show-toplevel 2>/dev/null)
ifeq ($(GIT_ROOT),)
	PROJECT_ROOT := $(CURDIR)
else
	PROJECT_ROOT := $(GIT_ROOT)
endif

# Use the folder name at the project root as the project name.
PROJECT_NAME := $(notdir $(PROJECT_ROOT))
TARGET_DIR := $(PROJECT_ROOT)/target/debug

# Default target: build the project using Cargo.
all:
	cargo build
	@echo "Built executable: $(TARGET_DIR)/$(PROJECT_NAME)"

# Clean build artifacts.
clean:
	cargo clean
