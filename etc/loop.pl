loop(A, X, Y) :- eq(A, [X|B]), eq(B, [Y|A]).

loopBad(A, B, X) :- eq(A, [X|B]), eq(B, [X|A]).
