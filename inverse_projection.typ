#let depth = $italic("depth")$
#let near = $italic("near")$
#let far = $italic("far")$
#let ndc = $italic("ndc")$

$
  depth = (1\/z - 1\/near)/(1\/far - 1\/near)\
  ndc = 2*depth - 1
$

Goal get the $ndc$ of any $z = t dot far + (1-t) dot near$ without knowing $far$ or
$near$.

$
  depth = (1\/z - 1\/near)/(1\/far - 1\/near)
  = (1\/(t dot far + (1-t) dot near) - 1\/near)/(1\/far - 1\/near)\
  = (1\/(t dot far + near - t dot near) - 1\/near)/((near - far)\/(far dot near))\
  = ((near - (t dot far + near - t dot near))
  /((t dot far + near - t dot near) dot near))\/((near - far)/(far dot near))\
  = (far dot near dot (near - (t dot far + near - t dot near)))
  /((near - far) dot (t dot far + near - t dot near) dot near)\
  = (far dot t dot (near - far))
  /((near - far) dot (t dot far + near - t dot near))\
  = (far dot t) /(t dot far + near - t dot near)\
$

Unfortunately the goal cannot be reached, $far$ and $near$ always have to be known.
