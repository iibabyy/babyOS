BUILD_DIR        ?= build
TARGET_NAME      ?= x86-target
CARGO_TARGET_DIR ?= target
TARGET_DIR       ?= $(CARGO_TARGET_DIR)/$(TARGET_NAME)/debug
KERNEL           ?= $(TARGET_DIR)/baby_os
ISO_DIR          ?= $(BUILD_DIR)/isodir
ISO              ?= $(BUILD_DIR)/baby_os.iso
GRUBCFG          ?= tools/build/grub.cfg
QEMU             ?= qemu-system-i386
QEMU_FLAGS		 := -cdrom $(ISO) -m 512M
BUILD_TOOLS      ?= $(addprefix tools/build/, boot.s build.rs $(TARGET_NAME).json link.ld)
KERNEL_DEPS      := $(BUILD_TOOLS) $(shell find src -name '*.rs')
BUILD_FLAGS      := -Zjson-target-spec

# Default target: builds the ISO via Docker
.PHONY: all
all: iso

# Build the ISO using a one-shot Docker container
.PHONY: iso
iso:
	docker compose run --rm dev

# Internal native ISO build target (used inside the container via CMD)
.PHONY: build-iso
build-iso: $(ISO)

# Stop and remove the dev container (volumes are preserved)
.PHONY: down
down:
	docker compose down

# Wipe persistent caches (toolchain + cargo target). Forces a cold rebuild.
.PHONY: docker-clean
docker-clean:
	docker compose down -v

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
run: iso
	$(QEMU) $(QEMU_FLAGS)

# Boots the ISO in QEMU, pausing at startup (-S) and opening a GDB stub on port 1234 (-s)
.PHONY: run-debug

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
debug: iso
	$(QEMU) $(QEMU_FLAGS) -s -S

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

# Cold restart: wipe persistent caches (named volumes) then do a fresh ISO build
.PHONY: re
re: clean iso