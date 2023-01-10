OS_DIR=target/riscv64gc-unknown-none-elf/release
OS=${OS_DIR}/os
OS_BIN=${OS}.bin
BOOT_LOADER=../bootloader/rustsbi-qemu.bin
SKERNEL=0x80200000

user_bin:
	@cd ../user && make build
	
os: user_bin
	cargo build --release
os.bin: os
	rust-objcopy --strip-all ${OS} -O binary ${OS_BIN}

link_app.asm:
	cargo build
	
run: os.bin link_app.asm
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ${BOOT_LOADER} \
		-device loader,file=${OS_BIN},addr=${SKERNEL}

gdbserver: os.bin
	qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios ${BOOT_LOADER} \
		-device loader,file=${OS_BIN},addr=${SKERNEL} \
		-s -S

gdbclient:
	riscv64-unknown-elf-gdb \
		-ex 'file ${OS}' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234'

nm:
	riscv64-unknown-elf-nm ${OS}