push([], X, X:[]).
push(A:B, X, A:C) :- push(B, X, C).

pop(X:[], X, []).
pop(X:A, Y, X:B) :- pop(A, Y, B).

unshift([], X, X:[]).
unshift(A, X, X:A).

shift(X:A, X, A).

contains(X:_, X).
contains(_:A, X) :- contains(A, X).

eq(X, X).
