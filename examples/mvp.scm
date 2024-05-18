;; Lust is a simple Lisp-like language based on the idea of term rewriting.
;; It is a functional language with a simple syntax and semantics.
;; Terms are rewritten using pattern matching and substitution.
;; The language is dynamically typed and has first-class functions.

;; Special Forms:
;; - def: define a variable
;; - let: bind variables in a scope
;; - match: pattern match a term
;; - list: create a list
;; - fn: create a lambda function
;; - and: short-circuiting logical and
;; - or: short-circuiting logical or
;; - quote: prevent evaluation of a term
;; - quasiquote: prevent evaluation of a term, except for unquoted terms
;; - unquote: evaluate a term in a quasiquote
;; - unquote-splicing: evaluate a term in a quasiquote and splice the result
;; - module: define a module

;; def declarations
(def x 42)

(def (fib n)
  (if (<= n 1)
      n
      (+ (fib (- n 1)) (fib (- n 2)))))

(def (fib n)
  (if (<= n 1)
      n
      (+ (fib (- n 1)) (fib (- n 2)))))

(def (fib-iter n)
  (let loop ((a 0) (b 1) (i n))
    (if (= i 0)
      a
      (loop b (+ a b) (- i 1)))))
    
;; this could also be done with a `match` expression
(def (fib n)
  (match n
    (0 0)
    (1 1)
    (n (+ (fib (- n 1)) (fib (- n 2))))))

;; call the `fib` function
(fib 10)

;; Note: The pattern matching in the `fib` function is not the same as in other languages.
;; You might think `(fib 0)` would be syntax sugar for binding `n` to `0` in the `fib` function.
;; However, it is actually binding the whole term `(fib 0)` to `0` in the `fib` function.
;; This is equivalent to telling the runtime that it can replace `(fib 0)` with `0`.

;; let expressions
(let ((x 1) (y 2)) (+ x y))

;; quote expressions
(quote (1 2 3))
'(1 2 3)
;; > '(1 2 3)
;; (1 2 3)

;; quasiquote/unquote expressions
(quasiquote (1 2 (unquote (+ 1 2)) 4))
`(1 2 ,(+ 1 2) 4)

;; lambda expressions
(fn (x) (+ x 1))
;; lambda call
((fn (x) (+ x 1)) 2)

;; and expressions (short-circuiting)
(and (< 1 2) (> 2 1))

;; or expressions (short-circuiting)
(or (< 1 2) (> 2 1))

;; not expressions
(not (< 1 2))

;; list expressions
[1 2 (+ 1 2) 4]

;; this is equivalent to
(List.new 1 2 (+ 1 2) 4)

;; maps
{:a 1 :b 2}

;; this is equivalent to
(Map.new :a 1 :b 2)

;; Maps that use keyword symbols as keys are called records. 
;; Keywords are symbols that evaluate to themselves they
;; are used to represent named arguments and are often 
;; used as keys in maps.
:foo
; => :foo

;; map update
(Map.insert {:a 1 :b 2} :a 3)

;; map access
(Map.get {:a 1 :b 2} :a)

;; map remove
(Map.remove {:a 1 :b 2} :a)

;; sets
#{1 2 3}

;; this is equivalent to
(Set.new 1 2 3)

;; Macros are rules for transforming terms at compile time.
;; They are used to define new syntax and to optimize code.
;; Macros are defined using the `macro` special form.
(macro (if cond then else) 
  `(match ,cond 
     (#t ,then) 
     (#f ,else)))

;; module declarations
(module Vector
  (def (new) {:data []})
  (def (new &xs) {:data xs}))

(def (fib n) 
  (match n
    (0 0)
    (1 1)
    (n (+ (fib (- n 1)) (fib (- n 2))))))
