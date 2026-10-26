# DashJ Build Pin

Core 24 support uses `org.dashj:dashj-core:24.0.0-20261024.220449-1`,
not the moving `24.0.0-SNAPSHOT` alias. This is the timestamped publication
currently listed in the Central Portal snapshot repository:

https://central.sonatype.com/repository/maven-snapshots/org/dashj/dashj-core/24.0.0-SNAPSHOT/maven-metadata.xml

The JAR resolved by Gradle from that repository has SHA-256:

```text
1f2afb88ca62ab910554e3aa9bf40c240111fc3d3379da152bf59bc19e7828bd
```

This records the verified artifact; it is not dependency checksum enforcement.
The timestamp fixes the selected build, but the repository may eventually delete
old snapshots. Move to a retained immutable release when available. Native SDK
sources and binaries are unchanged by this dependency pin.
