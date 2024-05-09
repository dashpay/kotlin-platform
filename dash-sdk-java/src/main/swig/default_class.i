%define DEFINE_DESTRUCTOR(CLASS_NAME, CTYPE)
%extend CTYPE {
     ~CTYPE() {
         CTYPE##_destroy($self);
     }
}
%enddef

%define DEFINE_CLASS(CLASS_NAME, CTYPE)
%rename (CLASS_NAME) CTYPE;
%extend CTYPE {
     ~CTYPE() {
         CTYPE##_destroy($self);
     }
 }
%enddef

%define START_CLASS(CLASS_NAME, CTYPE)
%rename (CLASS_NAME) CTYPE;
%extend CTYPE {
     ~CTYPE() {
         CTYPE##_destroy($self);
     }
%enddef

%define END_CLASS()
}
%enddef

%define DEFINE_CLASS_1(CLASS_NAME, CTYPE, PARAM)
%rename (CLASS_NAME) CTYPE;
%extend CTYPE {
    CTYPE(PARAM) {
        return CTYPE##_ctor(value);
    }
     ~CTYPE() {
         CTYPE##_destroy($self);
     }
 }
%enddef

%define DEFINE_ALIAS(CLASS_NAME, CTYPE, TYPE, GET, DEFAULT)
%rename (CLASS_NAME) CTYPE;
%extend CTYPE {
    CTYPE() {
        return CTYPE##_ctor(DEFAULT);
    }
    CTYPE(TYPE value) {
        return CTYPE##_ctor(value);
    }
    ~CTYPE() {
        CTYPE##_destroy($self);
    }
    TYPE GET() {
        return $self->_0;
    }
    bool objectEquals(CTYPE* other) {
        if ($self == other) return true;
        if ($self == nullptr || other == nullptr) return false;
        return $self->_0 == other->_0;
    }
}
%ignore CTYPE::_0;
%enddef