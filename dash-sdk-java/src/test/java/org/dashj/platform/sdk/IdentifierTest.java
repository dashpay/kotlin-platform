package org.dashj.platform.sdk;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class IdentifierTest extends BaseTest {
    @Test
    public void createIdentiferBytes32InJavaAndDestroy() {
        IdentifierBytes32 ib32 = new IdentifierBytes32(identifier);
        assertArrayEquals(identifier, ib32.get_0());
        ib32.delete();
    }

    @Test
    public void createIdentiferInJavaAndDestroy() {
        Identifier id = new Identifier(identifier);
        assertArrayEquals(identifier, id.get_0().get_0());
        id.delete();
    }

//    @Test
//    public void createIdentiferBytes32InRustAndDestroy() {
//        IdentifierBytes32 ib32 = example.identifierBytes32Ctor(identifier);
//        assertArrayEquals(identifier, ib32.get_0());
//        example.identifierBytes32Destroy(ib32);
//    }
//
//    @Test
//    public void createIdentiferInRustAndDestroy() {
//        Identifier id = example.identifierCtor(example.identifierBytes32Ctor(identifier));
//        assertArrayEquals(identifier, id.get_0().get_0());
//        example.identifierDestroy(id);
//    }

//    @Test
//    public void ctorOwnership() {
//        Identifier id = example.identifierCtor(example.identifierBytes32Ctor(identifier));
//        assertNotNull(id);
//        // identifier doesn't own the pointer
//        assertFalse(id.swigCMemOwn);
//        id.delete();
//
//        Identifier identifier1 = new Identifier(new byte[32]);
//        // identifier1 does own the pointer
//        assertTrue(identifier1.swigCMemOwn);
//        identifier1.delete();
//    }

    @Test
    public void cloneTest() {
        Identifier id = new Identifier(identifier);
        Identifier idClone = dashsdk.identifierClone(id);
        assertArrayEquals(id.get_0().get_0(), idClone.get_0().get_0());
        id.delete();
        idClone.delete();
    }
}
