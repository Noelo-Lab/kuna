typedef void CURL;
typedef enum { CURLOPT_URL = 10002, CURLOPT_TIMEOUT = 13 } CURLoption;
CURL *curl_easy_init(void);
int curl_easy_setopt(CURL *h, CURLoption opt, ...);
int fetch(const char *url) { CURL *h = curl_easy_init(); curl_easy_setopt(h, CURLOPT_URL, url); return curl_easy_setopt(h, CURLOPT_TIMEOUT, 30L); }
