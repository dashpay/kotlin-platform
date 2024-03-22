/* ----------------------------------------------------------------------------
 * This file was automatically generated by SWIG (http://www.swig.org).
 * Version 4.0.2
 *
 * Do not make changes to this file unless you know what you are doing--modify
 * the SWIG interface file instead.
 * ----------------------------------------------------------------------------- */

package org.dashj.platform.sdk;

public final class Purpose {
  public final static Purpose AUTHENTICATION = new Purpose("AUTHENTICATION", exampleJNI.AUTHENTICATION_get());
  public final static Purpose ENCRYPTION = new Purpose("ENCRYPTION", exampleJNI.ENCRYPTION_get());
  public final static Purpose DECRYPTION = new Purpose("DECRYPTION", exampleJNI.DECRYPTION_get());
  public final static Purpose WITHDRAW = new Purpose("WITHDRAW", exampleJNI.WITHDRAW_get());
  public final static Purpose SYSTEM = new Purpose("SYSTEM", exampleJNI.SYSTEM_get());
  public final static Purpose VOTING = new Purpose("VOTING", exampleJNI.VOTING_get());

  public final int swigValue() {
    return swigValue;
  }

  public String toString() {
    return swigName;
  }

  public static Purpose swigToEnum(int swigValue) {
    if (swigValue < swigValues.length && swigValue >= 0 && swigValues[swigValue].swigValue == swigValue)
      return swigValues[swigValue];
    for (int i = 0; i < swigValues.length; i++)
      if (swigValues[i].swigValue == swigValue)
        return swigValues[i];
    throw new IllegalArgumentException("No enum " + Purpose.class + " with value " + swigValue);
  }

  private Purpose(String swigName) {
    this.swigName = swigName;
    this.swigValue = swigNext++;
  }

  private Purpose(String swigName, int swigValue) {
    this.swigName = swigName;
    this.swigValue = swigValue;
    swigNext = swigValue+1;
  }

  private Purpose(String swigName, Purpose swigEnum) {
    this.swigName = swigName;
    this.swigValue = swigEnum.swigValue;
    swigNext = this.swigValue+1;
  }

  private static Purpose[] swigValues = { AUTHENTICATION, ENCRYPTION, DECRYPTION, WITHDRAW, SYSTEM, VOTING };
  private static int swigNext = 0;
  private final int swigValue;
  private final String swigName;
}
