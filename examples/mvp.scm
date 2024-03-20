;; Lust is a simple Lisp-like language based on the idea of term rewriting.
;; It is a functional language with a simple syntax and semantics.
;; Terms are rewritten using pattern matching and substitution.
;; The language is dynamically typed and has first-class functions.

;; def declarations
(def x 42)

;; def declarations with pattern matching
(def (fib 0) 0)
(def (fib 1) 1)
(def (fib n) (+ (fib (- n 1)) (fib (- n 2))))

;; You might think `(fib 0)` would be syntax sugar for binding `n` to `0` in the `fib` function.
;; However, it is actually binding the whole term `(fib 0)` to `0` in the `fib` function.
;; This is equivalent to telling the runtime that it can replace `(fib 0)` with `0`.

;; let expressions
(let ((x 1) (y 2)) (+ x y))

;; quote expressions
(quote (1 2 3))
'(1 2 3)

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

;; maps
{'a 1 'b 2}

;; map update
(Map.insert {'a 1 'b 2} 'a 3)

;; map access
(Map.get {'a 1 'b 2} 'a)

;; map remove
(Map.remove {'a 1 'b 2} 'a)

;; sets
#{1 2 3}

;; module declarations
(module Vector
  (macro (new) `{:data []})
  (macro (new &xs) `{:data ,@xs}))

