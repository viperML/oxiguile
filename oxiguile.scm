(define-module 
  (oxiguile)
  #:export (inc))

(define-public lib
  (dynamic-link "./target/debug/liboxiguile.so"))

(dynamic-call "init" lib)
