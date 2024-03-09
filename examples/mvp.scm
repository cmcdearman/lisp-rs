;; this is a basic statically-typed lambda calculus extended with the following special forms:
;; `def`
;; `let`
;; `if`
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
;; Where you might ordinarily write `(1 2 ,(+ 1 2)) in most Lisps, you would write
;; [1 2 (+ 1 2)] in this language. More precisely, the brackets are used to denote
;; a quasiquoted list where all elements are unquoted.
[1 2 (+ 1 2)]

;; vectors
#[1 2 3]

;; macros
(defmacro defn (name args body)
  `(def ,name (fn ,args ,body)))

(defn fact (n)
  (if (= n 0)
      1
      (* n (fact (- n 1)))))
    
;; expands to:
(def fact
  (fn (n)
    (if (= n 0)
        1
        (* n (fact (- n 1))))))

(defmacro cond (&clauses)
  (if (empty? clauses)
      '()
      (let ((clause (head clauses)))
        (if (= (head clause) 'else)
            `(begin ,@(tail clause))
            `(if ,(tail clause) 
               (begin ,@(tail clause)) 
               (cond ,@(tail clauses)))))))

(defn ack (m n)
  (cond
    ((= m 0) (+ n 1))
    ((= n 0) (ack (- m 1) 1))
    (t (ack (- m 1) (ack m (- n 1))))))

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
