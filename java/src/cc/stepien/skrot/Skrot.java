package cc.stepien.skrot;

import java.io.*;
import SevenZip.Compression.LZMA.Encoder;

public class Skrot {
  public static ISkrot with(IAlgorithm algo) throws Exception {
    if (algo == Algorithm.LZMA)
      return LZMA;
    throw new Exception("No such algorithm: " + algo);
  }

  private static final ISkrot LZMA = new ISkrot() {
    public void model(final InputStream in, final OutputStream out)
      throws IOException
    {
      final Encoder e = new Encoder();
      e.SetEndMarkerMode(true);
      e.SetDictionarySize(8 * 1024 * 1024);
      e.WriteCoderProperties(out);
      for (int i = 0; i < 8; i++)
        out.write((-1 >>> (8 * i)) & 0xff);
      e.Code(in, out, -1, -1, null);
    }
  };
}
