rm ./src/main/java/org/dashj/platform/sdk/*.java
swig -java -c++ -outdir src/main/java/org/dashj/platform/sdk -package org.dashj.platform.sdk -o src/main/cpp/sdk.cpp src/main/swig/example.i