%module dashsdk

%{
//extern "C" {
#include "config.h"
#include "../../../../dash-sdk-bindings/target/dash_sdk_bindings.h"
//}
#include <stdlib.h>
#include "dpp.h"
#include "conversions.h"
#include <ctime>
#include "wrapper.h"
#include "clone.h"
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
%ignore Arr_u8_36;
%ignore Arr_u8_32;
%ignore Arr_u8_20;
%ignore __int128;
%ignore i128;

//%rename("%(lowercamelcase)s") "";
%include "stdint.i"
// %include "arrays_java.i"
%include "default_class.i"

%include "myexception.i"
// generics
%include "generics/lists.i"
%include "generics/maps.i"
%include "generics/result.i"

// Identity Related Structures
%include "i128.i"
%include "binary_data.i"
%include "contract_bounds.i"
%include "drive.i"
%include "document.i"
%include "document_properties.i"
%include "hash.i"
%include "identifier.i"
%include "identity.i"
%include "identity_public_key.i"
%include "identity_public_key_enums.i"
%include "keyid.i"
%include "revision.i"
%include "uint8_array.i"
%include "uint8_array_20.i"
%include "timestamp_millis.i"
%include "enums.i"
%include "Arr_u8_36.i"
%include "Arr_u8_32.i"
%include "Arr_u8_20.i"
%include "platform_value.i"
%include "platform_value_array.i"

// modules and crates
%include "dpp.i"
%include "rs_sdk.i"

// ignore

%ignore FeatureVersion;
%ignore Vec_ferment_example_nested_FeatureVersion;
%ignore platform_mobile_MyValue;
%ignore platform_mobile_MyValueMap;

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

%include "ignore.i"


//extern "C" {
%include "../../../../dash-sdk-bindings/target/dash_sdk_bindings.h"
//}
%include "src/main/cpp/dpp.h"
