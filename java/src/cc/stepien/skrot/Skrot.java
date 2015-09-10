package cc.stepien.skrot;

import java.io.*;
import org.apache.commons.io.*;

/**
 * Pure Java implementation of Skrot compression.
 * Create a thread-safe instance using {@link #with}.
 */
public class Skrot {

  /**
   * Returns an instance backed by a given compression algorithm. Throws an
   * exception if the codec is invalid.
   */
  public static Skrot with(Codec codec) {
    if (codec == Codec.LZMA)
      return new Skrot(new LZMA());
    if (codec == Codec.LZ4)
      return new Skrot(new LZ4());
    throw new RuntimeException("No such algorithm: " + codec);
  }

  /**
   * Builds a Skrot model reading data from the given {@link InputStream} and
   * writes it into the given {@link OutputStream}. Use the generated model for
   * compression and decompression.
   * @param model Arbitrary input
   * @param output Generated model
   */
  public void model(final InputStream model, final OutputStream output)
    throws IOException
  {
    coder.encode(model, output);
  }

  /**
   * Compresses given input stream using given model and writes it to the given
   * output stream.
   * @param model A model created with {@link #model}
   * @param input Arbitrary input
   */
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
    output.write(diff - 128);
    output.write(comprConcatArr, idx, comprConcatArr.length - idx);
  }


  /**
   * Decompresses given input stream using given model and writes it to the
   * given output stream.
   * @param model A model created with {@link #model}
   * @param input Data compressed with {@link #compress}
   */
  public void decompress(final InputStream model, final InputStream input,
      final OutputStream output) throws IOException {
    final byte[] modelArr = IOUtils.toByteArray(model);
    final byte[] inputArr = IOUtils.toByteArray(input);
    final int diff = inputArr[0] + 128;
    final int cutoff = modelArr.length - diff;
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
