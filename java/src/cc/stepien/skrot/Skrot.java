package cc.stepien.skrot;

import java.io.*;
import org.apache.commons.io.*;

public class Skrot {

  public static Skrot with(Codec codec) throws Exception {
    if (codec == Codec.LZMA)
      return new Skrot(new LZMA());
    throw new Exception("No such algorithm: " + codec);
  }

  public void model(final InputStream model, final OutputStream output)
    throws IOException
  {
    coder.encode(model, output);
  }

  public void compress(final InputStream model, final InputStream input,
      final OutputStream output) throws IOException {
    final byte[] modelArr = IOUtils.toByteArray(model);
    final ByteArrayOutputStream uncompModel = new ByteArrayOutputStream();
    coder.decode(new ByteArrayInputStream(modelArr), uncompModel);
    final InputStream concatenated =
      concat(new ByteArrayInputStream(uncompModel.toByteArray()), input);
    final ByteArrayOutputStream comprConcat = new ByteArrayOutputStream();
    coder.encode(concatenated, comprConcat);
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
    coder.decode(new ByteArrayInputStream(modelArr), uncompModel);
    final int fullModelLen = uncompModel.toByteArray().length;
    final InputStream comprConcat =
      concat(new ByteArrayInputStream(modelArr, 0, cutoff),
          new ByteArrayInputStream(inputArr, 1, inputArr.length - 1));
    final ByteArrayOutputStream concat = new ByteArrayOutputStream();
    coder.decode(comprConcat, concat);
    final byte[] concatArr = concat.toByteArray();
    output.write(concatArr, fullModelLen, concatArr.length - fullModelLen);
  }

  private InputStream concat(final InputStream in1, final InputStream in2) {
    return new SequenceInputStream(in1, in2);
  }

  private Skrot(ICodec coder) {
    this.coder = coder;
  }

  private final ICodec coder;
}
