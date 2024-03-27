package org.dashj.platform.sdk.callbacks;

import org.dashj.platform.sdk.Identifier;
import org.jetbrains.annotations.Nullable;

abstract public class ContextProvider {
    public abstract @Nullable byte [] getQuorumPublicKey(int quorumType, byte [] quorumHash, int coreChainLockedHeight);
    public abstract byte [] getDataContract(Identifier identifier);
    public native long getQuorumPublicKeyCallback();
}
