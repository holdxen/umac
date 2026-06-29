CC ?= gcc
CFLAGS ?= -Wall -O2 -I.
AR ?= ar

OBJS = rijndael.o umac.o umac128.o
LIB = libumac.a

all: $(LIB)

$(LIB): $(OBJS)
	$(AR) rcs $@ $^

rijndael.o: rijndael.c rijndael.h
	$(CC) $(CFLAGS) -c $< -o $@

umac.o: umac.c umac.h umac_utils.h rijndael.h
	$(CC) $(CFLAGS) -c $< -o $@

umac128.o: umac128.c umac.c umac.h umac_utils.h rijndael.h
	$(CC) $(CFLAGS) -DUMAC_OUTPUT_LEN=16 -c umac.c -o $@

clean:
	rm -f $(OBJS) $(LIB)

.PHONY: all clean
