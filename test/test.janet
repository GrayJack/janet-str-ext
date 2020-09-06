(import "../build/str-ext" :as str-ext)

(defn test-contains? []
  (assert (str-ext/contains? "the" "Hey, there")))

(defn test-ascii? []
  (assert (str-ext/ascii? "abc"))
  (assert (str-ext/ascii? @"abc"))
  (assert (not (str-ext/ascii? "αβδ")))
  (assert (not (str-ext/ascii? @"αβδ"))))

(defn test-utf8? []
  (assert (str-ext/utf8? "abc"))
  (assert (str-ext/utf8? @"abc"))
  (assert (str-ext/utf8? "αβδ"))
  (assert (str-ext/utf8? @"αβδ"))
  (assert (not (str-ext/utf8? (string/from-bytes 237 160 128)))))

(defn test-chars []
  (let [t1 (str-ext/chars "abc")
        t2 (str-ext/chars @"abc")
        t3 (str-ext/chars "αβδ")
        t4 (str-ext/chars @"αβδ")
        t5 (str-ext/chars "")]
    (assert (= t1 ["a" "b" "c"]))
    (assert (= t2 ["a" "b" "c"]))
    (assert (= t3 ["α" "β" "δ"]))
    (assert (= t4 ["α" "β" "δ"]))
    (assert (= t5 []))))

(defn test-fields []
  (let [t1 (str-ext/fields "   foo\tbar\t\u2003\nquux   \n")
        t2 (str-ext/fields "  \n\t\u2003\n  \t")]
    (assert (= t1 ["foo" "bar" "quux"]))
    (assert (= t2 []))))

(defn test-graphemes []
  (assert (= (str-ext/graphemes "a\u0300\u0316\U01F1FA\U01F1F8") ["à̖" "🇺🇸"])))

(defn test-lines []
  (assert (= (str-ext/lines "foo\n\nbar\r\nbaz\n\n\nquux") ["foo" "" "bar" "baz" "" "" "quux"])))

(defn test-lines-with-terminator []
  (assert (= (str-ext/lines-with-terminator "foo\n\nbar\r\nbaz\n\n\nquux") ["foo\n" "\n" "bar\r\n" "baz\n" "\n" "\n" "quux"])))

(defn test-sentences []
  (assert (= (str-ext/sentences "I want this. Not that. Right now.") ["I want this. " "Not that. " "Right now."])))

(defn test-words []
  (assert (= (str-ext/words `The quick ("brown") fox can't jump 32.3 feet, right?`)) ["The" "quick" "brown" "fox" "can't" "jump" "32.3" "feet" "right"]))

(defn test-words-with-breaks []
  (assert (= (str-ext/words-with-breaks `The quick ("brown") fox can't jump 32.3 feet, right?`) ["The" " " "quick" " " "(" "\"" "brown" "\"" ")" " " "fox" " " "can't" " " "jump" " " "32.3" " " "feet" "," " " "right" "?"])))

(defn test-utf8-chunks []
  (let [[t1 t2] (str-ext/utf8-chunks "foo\xFD\xFEbar\xFF")]
    (assert (= t1 ["foo" "bar"]))
    (assert (= t2 ["\xFD" "\xFE" "\xFF"]))))

(def tests [test-contains? test-ascii? test-utf8? test-chars test-fields test-graphemes test-lines test-lines-with-terminator test-sentences test-words test-words-with-breaks test-utf8-chunks])
(each f tests
  (f))
