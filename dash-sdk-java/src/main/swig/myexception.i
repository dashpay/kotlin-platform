%include "exception.i"
#include <exception>

%exception {
    try {
        $action
    } catch(std::string& x) {
        SWIG_exception(SWIG_JavaUnknownError, x.c_str());
    } catch(std::runtime_error& x) {
        SWIG_exception(SWIG_RuntimeError, x.what());
    } catch(std::invalid_argument& x) {
        SWIG_exception(SWIG_TypeError, x.what());
    } catch(std::exception& x) {
        SWIG_exception(SWIG_UnknownError, x.what());
    } catch(...) {
        SWIG_exception(SWIG_UnknownError,"Unknown exception");
    }
}
