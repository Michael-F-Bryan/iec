TYPE UDT_STRUCTANYPTR
    STRUCT
        S7Code : BYTE; // Code for S7 = 0x10
        DataType : BYTE; // Code for data type = 0x02 = byte
        Length : INT; // Repetition factor = Send/receive length
        DBNumber : INT; // Data block Number
        MemoryArea : BYTE; // Specified memory area = 0x84 = data block
        ByteAddressMSB : BYTE; // Byte address most significant bits
        ByteAddressLSB : WORD; // Byte address least significant bits
    END_STRUCT
END_TYPE