use nalgebra::Point3;

#[derive(Debug, Clone)]
pub struct AABB {
    start: Point3<f64>,
    end: Point3<f64>,
}

impl AABB {
    /// Create an AABB from a start point and an end point.
    /// The components of the start point have to be smaller than the
    /// components of the end point.
    ///
    /// # Panics
    /// If the start point's components are not smaller than the end point's.
    #[must_use]
    pub fn new(start: Point3<f64>, end: Point3<f64>) -> Self {
        assert_eq!(start.inf_sup(&end), (start, end));
        Self { start, end }
    }

    /// Creates an AABB that encloses both AABBs
    #[must_use]
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            start: self.start.inf(&other.start),
            end: self.end.sup(&other.end),
        }
    }

    /// Check whether two AABBs overlap.
    #[must_use]
    #[inline]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start.x <= other.end.x
            && self.start.y <= other.end.y
            && self.start.z <= other.end.z
            && other.start.x <= self.end.x
            && other.start.y <= self.end.y
            && other.start.z <= self.end.z
    }

    /// Check whether two AABBs overlap.
    #[must_use]
    #[inline]
    pub fn overlaps_xy(&self, other: &Self) -> bool {
        self.start.x <= other.end.x
            && self.start.y <= other.end.y
            && other.start.x <= self.end.x
            && other.start.y <= self.end.y
    }

    /// Check whether two AABBs overlap.
    #[must_use]
    #[inline]
    pub fn overlaps_xz(&self, other: &Self) -> bool {
        self.start.x <= other.end.x
            && self.start.z <= other.end.z
            && other.start.x <= self.end.x
            && other.start.z <= self.end.z
    }

    /// Check whether two AABBs overlap.
    #[must_use]
    #[inline]
    pub fn overlaps_yz(&self, other: &Self) -> bool {
        self.start.y <= other.end.y
            && self.start.z <= other.end.z
            && other.start.y <= self.end.y
            && other.start.z <= self.end.z
    }

    /// Check whether two AABBs overlap.
    #[must_use]
    #[inline]
    pub fn overlaps_x(&self, other: &Self) -> bool {
        self.start.x <= other.end.x && other.start.x <= self.end.x
    }

    /// Check whether two AABBs overlap.
    #[must_use]
    #[inline]
    pub fn overlaps_y(&self, other: &Self) -> bool {
        self.start.y <= other.end.y && other.start.y <= self.end.y
    }

    /// Check whether two AABBs overlap.
    #[must_use]
    #[inline]
    pub fn overlaps_z(&self, other: &Self) -> bool {
        self.start.z <= other.end.z && other.start.z <= self.end.z
    }

    #[must_use]
    #[inline]
    pub const fn start(&self) -> &Point3<f64> {
        &self.start
    }

    #[must_use]
    #[inline]
    pub const fn end(&self) -> &Point3<f64> {
        &self.end
    }
}
