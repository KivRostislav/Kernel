BUILD_DIR=$(CURDIR)/build
ARTIFACTS_DIR=$(CURDIR)/artifacts
TESTS_DIR = $(CURDIR)/tests

TARGET=x86_64-unknown-uefi

.PHONY: build pack

build:
	cargo build --target-dir $(ARTIFACTS_DIR)/kernel --release --target $(TARGET)

test:
	cargo test --target-dir $(ARTIFACTS_DIR)/tests --release --target $(TARGET)

run: pack
	qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -drive format=raw,file=./build/kernel_image.iso

pack: build
	mkdir -p $(BUILD_DIR)
	mkdir -p $(ARTIFACTS_DIR)/mnt/
	touch $(BUILD_DIR)/kernel_image.iso
	dd if=/dev/zero of=$(BUILD_DIR)/kernel_image.iso bs=48000000 count=1
	parted $(BUILD_DIR)/kernel_image.iso -s -a minimal mklabel gpt
	parted $(BUILD_DIR)/kernel_image.iso -s -a minimal mkpart EFI FAT32 2048s 93716s
	parted $(BUILD_DIR)/kernel_image.iso -s -a minimal toggle 1 boot
	sudo losetup --offset 1048576 --sizelimit 46934528 /dev/loop0 $(BUILD_DIR)/kernel_image.iso
	sudo mkdosfs -F 32 /dev/loop0
	sudo mount /dev/loop0 $(ARTIFACTS_DIR)/mnt
	sudo mkdir -p $(ARTIFACTS_DIR)/mnt/efi/boot/
	sudo cp $(ARTIFACTS_DIR)/kernel/$(TARGET)/release/kernel.efi $(ARTIFACTS_DIR)/mnt/efi/boot/bootx64.efi
	sudo umount $(ARTIFACTS_DIR)/mnt
	sudo losetup -d /dev/loop0
