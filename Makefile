MODE             ?= debug
BUILD_DIR        ?= build
TARGET_NAME      ?= x86-target
CARGO_TARGET_DIR ?= target
TARGET_DIR       ?= $(CARGO_TARGET_DIR)/$(TARGET_NAME)/$(MODE)
KERNEL           ?= $(TARGET_DIR)/baby_os
ISO_DIR          ?= $(BUILD_DIR)/isodir
ISO              ?= $(BUILD_DIR)/baby_os.iso
GRUBCFG          ?= tools/build/grub.cfg
QEMU             ?= qemu-system-i386
BUILD_TOOLS      ?= $(addprefix tools/build/, boot.s build.rs $(TARGET_NAME).json link.ld)
KERNEL_DEPS      := $(BUILD_TOOLS) $(shell find src -name '*.rs') $(shell find tests -name '*.rs')
BUILD_FLAGS      := -Zjson-target-spec

ifeq ($(MODE), release)
	BUILD_FLAGS += --release
endif

# Default target: builds and runs the Docker environment
.PHONY: all
all: exec

# Build the Docker image via Compose
.PHONY: docker-build
docker-build:
	docker compose build

# Start the dev container in the background (persistent caches stay warm)
.PHONY: up
up:
	docker compose up -d

# Stop and remove the dev container (named volumes are preserved)
.PHONY: down
down:
	docker compose down

# Drop into a shell: exec into the running container, or one-shot run if none
.PHONY: sh
sh:
	@if docker compose ps --services --filter status=running | grep -q '^dev$$'; then \
		docker compose exec dev /bin/sh; \
	else \
		docker compose run --rm --service-ports dev /bin/sh; \
	fi

# Back-compat: old `make docker-run` -> start container and drop into a shell
.PHONY: docker-run
docker-run: up sh

# Wipe persistent caches (toolchain + cargo target). Forces a cold rebuild.
.PHONY: docker-clean
docker-clean:
	docker compose down -v

# Convenience target to trigger the generation of the bootable ISO file
.PHONY: iso
iso: $(ISO)

# Convenience target to trigger the build of the Rust kernel binary
.PHONY: kernel
kernel: $(KERNEL)

# Compiles the kernel binary using Cargo if any source files or build tools change
$(KERNEL): $(KERNEL_DEPS)
	mkdir -p $(BUILD_DIR)
	cargo build $(BUILD_FLAGS)

# Constructs the GRUB filesystem structure and generates the bootable ISO image
$(ISO): $(KERNEL) $(GRUBCFG)
	rm -rf $(ISO_DIR)
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL) $(ISO_DIR)/boot/babyOS
	cp $(GRUBCFG) $(ISO_DIR)/boot/grub/grub.cfg
	grub-file --is-x86-multiboot $(ISO_DIR)/boot/babyOS
	grub-mkrescue -o $(ISO) $(ISO_DIR)

# Boots the generated ISO image using QEMU in curses mode (terminal display)
.PHONY: run
run: $(ISO)
	$(QEMU) -cdrom $(ISO) -m 512M -display curses

# Boots the ISO in QEMU, pausing at startup (-S) and opening a GDB stub on port 1234 (-s)
.PHONY: run-debug
run-debug: $(ISO)
	$(QEMU) -cdrom $(ISO) -m 512M -display curses -s -S

# Runs the test suite using Cargo
.PHONY: test
test:
	mkdir -p $(BUILD_DIR)
	$(MAKE) KERNEL=$(shell cargo test --no-run --message-format json | jq -r 'select(.executable != null) | .executable') run

# Re-invokes make, forcing the build mode to release for compiler optimizations
.PHONY: release
release:
	@$(MAKE) --no-print-directory MODE=release

# Installs required build dependencies via the provided shell script
.PHONY: deps
deps:
	tools/install_deps.sh

# Uninstalls build dependencies via the provided shell script
.PHONY: uninstall-deps
uninstall-deps:
	tools/uninstall_deps.sh

# Cleans up the project by removing the build directory and running cargo clean
.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)
	cargo clean

# Completely cleans the project and then builds and runs the OS fresh
.PHONY: re
re: clean run