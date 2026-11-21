#ifndef CURLISH_MACRO_H
#define CURLISH_MACRO_H
#define CURL_EXTERN __attribute__((visibility("default")))
#define OPT(name, base, nu) CURLOPT_##name = base + nu
typedef void CURL;
typedef enum { OPT(URL, 10000, 2), OPT(TIMEOUT, 0, 13) } CURLoption;
CURL_EXTERN CURL *curl_easy_init(void);
CURL_EXTERN int curl_easy_setopt(CURL *handle, CURLoption option, ...);
#endif
