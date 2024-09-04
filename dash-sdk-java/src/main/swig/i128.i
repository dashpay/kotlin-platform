
%typemap(jstype) int128_t "java.math.BigInteger";
%typemap(javain) int128_t "$javainput";
%typemap(javaclassname) int128_t "java.math.BigInteger"
%typemap(javatype) int128_t "java.math.BigInteger"
%typemap(jtype) int128_t "java.math.BigInteger"

%typemap(jni) int128_t "jobject"

%typemap(in) int128_t {

    // Get the BigInteger byte array from Java
    jbyteArray byteArray = (jbyteArray)jenv->CallObjectMethod($input, jenv->GetMethodID(jenv->GetObjectClass($input), "toByteArray", "()[B"));
    jbyte *bytes = jenv->GetByteArrayElements(byteArray, NULL);
    jint length = jenv->GetArrayLength(byteArray);

    // Convert byte array to __int128
    $1 = 0; // Initialize the __int128 variable
    bool isNegative = (length > 0 && bytes[0] < 0);
    for (int i = 0; i < length; i++) {
        // Since BigInteger is big-endian, shift the __int128 left and add the new byte
        $1 <<= 8;
        $1 |= ((uint8_t)bytes[i] & 0xFF);
    }

    // If the BigInteger was negative, convert the magnitude to negative
    if (isNegative) {
        // Perform two's complement on the positive magnitude to get the negative value
        $1 = -($1 - (int128_t(1)));
    }

    jenv->ReleaseByteArrayElements(byteArray, bytes, JNI_ABORT); // Release memory without copying back
}

%typemap(javaout) int128_t {
    return $jnicall;
}

%typemap(out) int128_t {
bool isNegative = $1 < 0;
    int128_t temp = $1;
    std::vector<jbyte> bytes;

    while (temp != 0) {
        bytes.push_back(static_cast<jbyte>((jbyte)temp & 0xFF));
        temp >>= 8;
    }

    // Ensure the byte array is the full 128 bits, or 16 bytes
    if (bytes.size() < 16) {
        bytes.resize(16, 0);
    }

    // Reverse the byte array for BigInteger's big-endian expectation
    std::reverse(bytes.begin(), bytes.end());

    // If the number is negative, ensure the highest-order bit is set
    if (isNegative && ((uint8_t)bytes[0] & 0x80) == 0) {
        bytes[0] |= 0x80;
    } else if (!isNegative && (bytes[0] & 0x80) != 0) {
        // If it's positive but the high bit is set, prepend a zero byte to keep it positive
        bytes.insert(bytes.begin(), 0);
    }

    jbyteArray byteArray = jenv->NewByteArray(bytes.size());
    jenv->SetByteArrayRegion(byteArray, 0, bytes.size(), &bytes[0]);

    jclass bigIntegerClass = jenv->FindClass("java/math/BigInteger");
    jmethodID bigIntegerCtor = jenv->GetMethodID(bigIntegerClass, "<init>", "([B)V");
    jobject bigInteger = jenv->NewObject(bigIntegerClass, bigIntegerCtor, byteArray);
    $result = bigInteger;
}