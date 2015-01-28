(ns cc.stepien.skrot.test
  (:import [cc.stepien.skrot Skrot Codec]
           [java.io ByteArrayOutputStream])
  (:require [clojure.java.io :as io]
            [clojure.test.check
             [clojure-test :refer [defspec]]
             [generators :as gen]
             [properties :as prop]]))

(def gen-codec
  (gen/one-of (map gen/return [Codec/LZ4 Codec/LZMA])))

(defn- ->bytes [f]
  (.toByteArray (doto (ByteArrayOutputStream.) f)))

(defn- bytes->model-bytes [codec bs]
  (->bytes #(.model (Skrot/with codec)
                    (io/input-stream bs)
                    %)))

(defn- compress [codec model input]
  (->bytes #(.compress (Skrot/with codec)
                       (io/input-stream model)
                       (io/input-stream input)
                       %)))

(defn- decompress [codec model input]
  (->bytes #(.decompress (Skrot/with codec)
                         (io/input-stream model)
                         (io/input-stream input)
                         %)))

(defspec prop-reversability 1000
  (prop/for-all [codec gen-codec
                 model-source gen/bytes
                 input gen/bytes]
    (let [model (bytes->model-bytes codec model-source)
          compressed (compress codec model input)
          decompressed (decompress codec model compressed)]
      (= (seq decompressed) (seq input)))))
