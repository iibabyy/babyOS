MODE		?= debug
BUILD_DIR	:= build
TARGET_NAME	:= x86-target
TARGET_DIR	:= target/$(TARGET_NAME)/$(MODE)
KERNEL		?= $(TARGET_DIR)/baby_os
ISO_DIR		:= $(BUILD_DIR)/isodir
ISO			:= $(BUILD_DIR)/baby_os.iso
GRUBCFG		:= tools/build/grub.cfg
QEMU		:= qemu-system-i386

BUILD_TOOLS		:= $(addprefix tools/build/, boot.s build.rs $(TARGET_NAME).json link.ld)
KERNEL_DEPS = $(BUILD_TOOLS) $(shell find src -name '*.rs')

ifeq ($(MODE), release)
	BUILD_FLAGS := --release
else
	BUILD_FLAGS :=
endif

all: run

iso: $(ISO)

$(KERNEL): $(KERNEL_DEPS)
	mkdir -p $(BUILD_DIR)
	cargo build $(BUILD_FLAGS)

$(ISO): $(KERNEL) $(GRUBCFG)
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL) $(ISO_DIR)/boot/babyOS
	cp $(GRUBCFG) $(ISO_DIR)/boot/grub/grub.cfg
	grub-file --is-x86-multiboot $(ISO_DIR)/boot/babyOS
	grub-mkrescue --compress=xz -o $(ISO) $(ISO_DIR) --modules="normal multiboot part_msdos ext2"

run: $(ISO)
	$(QEMU) -cdrom $(ISO) -m 512M

release:
	$(MAKE) MODE=release

test:
	$(MAKE) KERNEL=$(shell cargo test --no-run --message-format=json | \
			jq -r 'select(.profile.test == true) | .executable')

deps:
	tools/install_deps.sh

uninstall-deps:
	tools/uninstall_deps.sh

clean:
	rm -rf $(BUILD_DIR)
	cargo clean

re: clean run

.PHONY: all iso run release deps clean re test