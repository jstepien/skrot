(defproject cc.stepien/skrot "0.2"
  :dependencies [[commons-io/commons-io "2.4"]
                 [net.jpountz.lz4/lz4 "1.2.0"]]
  :profiles {:dev {:dependencies [[org.clojure/clojure "1.8.0"]
                                  [org.clojure/test.check "0.9.0"]]}}
  :java-source-paths ["src"]
  :javac-options ["-source" "1.6" "-target" "1.6"])
