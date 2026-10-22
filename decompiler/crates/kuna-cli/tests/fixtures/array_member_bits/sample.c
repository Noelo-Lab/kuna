#ifndef PAYLOAD_BYTES
#define PAYLOAD_BYTES 4
#endif
#ifndef PAYLOAD_CAPACITY
#define PAYLOAD_CAPACITY PAYLOAD_BYTES
#endif
#if PAYLOAD_BYTES == 8
typedef double SampleFloat;
#else
typedef float SampleFloat;
#endif
typedef struct Packet {unsigned int tag;unsigned char payload[PAYLOAD_CAPACITY];} Packet;
__declspec(dllexport) SampleFloat read_packet(Packet *packet) {
    if (packet->tag != 7) return 0;
#if PAYLOAD_CAPACITY > PAYLOAD_BYTES
    if (packet->payload[PAYLOAD_CAPACITY-1] != 0xa5) return 0;
#endif
    SampleFloat value;
    __builtin_memcpy(&value,packet->payload,PAYLOAD_BYTES);
    return value;
}
__declspec(dllexport) SampleFloat wrap_float(SampleFloat value) {
    Packet packet;
    packet.tag=7;
#if PAYLOAD_CAPACITY > PAYLOAD_BYTES
    packet.payload[PAYLOAD_CAPACITY-1]=0xa5;
#endif
    __builtin_memcpy(packet.payload,&value,PAYLOAD_BYTES);
    return read_packet(&packet);
}
