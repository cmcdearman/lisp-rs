(def gcd
  (fn (a b) 
    (if (= b 0) 
      a 
      (gcd b (% a b)))))

(def gcd 
  (fn (a b) 
    (match b
      ([0 a]
      [_ (gcd b (% a b))]))))