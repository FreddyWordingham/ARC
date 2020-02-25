//! Hit-type enumeration.

/// Hit enumeration implementation.
#[derive(Debug)]
pub enum Hit {
    /// Scattering event.
    Scattering(f64),
    /// Cell collision.
    Cell(f64),
    /// Interface collision.
    Interface(f64),
    /// Interface collision, followed by a close cell collision.
    InterfaceCell(f64),
}

impl Hit {
    /// Construct a new scattering event instance.
    #[inline]
    #[must_use]
    pub fn new_scattering(dist: f64) -> Self {
        debug_assert!(dist > 0.0);

        Self::Scattering(dist)
    }

    /// Construct a new cell crossing instance.
    #[inline]
    #[must_use]
    pub fn new_cell(dist: f64) -> Self {
        debug_assert!(dist > 0.0);

        Self::Cell(dist)
    }

    /// Construct a new interface crossing instance.
    #[inline]
    #[must_use]
    pub fn new_interface(dist: f64) -> Self {
        debug_assert!(dist > 0.0);

        Self::Interface(dist)
    }

    /// Construct a new interface crossing instance, followed by a potential cell crossing.
    #[inline]
    #[must_use]
    pub fn new_interface_cell(dist: f64) -> Self {
        debug_assert!(dist > 0.0);

        Self::InterfaceCell(dist)
    }

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(scat_dist: f64, cell_dist: f64, inter_dist: Option<f64>, bump_dist: f64) -> Self {
        debug_assert!(scat_dist > 0.0);
        debug_assert!(cell_dist > 0.0);
        debug_assert!(inter_dist.is_none() || inter_dist.expect("Something went wrong.") > 0.0);
        debug_assert!(bump_dist > 0.0);

        if cell_dist <= scat_dist {
            if let Some(inter_dist) = inter_dist {
                if (inter_dist - cell_dist).abs() < (2.0 * bump_dist) {
                    return Self::new_interface_cell(inter_dist.min(cell_dist));
                } else if cell_dist < inter_dist {
                    return Self::new_cell(cell_dist);
                }

                return Self::new_interface(inter_dist);
            }

            return Self::new_cell(cell_dist);
        }

        if let Some(inter_dist) = inter_dist {
            if inter_dist <= scat_dist {
                return Self::new_interface(inter_dist);
            }
        }

        Self::new_scattering(scat_dist)
    }
}
