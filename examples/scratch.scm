;; this is a basic statically-typed lambda calculus extended with the following special forms:
;; `def`
;; `let`
;; `quote` 
;; `quasiquote`
;; `unquote`
;; `unquote-splicing`
;; `fn`
;; `and`
;; `or`
;; `begin`
;; and a few built-in functions (e.g. `+`, `-`, `*`, `/`, `=`, `>`, `<`, `<=`, `>=`)

;; def declarations
(def x 42)

;; let expressions
(let ((x 1) (y 2)) (+ x y))

;; if expressions
(if (< 1 2) 1 2)

;; quote expressions
(quote (1 2 3))
'(1 2 3)

;; quasiquote/unquote expressions
(quasiquote (1 2 (unquote (+ 1 2)) 4))
`(1 2 ,(+ 1 2) 4)

;; lambda expressions
(fn (x) (+ x 1))

;; and expressions (short-circuiting)
(and (< 1 2) (> 2 1))

;; or expressions (short-circuiting)
(or (< 1 2) (> 2 1))

;; not expressions
(not (< 1 2))

;; begin expressions
(begin (println 1) (println 2) (println 3))

;; We also have a special syntax for lists meant to be used only as data
;; Where you might ordinarily write (list 1 2 (+ 1 2)) in most Lisps, you would write
;; [1 2 (+ 1 2)] in this language. More precisely, the brackets are used to denote
;; a quasiquoted list where all elements are unquoted.
[1 2 (+ 1 2)]

;; vectors
#[1 2 3]

;; macros
(defmacro (loop &body)
  `(let ((loop (fn () (begin ,@body (loop)))))))

;; variadic macros
(defmacro while (test &body)
  `(let ((loop (fn () (if ,test (begin ,@body (loop))))))))

(while (< i 10)
  (println i)
  (def i (+ i 1)))

;; expands to:
(let ((loop (fn () 
              (if (< i 10) 
                (begin 
                  (println i) 
                  (def i (+ i 1)) 
                  (loop))))))
  (loop))

(let (fib n)
  (if (< n 2) 
      n 
      (+ (fib (- n 1)) (fib (- n 2)))))

(let (fib n)
  (if (< n 2) 
      n 
      (+ (fib (- n 1)) (fib (- n 2))))
  (fib 10))

(def x 1)

(defn id (x) x)

(module List
  (record Nil)
  (record (Pair head tail)))

;; overload `display` for `List`
(def (display xs)
  (match xs
    (Nil (println "[]"))
    ((Pair x xs) (begin
                   (print "[")
                   (display x)
                   (print ", ")
                   (display xs)
                   (print "]")))))

(def (map f xs)
  (match xs
    (Nil Nil)
    ((Pair x xs) (Pair (f x) (map f xs)))))

;; `begin` expands to a sequence of `let` forms
(macro (begin &body)
  `(let ((,_)) (begin ,@(tail body)) ,_))

(begin (println 1) (println 2) (println 3))

;; expands to:
(let ((_ (print 1))
      (_ (print 2) (print 3))))


(let ((x 1) 
      (y 2))
  (println (+ x y)))

(let ((fib n
        (if (< n 2) 
            n 
            (+ (fib (- n 1)) (fib (- n 2))))))
  (fib 10))
