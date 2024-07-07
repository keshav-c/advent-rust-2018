use crate::claim::Claim;

pub struct Cell<'a> {
    coordinates: (i32, i32),
    claims: Vec<&'a Claim>,
}

impl<'a> Cell<'a> {
    fn new(coordinates: (i32, i32)) -> Self {
        let claims = Vec::new();
        Self {
            coordinates,
            claims,
        }
    }

    fn add_claim(&mut self, claim: &'a Claim) -> Result<(), ()> {
        if self.coordinates == claim.coordinates {
            self.claims.push(claim);
            Ok(())
        } else {
            Err(())
        }
    }

    fn overlapping_claims(&self) -> bool {
        self.claims.len() >= 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_claim() {
        let mut cell = Cell::new((1, 1));
        let claim = dummy_claim("1", (1, 1));
        assert!(cell.add_claim(&claim).is_ok())
    }

    #[test]
    fn test_cannot_add_claim() {
        let mut cell = Cell::new((1, 1));
        let claim = dummy_claim("1", (1, 2));
        assert!(cell.add_claim(&claim).is_err())
    }

    #[test]
    fn test_add_and_overlapping() {
        let mut cell = Cell::new((1, 1));
        let claim1 = dummy_claim("1", (1, 1));
        cell.add_claim(&claim1).unwrap();
        assert!(!cell.overlapping_claims());
        let claim2 = dummy_claim("2", (1, 1));
        cell.add_claim(&claim2).unwrap();
        assert!(cell.overlapping_claims());
    }

    fn dummy_claim(id: &str, coordinates: (i32, i32)) -> Claim {
        Claim::new(id, coordinates, 1, 1)
    }
}
