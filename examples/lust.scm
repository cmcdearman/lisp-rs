;; Lust compiler written in Lust
(require 'lust)

;; Reader
(let (read s) 
    (setq read (lambda () (read-from-minibuffer "Lust> ")))
    (setq s (read))
    (while (not (equal s "quit"))
        (princ (lust-eval s))
        (setq s (read))))

