(declare-project
  :name "str-ext"
  :description "Janet library with functions to extend the available API for string and buffers in the langauge"
  :author "GrayJack"
  :license "MIT or Apache 2.0"
  :url "https://grayjack.github.io/janet-str-ext/"
  :repo "git+https://github.com/GrayJack/janet-str-ext.git")

(post-deps
  (declare-native
    :name "template"
    :source [])

  (phony "build-release" []
    (os/execute ["mkdir" "-p" "build"] :p)
    (os/execute ["cargo" "build" "--release" "--target-dir" "target" "--quiet"] :p)
    (os/execute ["cp" "target/release/libstr_ext.a" "build/str_ext.a"] :p)
    (let [os (os/which)]
      (if (= os :linux)
        (os/execute ["cp" "target/release/libstr_ext.so" "build/str_ext.so"] :p))
      (if (= os :macos)
        (os/execute ["cp" "target/release/libstr_ext.dylib" "build/str_ext.dylib"] :p))
      (if (= os :windows)
        (os/execute ["cp" "target/release/libstr_ext.dll" "build/str_ext.dll"] :p))))

  (phony "build-debug" []
    (os/execute ["cargo" "build" "--debug" "--target-dir" "target" "--quiet"] :p)
    (os/execute ["mkdir" "-p" "build"] :p)
    (os/execute ["cp" "target/debug/libstr_ext.a" "build/str_ext.a"] :p)
    (let [os (os/which)]
      (if (= os :linux)
        (os/execute ["cp" "target/debug/libstr_ext.so" "build/str_ext.so"] :p))
      (if (= os :macos)
        (os/execute ["cp" "target/debug/libstr_ext.dylib" "build/str_ext.dylib"] :p))
      (if (= os :windows)
        (os/execute ["cp" "target/debug/libstr_ext.dll" "build/str_ext.dll"] :p))))

  (phony "all" ["build-release"])

  (add-dep "build" "all")

  (phony "clean-target" []
    (os/execute ["rm" "-rf" "target"] :p))

  (add-dep "clean" "clean-target"))