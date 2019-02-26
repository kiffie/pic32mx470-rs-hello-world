#
# Makefile for PIC32 native library for Rust runtime
#

CC=pic32-gcc
BIN2HEX = pic32-bin2hex
OBJDUMP = pic32-objdump

#CC = xc32-gcc
#BIN2HEX = xc32-bin2hex
#OBJDUMP = xc32-objdump

#PRG = usb-bootloader
LIB = libnative
TARGET = 32MX470F512H

CFLAGS = -Wall -g -mprocessor=$(TARGET) -Os -DSYS_CLOCK=96000000 -I../include -I../../libs/include

CFLAGS += -DUSB_DISABLE_IRQ
#CFLAGS += -save-temps

LDFLAGS = $(CFLAGS) -Wl,-Map=$(PRG).map,--memorysummary,$(PRG).mem,--defsym=_min_heap_size=4096,
LDFLAGS += -Wl,--script=../btl_32MX470F512H.ld -nostartfiles

SRCDIR = ..

OBJECTS =       crt0.o default_handlers.o config_mx470.o

DEPS    = $(patsubst %.o, %.d, $(notdir $(OBJECTS)))

VPATH := $(SRCDIR):../../libs:$(SRCDIR)/pico-crt

.PHONY: all
all: $(LIB).a


%.o: %.c
	$(CC) -c $(CFLAGS) -MMD -o $@_ $<
	bbe -e 'r 38 \x00' $@_ > $@
	rm -f $@_

%.o: %.S
	$(CC) -c $(CFLAGS) -MMD -o $@_ $<
	bbe -e 'r 38 \x00' $@_ > $@
	rm -f $@_

%.lst: %.elf
	$(OBJDUMP) -h -S $< > $@

#%.dep: %.c
#	@set -e; rm -f $@; \
#	$(CC) -MM $(CFLAGS) $< > $@.$$$$; \
#	sed 's,\($*\)\.o[ :]*,\1.o $@ : ,g' < $@.$$$$ > $@; \
#	rm -f $@.$$$$


$(LIB).a: $(OBJECTS)
	$(AR) crs $@ $^


lst: $(PRG).lst

clean:
	rm -f $(OBJECTS) $(DEPS) $(LIB).a

-include $(DEPS)
