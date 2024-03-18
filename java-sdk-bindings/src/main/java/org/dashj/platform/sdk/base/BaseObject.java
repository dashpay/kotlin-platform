package org.dash.sdk.base;

import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/**
 * This base class will support a basic equals
 */
public abstract class BaseObject {
    protected abstract long getCPointer();

    public boolean equals(Object obj) {
        boolean equal = false;
        if (obj instanceof BaseObject)
            equal = (((BaseObject)obj).getCPointer() == this.getCPointer()) || baseObjectEquals((BaseObject) obj);
        return equal;
    }

    public int hashCode() {
        return (int)getCPointer();
    }

    // call the function "objectEquals" in the derived class if it exists.
    protected boolean baseObjectEquals(BaseObject obj) {
        try {
            Method method = getClass().getMethod("objectEquals", getClass());
            Object result = method.invoke(this, obj);
            if (result instanceof Boolean) {
                return (Boolean) result;
            } else {
                System.out.printf("invoke returned type: %s, value: %s", result.getClass().getName(), result);
                return false;
            }
        } catch (NoSuchMethodException e) {
            e.printStackTrace();
            // swallow
        } catch (IllegalAccessException e) {
            e.printStackTrace();
            // swallow
        } catch (InvocationTargetException e) {
            throw new RuntimeException(e);
        }
        return false;
    }
}