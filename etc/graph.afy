node(a).
node(b).
node(c).
node(d).

edge(a, b).
edge(b, c).
edge(c, d).
edge(a, d).

reachable(X, X).
reachable(X, Y) :- edge(X, A), reachable(A, Y).