typedef struct BytePacket {
    unsigned char payload[4];
    void *context;
} BytePacket;

__declspec(dllexport) float read_packet(BytePacket *packet) {
    float value;
    __builtin_memcpy(&value, packet->payload, 4);
    return value;
}

__declspec(dllexport) float wrap_word(unsigned int bits) {
    BytePacket packet;
    __builtin_memcpy(packet.payload, &bits, 4);
    packet.context = 0;
    return read_packet(&packet);
}
