rm ./src/main/java/org/dash/sdk/*.java
swig -java -c++ -outdir src/main/java/org/dash/sdk -package org.dash.sdk -o src/main/cpp/sdk.cpp src/main/swig/example.i