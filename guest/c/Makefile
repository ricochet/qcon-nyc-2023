DEPS = 
OBJ = critter.o
OUTPUT = critter.wasm

COMPILE_FLAGS = -Wall \
		--target=wasm32 \
		-Os \
		-nostdlib \
		-fvisibility=hidden \
		-ffunction-sections \
		-fdata-sections \
		-DPRINTF_DISABLE_SUPPORT_FLOAT=1 \
		-DPRINTF_DISABLE_SUPPORT_LONG_LONG=1 \
		-DPRINTF_DISABLE_SUPPORT_PTRDIFF_T=1


%.o: %.c $(DEPS) Makefile critter.h
	/opt/wasi-sdk/bin/clang \
		-c \
		$(COMPILE_FLAGS) \
		-o $@ \
		$<


