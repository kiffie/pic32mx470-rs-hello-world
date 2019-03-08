#
# Makefile for PIC32 native library for Rust runtime
#

CC=pic32-gcc
BIN2HEX = pic32-bin2hex
OBJDUMP = pic32-objdump

LIB = libnative
TARGET = 32MX470F512H

CFLAGS = -Wall -g -mprocessor=$(TARGET) -Os

OBJECTS = crt0.o isr_context.o excpetion_table.o mips_irq.o general-exception.o

DEPS    = $(patsubst %.o, %.d, $(notdir $(OBJECTS)))

.PHONY: all
all: $(LIB).a


%.o: %.c
	$(CC) -c $(CFLAGS) -MMD -o $@_ $<
	bbe -e 'r 38 \x00' $@_ > $@
	rm -f $@_

# bbe is used to fix the ELF header
%.o: %.S
	$(CC) -c $(CFLAGS) -MMD -o $@_ $<
	bbe -e 'r 38 \x00' $@_ > $@
	rm -f $@_

$(LIB).a: $(OBJECTS)
	$(AR) crs $@ $^

clean:
	rm -f $(OBJECTS) $(DEPS)

mrpropoer: clean
	rm -f $(LIB).a

-include $(DEPS)
