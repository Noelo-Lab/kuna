extern unsigned observe(unsigned char *, unsigned);
extern unsigned observe_aligned(unsigned char *, unsigned);
extern unsigned observe_many(unsigned char *, unsigned, unsigned, unsigned,
                             unsigned, unsigned, unsigned, unsigned, unsigned,
                             unsigned, unsigned, unsigned);
extern unsigned finish_vla(unsigned);
extern unsigned vla_variadic(unsigned, unsigned, unsigned, unsigned, unsigned,
                             unsigned, unsigned, unsigned, unsigned, ...);
extern unsigned vla_register_variadic(unsigned, ...);
extern void mutate_delimiter(void *);
extern unsigned char *escaped_vla_address;
extern unsigned escape_vla(unsigned char *, unsigned);
extern unsigned inspect_expired(void *, unsigned);
extern void *vla_identity(void *);
#ifndef KUNA_ADDRESS_BITS
#if __SIZEOF_POINTER__ == 8
#define KUNA_ADDRESS_BITS unsigned long
#else
#define KUNA_ADDRESS_BITS unsigned int
#endif
#endif
typedef KUNA_ADDRESS_BITS address_bits;
extern address_bits vla_identity_bits(void *);
extern int signed_vla_result(unsigned char *, unsigned);

#define CONCAT44(high, low) \
  (((unsigned long long)(unsigned int)(high) << 32) | (unsigned int)(low))
