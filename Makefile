MODE		?= debug
BUILD_DIR	:= build
TARGET_NAME	:= x86-target
CARGO_TARGET_DIR ?= target
TARGET_DIR	:= $(CARGO_TARGET_DIR)/$(TARGET_NAME)/$(MODE)
KERNEL		?= $(TARGET_DIR)/baby_os
ISO_DIR		:= $(BUILD_DIR)/isodir
ISO			:= $(BUILD_DIR)/baby_os.iso
GRUBCFG		:= tools/build/grub.cfg
QEMU		:= qemu-system-i386

BUILD_TOOLS		:= $(addprefix tools/build/, boot.s build.rs $(TARGET_NAME).json link.ld)
KERNEL_DEPS = $(BUILD_TOOLS) $(shell find src -name '*.rs') $(shell find tests -name '*.rs')

BUILD_FLAGS := -Zjson-target-spec

ifeq ($(MODE), release)
	BUILD_FLAGS += --release
endif

all: docker

debug:
	docker build . -t babyos
	docker run -it -p 1234:1234 -v ./target:/workspace/target babyos make run-debug

docker:
	docker build . -t babyos
	docker run -it babyos

iso: $(ISO)

kernel: $(KERNEL)

$(KERNEL): $(KERNEL_DEPS)
	mkdir -p $(BUILD_DIR)
	cargo build $(BUILD_FLAGS)

$(ISO): $(KERNEL) $(GRUBCFG)
	rm -rf $(ISO_DIR)
	mkdir -p $(ISO_DIR)/boot/grub
	cp $(KERNEL) $(ISO_DIR)/boot/babyOS
	cp $(GRUBCFG) $(ISO_DIR)/boot/grub/grub.cfg
	grub-file --is-x86-multiboot $(ISO_DIR)/boot/babyOS
	grub-mkrescue -o $(ISO) $(ISO_DIR)

run: $(ISO)
	$(QEMU) -cdrom $(ISO) -m 512M -display curses

run-debug: $(ISO)
	$(QEMU) -cdrom $(ISO) -m 512M -display curses -s -S

release:
	@$(MAKE) --no-print-directory MODE=release

# Tests are not working for now
# test:
# 	@$(MAKE) --no-print-directory KERNEL=$(shell \
# 		cargo test --no-run --message-format=json | \
# 		jq -r 'select(.profile.test == true) | .executable' \
# 	)

deps:
	tools/install_deps.sh

uninstall-deps:
	tools/uninstall_deps.sh

clean:
	rm -rf $(BUILD_DIR)
	cargo clean

re: clean run

.PHONY: all iso run run-debug release deps clean re kernel debug
