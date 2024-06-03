package org.dashj.platform.dpp.identity

import org.bitcoinj.core.Sha256Hash
import org.dashj.platform.dpp.BaseObject
import org.dashj.platform.dpp.identifier.Identifier

abstract class AssetLockProof : BaseObject() {

    abstract val type: Int

    abstract fun getOutPoint(): ByteArray

    /**
     * the identifier is the double SHA hash of the outPoint structure
     * which is the txid in big endian followed by the output index in little Endian
     */
//    pub fn create_identifier(&self) -> Result<Identifier, ProtocolError> {
//        let outpoint = self.out_point().ok_or_else(|| {
//            ProtocolError::IdentifierError(String::from("No output at a given index"))
//        })?;
//
//        let outpoint_bytes: [u8; 36] = outpoint.try_into().unwrap();
//
//        let hash = hash_double(outpoint_bytes.as_slice());
//
//        Ok(Identifier::new(hash))
//    }

    fun createIdentifier(): Identifier {
        return Identifier.from(Sha256Hash.twiceOf(getOutPoint()))
    }
}
