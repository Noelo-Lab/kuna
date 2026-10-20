#if PAYLOAD_BYTES == 8
typedef double SampleFloat;
#else
typedef float SampleFloat;
#endif
typedef struct Packet {unsigned int tag;unsigned char payload[PAYLOAD_BYTES];void *context;} Packet;
__declspec(dllexport) unsigned int read_word(Packet *packet) {
    if(packet->tag!=7 || packet->context!=0)return 0;
    unsigned int first;__builtin_memcpy(&first,packet->payload,4);
#if PAYLOAD_BYTES == 8
    unsigned int second;__builtin_memcpy(&second,packet->payload+4,4);first=first*257U ^ second;
#endif
    return first;
}
__declspec(dllexport) unsigned int mix_words(SampleFloat *values) {
    Packet packet;packet.tag=7;packet.context=0;
    __builtin_memcpy(packet.payload,&values[0],PAYLOAD_BYTES);
    unsigned int first=read_word(&packet);
    __builtin_memcpy(packet.payload,&values[1],PAYLOAD_BYTES);
    return first*33U ^ read_word(&packet);
}
