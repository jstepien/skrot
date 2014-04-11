package cc.stepien.skrot;

import java.io.*;
import net.jpountz.lz4.*;
import org.apache.commons.io.*;

class LZ4 implements ICodec {
  private final LZ4Factory factory = LZ4Factory.safeInstance();

  private static final int maxSize = 1024 * 1024;

  public void encode(final InputStream in, final OutputStream out)
    throws IOException
  {
    final LZ4Compressor enc = factory.highCompressor();
    out.write(enc.compress(IOUtils.toByteArray(in)));
  }

  public void decode(final InputStream in, final OutputStream out)
    throws IOException
  {
    final LZ4SafeDecompressor dec = factory.safeDecompressor();
    out.write(dec.decompress(IOUtils.toByteArray(in), maxSize));
  }
}
