BINARY_NAME		 ?= baby
BUILD_DIR        ?= build
TARGET_NAME      ?= x86-target
CARGO_TARGET_DIR ?= target
TARGET_DIR       ?= $(CARGO_TARGET_DIR)/$(TARGET_NAME)/debug
KERNEL           ?= $(TARGET_DIR)/$(BINARY_NAME)
ISO_DIR          ?= $(BUILD_DIR)/isodir
ISO              ?= $(BUILD_DIR)/$(BINARY_NAME).iso
GRUBCFG          ?= tools/build/grub.cfg
QEMU             ?= qemu-system-i386
QEMU_FLAGS		 := -cdrom $(ISO) -m 512M
BUILD_TOOLS      ?= $(addprefix tools/build/, boot.s build.rs $(TARGET_NAME).json link.ld)
KERNEL_DEPS      := $(BUILD_TOOLS) $(shell find src -name '*.rs')
BUILD_FLAGS      := -Zjson-target-spec

.PHONY: all
all: run

# Boots the generated ISO image using QEMU
.PHONY: run
run: iso
	$(QEMU) $(QEMU_FLAGS)

debug: iso
	$(QEMU) $(QEMU_FLAGS) -s -S -d int

# Build the ISO using a one-shot Docker container
.PHONY: iso
iso:
	docker compose run --build --rm dev

# Internal native ISO build target (used inside the container)
.PHONY: build-iso
build-iso: $(ISO)

# Compiles the kernel binary using Cargo
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

# Stop and remove the dev container (volumes are preserved)
.PHONY: down
down:
	docker compose down

# Wipe persistent caches (toolchain + cargo target) and docker compose cache. Forces a cold rebuild.
.PHONY: docker-clean
docker-clean:
	docker compose down -v --remove-orphans --rmi local
	podman image prune --build-cache -f
	docker builder prune -f
	podman image prune -f
	docker image prune -f

# Tests (disabled for now)
# # Boots the test kernel ISO headlessly, streaming COM1 to host stdio.
# # Invoked by `test` via a KERNEL= override so the ISO is built from the
# # cargo-test binary instead of the regular kernel.
# .PHONY: run-test
# run-test: $(ISO)
# 	$(QEMU) $(QEMU_TEST_FLAGS)

# # Runs the test suite using Cargo
# .PHONY: test
# test:
# 	mkdir -p $(BUILD_DIR)
# 	$(MAKE) KERNEL=$(shell cargo test --no-run --message-format json | jq -r 'select(.profile.test == true and .target.kind[] == "bin") | .executable') run-test

# Cleans up the project: wipes Docker caches/volumes, the build directory, and cargo artifacts
.PHONY: clean
clean: docker-clean
	rm -rf $(BUILD_DIR)
	rm -rf target

# Cold restart: wipe persistent caches (named volumes) then do a fresh ISO build
.PHONY: re
re: clean run
