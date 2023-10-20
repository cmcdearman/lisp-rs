;; Lust compiler written in Lust
; (require 'lust)
(import 'llvm)
(import 'std)

;; Utils
(struct (span start end))
(struct (meta span))

;; Lexer
(struct (token type-tag meta))

(let (lex src pos tokens) 
    (cond ))

;; Reader


