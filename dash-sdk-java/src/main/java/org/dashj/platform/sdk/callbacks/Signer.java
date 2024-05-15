package org.dashj.platform.sdk.callbacks;

import org.dashj.platform.sdk.BinaryData;
import org.dashj.platform.sdk.Identifier;
import org.dashj.platform.sdk.IdentityPublicKey;
import org.jetbrains.annotations.Nullable;

abstract public class Signer {
    public abstract byte[] sign(byte[] key, byte[] data);
    public native long getSignerCallback();
}
