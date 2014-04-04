package cc.stepien.skrot;

import java.io.*;

public interface ISkrot {
  void model(final InputStream input, final OutputStream output)
    throws IOException;

  void compress(final InputStream model, final InputStream input,
      final OutputStream output) throws IOException;
}
