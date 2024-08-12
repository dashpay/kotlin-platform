// DEFINE_ALIAS(
//     KeyCount,
//     dpp_identity_identity_public_key_KeyCount,
//     dpp_identity_identity_public_key_KeyID,
//     toKeyID,
//     0
// );

DEFINE_ALIAS(
    BlockHeight,
    dpp_prelude_BlockHeight,
    long,
    toLong,
    0
);

DEFINE_ALIAS(
    CoreBlockHeight,
    dpp_prelude_CoreBlockHeight,
    uint32_t,
    toLong,
    0
);

%ignore OutPoint::txid;
%ignore OutPoint::vout;
%extend OutPoint {
    OutPoint(Arr_u8_32 * txid, uint32_t vout) {
        OutPoint * out_point = new OutPoint;
        out_point->txid = (uint8_t (*)[32])memoryFactory.clone((uint8_t*)txid->values, 32);
        out_point->vout = vout;
        return out_point;
    }
    ~OutPoint() {
        memoryFactory.destroy($self->txid);
        delete $self;
    }
    uint8_t (* getTxid())[32] {
        return $self->txid;
    }
    long getIndex() { return $self->vout; }
}

START_CLASS(
    OutPointFFI,
    platform_mobile_put_OutPointFFI
);
    platform_mobile_put_OutPointFFI(Arr_u8_32 *txid, uint32_t vout) {
        return platform_mobile_put_OutPointFFI_ctor(clone(txid), vout);
    }

//     bool objectEquals(platform_mobile_put_OutPointFFI* other) {
//         if ($self == other) return true;
//         if ($self == nullptr || other == nullptr) return false;
//         return $self->_0 == other->_0;
//     }
END_CLASS();

START_CLASS(
    AssetLockProof,
    platform_mobile_put_AssetLockProofFFI
);
    platform_mobile_put_AssetLockProofFFI(platform_mobile_put_ChainAssetLockProofFFI * chain) {
        return platform_mobile_put_AssetLockProofFFI_Chain_ctor(clone(chain));
    }
    platform_mobile_put_AssetLockProofFFI(platform_mobile_put_InstantAssetLockProofFFI * instant) {
        return platform_mobile_put_AssetLockProofFFI_Instant_ctor(clone(instant));
    }
END_CLASS();

START_CLASS(
    ChainAssetLockProof,
    platform_mobile_put_ChainAssetLockProofFFI
);
    platform_mobile_put_ChainAssetLockProofFFI(uint32_t core_chain_locked_height, OutPoint *out_point) {
        return platform_mobile_put_ChainAssetLockProofFFI_ctor(
            core_chain_locked_height,
            clone(out_point)
        );
    }

END_CLASS();



START_CLASS(
    InstantAssetLockProof,
    platform_mobile_put_InstantAssetLockProofFFI
);
    platform_mobile_put_InstantAssetLockProofFFI(Vec_u8 *instant_lock, Vec_u8 *transaction, uint32_t output_index) {
        return platform_mobile_put_InstantAssetLockProofFFI_ctor(
            clone(instant_lock),
            clone(transaction),
            output_index
        );
    }
END_CLASS();