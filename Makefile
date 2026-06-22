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
KERNEL_DEPS      := $(BUILD_TOOLS) $(shell find src -name '*.rs')
BUILD_FLAGS      := -Zjson-target-spec

ifeq ($(MODE), release)
	BUILD_FLAGS += --release
endif

# Default target: builds and runs the Docker environment
.PHONY: all
all: up

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
.PHONY: exec
exec:
	@if docker compose ps --services --filter status=running | grep -q '^dev$$'; then \
		docker compose exec dev /bin/sh; \
	else \
		docker compose run --rm --service-ports dev /bin/sh; \
	fi

# Back-compat: old `make docker-run` -> start container and drop into a shell
.PHONY: docker-run
docker-run: up exec

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

# Common QEMU flags shared by every boot target
QEMU_BASE_FLAGS  := -cdrom $(ISO) -m 512M

# Interactive boot: curses VGA, no host serial mirroring
QEMU_RUN_FLAGS   := $(QEMU_BASE_FLAGS) -display curses

# Headless boot for tests: no display, COM1 mirrored to host stdio so
# serial_println! output streams to the terminal that invoked `make test`
QEMU_TEST_FLAGS  := $(QEMU_BASE_FLAGS) -display none -serial stdio

# Boots the generated ISO image using QEMU in curses mode (terminal display)
.PHONY: run
run: $(ISO)
	$(QEMU) $(QEMU_RUN_FLAGS)

# Boots the ISO in QEMU, pausing at startup (-S) and opening a GDB stub on port 1234 (-s)
.PHONY: run-debug
run-debug: $(ISO)
	$(QEMU) $(QEMU_RUN_FLAGS) -s -S

# Boots the test kernel ISO headlessly, streaming COM1 to host stdio.
# Invoked by `test` via a KERNEL= override so the ISO is built from the
# cargo-test binary instead of the regular kernel.
.PHONY: run-test
run-test: $(ISO)
	$(QEMU) $(QEMU_TEST_FLAGS)

# Runs the test suite using Cargo
.PHONY: test
test:
	mkdir -p $(BUILD_DIR)
	$(MAKE) KERNEL=$(shell cargo test --no-run --message-format json | jq -r 'select(.profile.test == true and .target.kind[] == "bin") | .executable') run-test

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

# Cleans up the project: wipes Docker caches/volumes, the build directory, and cargo artifacts
.PHONY: clean
clean: docker-clean
	rm -rf $(BUILD_DIR)
	cargo clean

# Cold restart: wipe persistent caches (named volumes) then start a fresh container
.PHONY: re
re: clean up