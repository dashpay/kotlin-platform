package org.dashj.platform.sdk;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertArrayEquals;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;

public class ContractBoundsTest extends BaseTest {

//    @Test
//    public void createSingleContractInJavaAndDestroy() {
//        Identifier id = new Identifier(identifier);
//        ContractBounds singleContractA = new ContractBounds(id);
//        singleContractA.delete();
//
//
//        ContractBounds singleContract = example.singleContract(id);
//        assertEquals(ContractBounds_Tag.ContractBounds_SingleContract, singleContract.getTag());
//        assertArrayEquals(identifier, singleContract.getSingle_contract_document_type().getId().get_0().get_0());
//        //singleContract.delete(); // doesn't delete the rust object because singleContract does not own it
//        //ContractBounds contractBounds2 = new ContractBounds(ContractBounds.getCPtr(singleContract), true);
//        //contractBounds2.delete();
//        singleContract.delete();
//        // example.contractBoundsDestroy(singleContract); // crash
//        assertFalse(singleContract.swigCMemOwn);
//        assertEquals(0, ContractBounds.getCPtr(singleContract));
//        id.delete();
//
//        //ContractBounds contractBounds = new ContractBounds(ContractBounds.getCPtr(example.contractBoundsSingleContractCtor(id)), true);
//        //contractBounds.delete();
//    }

//    @Test
//    public void createSingleContractInRustAndTakeOwnershipInJavaAndDestroy() {
//        Identifier id = new Identifier(identifier);
//        ContractBounds singleContract = example.singleContract(id); //ContractBounds.singleContract(id);
//        assertEquals(ContractBounds_Tag.SingleContract, singleContract.getTag());
//        assertArrayEquals(identifier, singleContract.getSingle_contract_document_type().getId().get_0().get_0());
//        singleContract.delete();
//        //ContractBounds singleContractOwner = new ContractBounds(ContractBounds.getCPtr(singleContract), true);
//        //singleContractOwner.delete();
//        id.delete();
//    }

    @Test
    public void createSingleContractInJavaAndDestroy2() {
        Identifier id = new Identifier(identifier);
        ContractBounds singleContract = new ContractBounds(id);
        Assertions.assertEquals(ContractBounds_Tag.SingleContract, singleContract.getTag());
        assertArrayEquals(identifier, singleContract.getSingle_contract_document_type().getId().get_0().get_0());
        singleContract.delete();
        id.delete();
    }

//    @Test
//    public void createSingleContractDocumentTypeInJavaAndDestroy() {
//        Identifier id = new Identifier(identifier);
//        ContractBounds singleContract = example.singleContractDocumentType(id, "type");
//        assertEquals(ContractBounds_Tag.SingleContractDocumentType, singleContract.getTag());
//        assertArrayEquals(identifier, singleContract.getSingle_contract_document_type().getId().get_0().get_0());
//        assertEquals("type", singleContract.getSingle_contract_document_type().getDocument_type_name());
//        singleContract.delete();
//        id.delete();
//    }

    // @Test
    public void createSingleContractDocumentTypeInJavaAndDestroy2() {
        Identifier id = new Identifier(identifier);
        ContractBounds singleContract = new ContractBounds(id, "type");
        assertEquals(ContractBounds_Tag.SingleContractDocumentType, singleContract.getTag());
        assertArrayEquals(identifier, singleContract.getSingle_contract_document_type().getId().get_0().get_0());
        assertEquals("type", singleContract.getSingle_contract_document_type().getDocument_type_name());
        singleContract.delete(); // doesn't delete the rust object because singleContract does not own it
        id.delete();
    }
}
