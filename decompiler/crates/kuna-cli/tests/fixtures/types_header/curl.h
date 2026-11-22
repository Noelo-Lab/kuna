typedef void CURL;
typedef enum { CURLOPT_URL = 10002, CURLOPT_TIMEOUT = 13 } CURLoption;
CURL *curl_easy_init(void);
int curl_easy_setopt(CURL *h, CURLoption opt, ...);
