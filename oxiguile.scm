(define-module 
  (oxiguile)
  #:export (inc
            hello))

(define-public lib
  (dynamic-link "./target/debug/liboxiguile.so"))

(dynamic-call "init" lib)
