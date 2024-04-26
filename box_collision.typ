#import "@preview/cetz:0.2.1"

// A szimulációban a téglatesteknek három paramétere van: méret, forgatás, pozíció.

// = Gömb-téglatest ütközés

// Egy gömb és egy téglatest akkor ütköznek, ha a gömb középpontjának a távolsága
// a téglatesttől kisebb, mint a gömb sugara.

// Legyenek a téglatest paraméterei $S$, a téglatest mérete.
// A pozíciótól és forgatástól még eltekintünk.

// Legyenek a gömb paraméterei $R$, a gömb sugara és $O$ a gömb középpontja.

// #cetz.canvas({
//   import cetz.draw
//   draw.rect((0, 0), (10, 10))
//   let position = (4, 4)
//   let rotation = 40deg
//   let scal = (.5, 1)

//   let draw_box(position, rotation, scal, radius: 0) = {
//     draw.group({
//       draw.translate(position)
//       draw.rotate(z: rotation)
//       let x_size = scal.at(0) + radius
//       let y_size = scal.at(1) + radius
//       draw.rect((-x_size, -y_size), (x_size, y_size), radius: radius)
//     })
//   }

//   draw_box(position, rotation, scal)
//   draw_box(position, rotation, scal, radius: 0.5)
// })

= Téglatest-téglatest ütközés

Két téglatest sokféleképpen ütközhet, ezekből csak néhány esetet kell kezelni:
- Csúcs-csúcs: ritkán történik meg, egy frame múlva már más eset lesz, nem kell
  külön kezelni.
- Csúcs-él: ritkán történik meg, egy frame múlva már más eset lesz, nem kell
  külön kezelni.
- *Csúcs-oldal:* gyakran megtörténhet, a csúcs ütközések nagy valószínűséggel
  erre fognak változni, kezelni kell.
- *Él-él:* egy másik gyakori eset, kezelni kell.
- Él-oldal: ez csúcs-oldal és él-él ütközéskre bontható.
- Oldal-oldal: es csúcs-oldal és él-él ütközésekre bontható.

== Csúcs-oldal ütközés

Adott két téglatest a transzformációs mátrixukkal: $M_1, M_2$.

Az ütközést detektálhatjuk úgy, hogy az egyik téglatestet a axis-aligned-ra
transformáljuk, és a másik téglatesten elvégezzük ugyan ezt a transzformációt.

Az axis-aligned-ra transzformáció megegyezik az inverz transzformáció szóval
az egyik téglatest tejles transzformációja $M_1^(-1) dot M_1 = I$, a másiké
$M_1^(-1) dot M_2$.

Az ütköző csúcsokat a következő képpen találhatjuk meg: 

$p in P "(az egyik téglatest csúcsai)",\
p' = M_1^(-1) dot M_2 dot p,\
-0.5 < x(p) < 0.5, -0.5 < y(p) < 0.5, -0.5 < z(p) < 0.5
$

Ha több, mint egy pontot kapunk, akkor átlagolhatjuk, hogy jó közelítést
kapjunk.

Az ütközési normál a kapott pont legnagyobb komponensének az irányába mutat.

A másik téglatest ütközési pontját az első pontból és az ütközési normálból
könnyen megkapjuk.

Ezt a számítást mindkét irányba ki kell számolni, hogy ellenőrizzük az összes
csúcs-oldal ütközést.

== Él-él ütközés

// $
//   D^2 = (P_1 + t_1 dot V_1 - P_2 - t_2 dot V_2)^2\
//   D^2 = (P_1 - P_2 + t_1 dot V_1 - t_2 dot V_2)^2\
//   D^2 = (P_1 - P_2)^2 + t_1^2 dot V_1^2 + t_2^2 dot V_2^2
//     + 2(P_1 - P_2) dot t_1 V_1 - 2(P_1 - P_2) dot t_2 V_2
//     - 2 t_1 t_2 V_1 V_2
// $

// Adott kettő él a normál szerinti síkra vetítve, ezenek az éleknek kell a
// metszéspontja:
$
  // B + t_1 (A - B) = D + t_2 (C - D)\
  // x_B + t_1 (x_A - x_B) = x_D + t_2 (x_C - x_D)\
  // y_B + t_1 (y_A - y_B) = y_D + t_2 (y_C - y_D)\
  // (x_C - x_D)/(y_C - y_D) (y_B - y_D + t_1 (y_A - y_B)) = t_2 (x_C - x_D)
// \
// t_1 = (x_D - x_B + t_2 (x_C - x_D))/(x_A - x_B)\
// t_2 = (y_B - y_D + t_1 (y_A - y_B))/(y_C - y_D)
P_1 + t_1 V_1 + t_N N = P_2 + t_2 V_2\
\
x_P_1 + t_1 x_V_1 + t_N x_N = x_P_2 + t_2 x_V_2\
y_P_1 + t_1 y_V_1 + t_N y_N = y_P_2 + t_2 y_V_2\
z_P_1 + t_1 z_V_1 + t_N z_N = z_P_2 + t_2 z_V_2\
\
t_1 x_V_1 + t_N x_N - t_2 x_V_2 = x_P_2 - x_P_1\
t_1 y_V_1 + t_N y_N - t_2 y_V_2 = y_P_2 - y_P_1\
t_1 z_V_1 + t_N z_N - t_2 z_V_2 = z_P_2 - z_P_1\
$

#pagebreak()
=== Egyenes-síklap metszet
Egyenes: $ e(t) = P + t dot V $
Sík: $ x = 0.5 $
Metszet: $
  x_P + t dot x_V = 0.5\
  t = (0.5 - x_P)/x_v\
  "ha" x_V = 0, "akkor az egyenes párhuzamos a síkkal és nincs metszet"\
$
