#ifndef UMAC_UTILS_H
#define UMAC_UTILS_H

#include <stdint.h>
#include <string.h>

static inline uint32_t get_u32(const void *vp)
{
	const uint8_t *p = (const uint8_t *)vp;
	uint32_t v;

	v  = (uint32_t)p[0] << 24;
	v |= (uint32_t)p[1] << 16;
	v |= (uint32_t)p[2] << 8;
	v |= (uint32_t)p[3];

	return (v);
}

static inline void put_u32(void *vp, uint32_t v)
{
	uint8_t *p = (uint8_t *)vp;

	p[0] = (uint8_t)(v >> 24) & 0xff;
	p[1] = (uint8_t)(v >> 16) & 0xff;
	p[2] = (uint8_t)(v >> 8) & 0xff;
	p[3] = (uint8_t)v & 0xff;
}

static inline uint32_t get_u32_le(const void *vp)
{
	const uint8_t *p = (const uint8_t *)vp;
	uint32_t v;

	v  = (uint32_t)p[0];
	v |= (uint32_t)p[1] << 8;
	v |= (uint32_t)p[2] << 16;
	v |= (uint32_t)p[3] << 24;

	return (v);
}

static inline void put_u32_le(void *vp, uint32_t v)
{
	uint8_t *p = (uint8_t *)vp;

	p[0] = (uint8_t)v & 0xff;
	p[1] = (uint8_t)(v >> 8) & 0xff;
	p[2] = (uint8_t)(v >> 16) & 0xff;
	p[3] = (uint8_t)(v >> 24) & 0xff;
}

#endif /* UMAC_UTILS_H */
