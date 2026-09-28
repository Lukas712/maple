ARCH := x86_64
IMAGE_NAME := maple
TARGET := x86_64-unknown-none
KERNEL_BIN := target/$(TARGET)/release/maple

.PHONY: all
all: $(IMAGE_NAME).iso

# 1. Compila o Kernel Rust usando Cargo (com alvo bare-metal)
$(KERNEL_BIN): src/init/main.rs Cargo.toml linker.ld
	cargo build --target $(TARGET) --release

# 2. Baixa e compila o utilitário binário do Limine para criar a ISO
.INTERMEDIATE: limine-binary.tar.gz
limine-binary.tar.gz:
	curl -fL -o $@ https://github.com/Limine-Bootloader/Limine/releases/latest/download/limine-binary.tar.gz

limine-binary/limine: limine-binary.tar.gz
	rm -rf limine-binary
	gunzip < limine-binary.tar.gz | tar -xf -
	$(MAKE) -C limine-binary

# 3. Empacota tudo em uma imagem ISO bootável
$(IMAGE_NAME).iso: limine-binary/limine $(KERNEL_BIN)
	rm -rf iso_root
	mkdir -p iso_root/boot/limine
	cp $(KERNEL_BIN) iso_root/boot/maple.elf
	cp limine.conf iso_root/
	cp limine-binary/limine-bios.sys limine-binary/limine-bios-cd.bin limine-binary/limine-uefi-cd.bin iso_root/boot/limine/
	mkdir -p iso_root/EFI/BOOT
	cp limine-binary/BOOTX64.EFI limine-binary/BOOTIA32.EFI iso_root/EFI/BOOT/
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot boot/limine/limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		iso_root -o $(IMAGE_NAME).iso
	./limine-binary/limine bios-install $(IMAGE_NAME).iso
	rm -rf iso_root

# 4. Roda no QEMU redirecionando a serial para o seu terminal
.PHONY: run
run: $(IMAGE_NAME).iso
	qemu-system-$(ARCH) \
		-M q35 \
		-m 512M \
		-cdrom $(IMAGE_NAME).iso \
		-serial stdio \
		-no-reboot \
		-no-shutdown

# Limpeza dos artefatos
.PHONY: clean
clean:
	cargo clean
	rm -rf iso_root *.iso limine-binary limine-binary.tar.gz