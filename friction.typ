#let tr = $t "rel"$
#let nr = $n "rel"$
#let big(x) = $lr((#x), size: #150%)$

A súrlódás "célja" a relatív felszín irányú sebesség csökkentése. Ehhez a
relatív felszín irányú sebességgel ellentétes irányba kell erőt kifejteni.
Ez az erő $F_f <= mu F_n$, ahol $F_n$ a testek között ható erő normál irányú
kompense, $mu$ a súrlódási tényező.

Jelenleg a szimulációban a testek között impulzusok hatnak, amiket el lehet
képzelni nagyon erős, nagyon rövid ideig tartó erőkként. $I = Delta t dot F$,
ahol $Delta t -> 0, F -> infinity$.

A súrlódást is fel tudjuk írni impulzusként: $I_f <= mu I_n$, ahol $I_n$ ismert,
az ütközésből származó impulzus.

Az érintkező pontok $p(t)$ pontok sebessége $dot(p)(t) = v(t) + omega(t) times
big(p(t) - o(t))$, $t$ elhagyható, mert minden ugyan abban az
időpillanatban történik.

Ekkor a relatív sebesség: $v_"rel" = dot(p)_a - dot(p)_b$, a normál irányban
$|v_nr| = n dot v_"rel"$, az ütközés síkjában $v_tr = v_"rel" -
v_nr$

Jelölje $v'_tr$ az ütközés utáni relatív sebességet. A súrlódás "célja", hogy
ez minél kisebb legyen, $v'_tr = 0$.

$
  v'_tr = v'_"rel" - v'_nr
$

$
  v'_a = v_a + J/m_a, quad omega'_a = omega_a + I_a^(-1) big((p_a - o_a) times
  J)\
  dot(p)'_a = v'_a + omega'_a times (p_a - o_a) =
  v_a + J/m_a + big(omega_a + I_a^(-1) big(p_a - o_a) times J) times(p_a - o_a)\
  = v_a + omega_a times (p_a - o_a) + J/m_a
   + big(I_a^(-1) big(p_a - o_a) times J) times (p_a - o_a) quad quad quad
  "legyen" j = |J|, arrow(j) = J/(|J|)\
  = dot(p)_a + (j arrow(j))/m_a + big(I_a^(-1) (p_a - o_a) times (j arrow(j)))
   times (p_a - o_a)\
  = dot(p)_a + j (arrow(j)/m_a + (I_a^(-1) (p_a - o_a) times arrow(j))
   times (p_a - o_a)) = dot(p)_a + j K_a\ \
  dot(p)'_b = dot(p)_b - j (arrow(j)/m_b + (I_b^(-1) (p_b - o_b) times arrow(j))
   times (p_b - o_b)) = dot(p)_b - j K_b
$

$
  v'_"rel" = dot(p)'_a - dot(p)'_b = dot(p)_a - dot(p)_b + j big(K_a + K_b) =
  v_"rel" + j big(K_a + K_b)\
  j big(K_a + K_b) = v'_"rel" - v_"rel"\
  j = (v'_"rel" - v_"rel")/(K_a + K_b) =
  // skaláris osztás pillanatok
  (arrow(j) dot (v'_"rel" - v_"rel"))/(arrow(j) dot (K_a + K_b))
$

$
  I_n = n dot (-epsilon v_"rel" - v_"rel")/(K_a + K_b) =
  n dot (-(epsilon + 1)arrow(j) dot v_"rel")/(arrow(j) dot (K_a + K_b))
$

$
  I_f = min(mu I_n, I_(f "max"))\
  f = (v_"rel" - n (n dot v_"rel"))/(|v_"rel" - n (n dot v_"rel")|)\
  I_(f "max") = f dot (0 - v_"rel")/(K_a + K_b) =
  f dot (-arrow(j) dot v_"rel")/(arrow(j) dot (K_a + K_b))
$


