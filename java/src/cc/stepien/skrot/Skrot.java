package cc.stepien.skrot;

import java.io.*;
import SevenZip.Compression.LZMA.*;
import org.apache.commons.io.*;

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
      lzma(in, out);
    }

    private void lzma(final InputStream in, final OutputStream out)
      throws IOException {
      final Encoder e = new Encoder();
      e.SetEndMarkerMode(true);
      e.SetDictionarySize(8 * 1024 * 1024);
      e.WriteCoderProperties(out);
      for (int i = 0; i < 8; i++)
        out.write((-1 >>> (8 * i)) & 0xff);
      e.Code(in, out, -1, -1, null);
    }

    private void unlzma(final byte[] input, final OutputStream out)
      throws IOException
    {
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

    private void unlzma(final InputStream input, final OutputStream out)
      throws IOException
    {
      final byte[] bytes = IOUtils.toByteArray(input);
      unlzma(bytes, out);
    }

    private InputStream concat(final InputStream in1, final InputStream in2) {
      return new SequenceInputStream(in1, in2);
    }

    private void throwUnless(boolean value) throws IOException {
      if (!value)
        throw new IOException();
    }

    public void compress(final InputStream model, final InputStream input,
        final OutputStream output) throws IOException {
      final byte[] modelArr = IOUtils.toByteArray(model);
      final ByteArrayOutputStream uncompModel = new ByteArrayOutputStream();
      unlzma(modelArr, uncompModel);
      final InputStream concatenated =
        concat(new ByteArrayInputStream(uncompModel.toByteArray()), input);
      final ByteArrayOutputStream comprConcat = new ByteArrayOutputStream();
      lzma(concatenated, comprConcat);
      final byte[] comprConcatArr = comprConcat.toByteArray();
      int idx;
      for (idx = 0; idx < modelArr.length; idx++)
        if (comprConcatArr[idx] != modelArr[idx])
          break;
      final int diff = modelArr.length - idx;
      assert(diff < 0xff);
      output.write(diff);
      output.write(comprConcatArr, idx, comprConcatArr.length - idx);
    }

    public void decompress(final InputStream model, final InputStream input,
        final OutputStream output) throws IOException {
      final byte[] modelArr = IOUtils.toByteArray(model);
      final byte[] inputArr = IOUtils.toByteArray(input);
      final int cutoff = modelArr.length - inputArr[0];
      final int patchedLen = cutoff + inputArr.length - 1;
      final ByteArrayOutputStream uncompModel = new ByteArrayOutputStream();
      unlzma(modelArr, uncompModel);
      final int fullModelLen = uncompModel.toByteArray().length;
      final InputStream comprConcat =
        concat(new ByteArrayInputStream(modelArr, 0, cutoff),
            new ByteArrayInputStream(inputArr, 1, inputArr.length - 1));
      final ByteArrayOutputStream concat = new ByteArrayOutputStream();
      unlzma(comprConcat, concat);
      final byte[] concatArr = concat.toByteArray();
      output.write(concatArr, fullModelLen, concatArr.length - fullModelLen);
    }
  };
}
