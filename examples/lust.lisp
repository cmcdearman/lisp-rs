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

(let (lex-tok src pos)
    (cond (digit? (peek src pos) (lex-number src pos))
          (letter? (peek src pos) (lex-ident src pos))
          (else (lex-symbol lex-tok src pos))))

(let lex-rules 
;; Reader


