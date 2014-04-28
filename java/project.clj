(defproject cc.stepien/skrot "0.1"
  :dependencies [[commons-io/commons-io "2.4"]
                 [net.jpountz.lz4/lz4 "1.2.0"]]
  :profiles {:dev {:dependencies [[org.clojure/clojure "1.6.0"]]}}
  :java-source-paths ["src"]
  :javac-options ["-source" "1.6" "-target" "1.6"])
