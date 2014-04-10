package cc.stepien.skrot;

import java.io.*;

interface ICodec {
  void encode(final InputStream in, final OutputStream out) throws IOException;

  void decode(final InputStream in, final OutputStream out) throws IOException;
}
