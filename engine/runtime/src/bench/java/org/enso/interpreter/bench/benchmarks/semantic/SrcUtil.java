package org.enso.interpreter.bench.benchmarks.semantic;

import java.io.File;
import java.io.FileWriter;
import java.io.IOException;
import org.graalvm.polyglot.Source;
import org.openjdk.jmh.infra.BenchmarkParams;

final class SrcUtil {
  private SrcUtil() {
  }

  static String findName(BenchmarkParams params) {
    return params.getBenchmark().replaceFirst(".*\\.", "");
  }

  static Source source(String benchmarkName, String code) throws IOException {
    var d = new File(new File(new File("."), "target"), "bench-data");
    d.mkdirs();
    var f = new File(d, benchmarkName + ".enso");
    try ( var w = new FileWriter(f)) {
      w.write(code);
    }
    return Source.newBuilder("enso", f).build();
  }
}
