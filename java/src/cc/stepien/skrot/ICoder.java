package cc.stepien.skrot;

import java.io.*;

interface ICoder {
  void encode(final InputStream in, final OutputStream out) throws IOException;

  void decode(final InputStream in, final OutputStream out) throws IOException;
}
