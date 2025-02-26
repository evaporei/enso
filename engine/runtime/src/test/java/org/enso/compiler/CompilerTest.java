package org.enso.compiler;

import com.oracle.truffle.api.source.Source;
import java.io.File;
import org.enso.compiler.core.IR;
import org.junit.AfterClass;
import org.junit.BeforeClass;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardOpenOption;
import java.util.function.Function;
import static org.junit.Assert.assertEquals;

import static org.junit.Assert.assertNotNull;

public abstract class CompilerTest {

  protected static EnsoCompiler ensoCompiler;

  @BeforeClass
  public static void initEnsoCompiler() {
    ensoCompiler = new EnsoCompiler();
  }

  @AfterClass
  public static void closeEnsoCompiler() throws Exception {
    ensoCompiler.close();
  }

  protected static IR.Module parse(String code) throws IOException {
    var src =
        Source.newBuilder("enso", code, "test-" + Integer.toHexString(code.hashCode()) + ".enso")
            .build();
    IR.Module ir = ensoCompiler.compile(src);
    assertNotNull("IR was generated", ir);
    return ir;
  }

  static void assertIR(String msg, IR.Module old, IR.Module now) throws IOException {
    Function<IR, String> filter = f -> simplifyIR(f, true, true, false);
    String ir1 = filter.apply(old);
    String ir2 = filter.apply(now);
    if (!ir1.equals(ir2)) {
      String name = findTestMethodName();
      var home = new File(System.getProperty("java.io.tmpdir")).toPath();
      var file1 = home.resolve(name + ".1");
      var file2 = home.resolve(name + ".2");
      Files.writeString(file1, ir1, StandardOpenOption.TRUNCATE_EXISTING, StandardOpenOption.CREATE, StandardOpenOption.WRITE);
      Files.writeString(file2, ir2, StandardOpenOption.TRUNCATE_EXISTING, StandardOpenOption.CREATE, StandardOpenOption.WRITE);
      assertEquals(msg, file1, file2);
    }
  }

  private static String findTestMethodName() {
    for (var e : new Exception().getStackTrace()) {
      if (e.getMethodName().startsWith("test")) {
        return e.getMethodName();
      }
    }
    throw new IllegalStateException();
  }

  /** Takes an {@link IR} and converts it to text representation suitable for
   * "diffing" while "simplifying" it.
   *
   * @param ir the intermediate representation
   * @param noIds remove all UUIDs or keep them? Multiple runs usually assign
   *   random/different UUIDs to various IR elements. Removing them is a best
   *   way to make the converted text comparable
   * @param noLocations locations may slightly differ. Usually off-by-one.
   *   Especially when running old and new parser in parallel - removing them
   *   may be useful
   * @param lessDocs documentation often isn't an essential part of the IR
   *   one can easily remove it by specifying {@code false}
   * @return string representation of the IR
   */
  private static String simplifyIR(IR ir, boolean noIds, boolean noLocations, boolean lessDocs) {
    String txt = ir.pretty();
    if (noIds) {
      txt = txt.replaceAll("[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]\\-[0-9a-f][0-9a-f][0-9a-f][0-9a-f]\\-[0-9a-f][0-9a-f][0-9a-f][0-9a-f]\\-[0-9a-f][0-9a-f][0-9a-f][0-9a-f]\\-[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]", "_");
    }
    if (noLocations) {
      for (;;) {
        final String pref = " Location(";
        int at = txt.indexOf(pref);
        if (at == -1) {
          break;
        }
        int to = at + pref.length();
        int depth = 1;
        while (depth > 0) {
          switch (txt.charAt(to)) {
            case '(' -> depth++;
            case ')' -> depth--;
          }
          to++;
        }
        txt = txt.substring(0, at) + "Location[_]" + txt.substring(to);
      }
    }
    if (lessDocs) {
      for (;;) {
        final String pref = "IR.Comment.Documentation(";
        int at = txt.indexOf(pref);
        if (at == -1) {
          break;
        }
        int to = txt.indexOf("location =", at + pref.length());
        txt = txt.substring(0, at) + "IR.Comment.Doc(" + txt.substring(to);
      }
      for (;;) {
        final String pref = "IR.Case.Pattern.Doc(";
        int at = txt.indexOf(pref);
        if (at == -1) {
          break;
        }
        int to = txt.indexOf("location =", at + pref.length());
        txt = txt.substring(0, at) + "IR.Comment.CaseDoc(" + txt.substring(to);
      }
    }
    for (;;) {
      final String pref = "IR.Error.Syntax(";
      int at = txt.indexOf(pref);
      if (at == -1) {
        break;
      }
      int to = txt.indexOf("reason =", at + pref.length());
      txt = txt.substring(0, at) + "IR.Error.Syntax (" + txt.substring(to);
    }
    return txt;
  }
}
