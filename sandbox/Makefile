BUILD_PATH = "${PWD}/../target/thumbv6m-none-eabi/release"
TARGET = "${BUILD_PATH}/sandbox"
OBJCOPY = "arm-none-eabi-objcopy"
OBJDUMP = "arm-none-eabi-objdump"

.PHONY: all
all: ${TARGET}.bin ${TARGET}.hex ${TARGET}.lst

.PHONY: clean
clean:
	cargo clean --release

${TARGET}:
	cargo build --release

${TARGET}.bin: ${TARGET}
	${OBJCOPY} -O binary -S $< $@

${TARGET}.hex: ${TARGET}
	${OBJCOPY} -O ihex -S $< $@

${TARGET}.lst: ${TARGET}
	${OBJDUMP} -d $< > $@

.PHONY: flash
flash: ${TARGET}.bin
	dfu-util -d 1209:70b1 -D ${TARGET}.bin
