package cc.stepien.skrot;

import java.io.*;
import org.apache.commons.io.*;
import SevenZip.Compression.LZMA.*;

class LZMA implements ICoder {
  public void encode(final InputStream in, final OutputStream out)
    throws IOException
  {
    final Encoder enc = new Encoder();
    enc.SetEndMarkerMode(true);
    enc.SetDictionarySize(8 * 1024 * 1024);
    enc.WriteCoderProperties(out);
    for (int i = 0; i < 8; i++)
      out.write((-1 >>> (8 * i)) & 0xff);
    enc.Code(in, out, -1, -1, null);
  }

  public void decode(final InputStream in, final OutputStream out)
    throws IOException
  {
    final byte[] input = IOUtils.toByteArray(in);
    final int propertiesSize = 5;
    final byte[] properties = new byte[propertiesSize];
    for (int i = 0; i < 5; i++)
      properties[i] = input[i];
    final Decoder dec = new Decoder();
    throwUnless(dec.SetDecoderProperties(properties));
    final int offset = propertiesSize + 8;
    final InputStream inputStream =
      new ByteArrayInputStream(input, offset, input.length - offset);
    throwUnless(dec.Code(inputStream, out, -1));
  }

  private void throwUnless(boolean value) throws IOException {
    if (!value)
      throw new IOException();
  }
}
