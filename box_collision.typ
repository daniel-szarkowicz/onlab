#import "@preview/cetz:0.2.1"

// A szimulációban a téglatesteknek három paramétere van: méret, forgatás, pozíció.

= Gömb-téglatest ütközés

Egy gömb és egy téglatest akkor ütköznek, ha a gömb középpontjának a távolsága
a téglatesttől kisebb, mint a gömb sugara.

Legyenek a téglatest paraméterei $S$, a téglatest mérete.
A pozíciótól és forgatástól még eltekintünk.

Legyenek a gömb paraméterei $R$, a gömb sugara és $O$ a gömb középpontja.

#cetz.canvas({
  import cetz.draw
  draw.rect((0, 0), (10, 10))
  let position = (4, 4)
  let rotation = 40deg
  let scal = (.5, 1)

  let draw_box(position, rotation, scal, radius: 0) = {
    draw.group({
      draw.translate(position)
      draw.rotate(z: rotation)
      let x_size = scal.at(0) + radius
      let y_size = scal.at(1) + radius
      draw.rect((-x_size, -y_size), (x_size, y_size), radius: radius)
    })
  }

  draw_box(position, rotation, scal)
  draw_box(position, rotation, scal, radius: 0.5)
})
