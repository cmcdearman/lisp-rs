;;;; A file for feature/syntax ideas

;; You can use `def` to bind values to names. Note
;; that we don't call these variables because they
;; can't vary. They are immutable.
(def x 10)

;; `def` can bind to lambdas
(def gcd (lambda (a b)
  (if (= b 0) 
      a
      (gcd b (% a b)))))

;; `def` can also bind application forms
(def (gcd a b)
  (if (= b 0) 
      a
      (gcd b (% a b))))

;; you can use `match` to pattern match or `if` for conditionals
(def (gcd a b)
  (match b
    ((0) a)
    ((_) (gcd b (% a b)))))

(def (fib n)
  (match n
    ((0) 0)
    ((1) 1)
    ((_) (+ (fib (- n 1)) (fib (- n 2))))))

(def ack (m n)
  (cond ((= m 0) (+ n 1))
        ((= n 0) (ack (- m 1) 1))
        (ack (- m 1) (ack m (- n 1)))))

(def (map f xs)
  (if (empty? xs) nil
      (cons (f (car xs)) (map f (cdr xs)))))

(def (fib n)
  (if (<= n 1) 
      n
      (+ (fib (- n 1)) (fib (- n 2)))))

(println (fib 45))

;; `let` expressions are used to bind names to values
;; in a local scope. The names are immutable.
(let ((a 10) 
      (b 5))
  (+ a b))


;;; ==================================================================
;;; *                            Macros                              *
;;; ==================================================================

;; `defmacro` is used to define macros. Macros are
;; like functions, but they are evaluated at compile
;; time. They can be used to define new syntax.
(defmacro (backwards . body)
  (cons 'begin
	(reverse body)))

(defmacro (while condition . body)
  `(let loop ()
     (cond (,condition
	    (begin . ,body)
	    (loop)))))

(defmacro (when test . expr)
  (list 'if test (cons 'progn expr)))

;; Macro calls are like function calls, but the arguments
;; are not evaluated. Instead, they are passed to the macro
;; as unevaluated forms.

(when (= 1 1)
  (println "1 is equal to 1"))

;;; ==================================================================
;;; *                         Data Types                             *
;;; ==================================================================

(data (point x y))
  
(class Eq ()
  (def (eq self other)
    (raise :NotImplementedError)))

(class Ord (Eq PartialOrd)
  (def (cmp self other)
    (raise :NotImplementedError)))