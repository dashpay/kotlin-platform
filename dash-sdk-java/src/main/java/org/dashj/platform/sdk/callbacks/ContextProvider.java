package org.dashj.platform.sdk.callbacks;

import org.dashj.platform.sdk.Identifier;
import org.jetbrains.annotations.Nullable;

import java.io.Closeable;
import java.io.IOException;

abstract public class ContextProvider implements Closeable {
    long nativeContext = 0;
    public abstract @Nullable byte [] getQuorumPublicKey(int quorumType, byte [] quorumHash, int coreChainLockedHeight);
    public abstract byte [] getDataContract(Identifier identifier);
    public native long getQuorumPublicKeyCallback();
    private native long getNativeContextProvider();
    private native void freeNativeContextProvider(long nativeContextProvider);

    public long getNativeContext() {
        if (nativeContext == 0) {
            nativeContext = getNativeContextProvider();
        }
        return nativeContext;
    }

    @Override
    public void close() throws IOException {
        if (nativeContext != 0) {
            freeNativeContextProvider(nativeContext);
        }
    }
}
