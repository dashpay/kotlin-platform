%module example

%{
//extern "C" {
#include "../../../../rs-sdk/target/rs_sdk_bindings.h"
//}
#include <stdlib.h>
#include "dpp.h"
#include <ctime>

MemoryFactory * MemoryFactory::instance = new MemoryFactory();
MemoryFactory & memoryFactory = *MemoryFactory::getInstance();
%}
//%include "enumsimple.swg"

%ignore dpp_identity_identity_IdentityFactory_TraitObject::object;
%ignore dpp_identity_identity_IdentityFactory_TraitObject::vtable;
%ignore IdentityFactory_VTable;
%ignore IHaveChainSettings_TraitObject::object;
%ignore IHaveChainSettings_TraitObject::vtable;
%ignore IHaveChainSettings_VTable;
%nodefaultctor;

%rename("%(lowercamelcase)s", %$isfunction) "";

%ignore Vec_u8;

//%ignore platform_value_types_identifier_IdentifierBytes32;
//%rename("%(lowercamelcase)s") "";
%include "stdint.i"
// %include "arrays_java.i"

// Identity Related Structures
%include "binary_data.i"
%include "contract_bounds.i"
%include "identifier.i"
%include "identity.i"
%include "identity_public_key.i"
%include "identity_public_key_enums.i"
%include "keyid.i"
%include "revision.i"
%include "uint8_array.i"
%include "timestamp_millis.i"
%include "enums.i"

// ignore

%ignore FeatureVersion;
%ignore Vec_ferment_example_nested_FeatureVersion;

// typemaps
%include "Vec_u8.i"

// Java Classes
%typemap(javaimports) SWIGTYPE, SWIGTYPE *, SWIGTYPE &, SWIGTYPE [], SWIGTYPE (CLASS::*) %{
import org.dashj.platform.sdk.base.BaseObject;
%}
%typemap(javabase) SWIGTYPE, SWIGTYPE *, SWIGTYPE &, SWIGTYPE [], SWIGTYPE (CLASS::*) "BaseObject"

%typemap(javacode) SWIGTYPE, SWIGTYPE *, SWIGTYPE &, SWIGTYPE [], SWIGTYPE (CLASS::*) %{
  protected long getCPointer() {
    return swigCPtr;
  }
%}

//extern "C" {
%include "../../../../rs-sdk/target/rs_sdk_bindings.h"
//}
%include "src/main/cpp/dpp.h"
