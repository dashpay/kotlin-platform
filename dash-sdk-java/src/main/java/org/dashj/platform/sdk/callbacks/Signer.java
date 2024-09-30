package org.dashj.platform.sdk.callbacks;

import org.jetbrains.annotations.NotNull;

import java.io.Closeable;
import java.io.IOException;

abstract public class Signer implements Closeable {
    long nativeContext = 0;
    public abstract byte[] sign(byte @NotNull [] key, byte @NotNull [] data);
    public native long getSignerCallback();
    private native long getNativeSigner();
    private native void freeNativeSigner(long nativeContextProvider);

    public long getNativeContext() {
        if (nativeContext == 0) {
            nativeContext = getNativeSigner();
        }
        return nativeContext;
    }

    @Override
    public void close() throws IOException {
        if (nativeContext != 0) {
            freeNativeSigner(nativeContext);
        }
    }
}
