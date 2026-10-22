#if PAYLOAD_BYTES == 8
typedef double SampleFloat;
#else
typedef float SampleFloat;
#endif
typedef struct Packet {unsigned int tag;unsigned char payload[PAYLOAD_BYTES];} Packet;
__declspec(dllexport) SampleFloat read_packet(Packet *packet) {
    if(packet->tag!=7)return 0;
    SampleFloat value;__builtin_memcpy(&value,packet->payload,PAYLOAD_BYTES);return value;
}
__declspec(dllexport) SampleFloat convert_packet(int input) {
    SampleFloat value=(SampleFloat)input;
    Packet packet;packet.tag=7;
    __builtin_memcpy(packet.payload,&value,PAYLOAD_BYTES);return read_packet(&packet);
}
