use crate::claim::Claim;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Coordinates(pub i32, pub i32);

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
}

#[derive(Debug, PartialEq)]
pub struct Fabric<'a> {
    claims: HashMap<Coordinates, Vec<&'a Claim>>,
}

impl<'a> Fabric<'a> {
    pub fn new() -> Self {
        Self {
            claims: HashMap::new(),
        }
    }

    pub fn process_claim(&mut self, claim: &'a Claim) {
        for x in claim.x_range() {
            for y in claim.y_range() {
                self.claim_cell(Coordinates(x, y), claim);
            }
        }
    }

    fn claim_cell(&mut self, cell: Coordinates, claim: &'a Claim) {
        let claims = self.claims.entry(cell).or_insert(vec![]);
        claims.push(claim);
    }

    pub fn count_overlapping_cells(&self) -> i32 {
        let mut area = 0;
        for claims in self.claims.values() {
            if claims.len() > 1 {
                area += 1
            }
        }
        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_claim() {
        let mut fabric = Fabric::new();
        let claim = Claim::new("1", 1, 1, 1, 1);
        fabric.claim_cell(Coordinates(1, 1), &claim);
        let mut claims: HashMap<Coordinates, Vec<&Claim>> = HashMap::new();
        claims.insert(Coordinates(1, 1), vec![&claim]);
        let expected = Fabric { claims };
        assert_eq!(fabric, expected);
    }

    #[test]
    fn test_process_claim() {
        let mut fabric = Fabric::new();
        let claim = Claim::new("1", 5, 5, 2, 2);
        fabric.process_claim(&claim);
        let mut claims: HashMap<Coordinates, Vec<&Claim>> = HashMap::new();
        claims.insert(Coordinates(5, 5), vec![&claim]);
        claims.insert(Coordinates(5, 6), vec![&claim]);
        claims.insert(Coordinates(6, 5), vec![&claim]);
        claims.insert(Coordinates(6, 6), vec![&claim]);
        let expected = Fabric { claims };
        assert_eq!(fabric, expected);
    }

    #[test]
    fn test_count_overlapping() {
        let mut fabric = Fabric::new();
        let claim1 = Claim::new("1", 1, 1, 1, 1);
        fabric.claim_cell(Coordinates(1, 1), &claim1);
        assert_eq!(fabric.count_overlapping_cells(), 0);
        let claim2 = Claim::new("2", 1, 1, 1, 1);
        fabric.claim_cell(Coordinates(1, 1), &claim2);
        assert_eq!(fabric.count_overlapping_cells(), 1);
        let claim3 = Claim::new("2", 1, 2, 1, 1);
        fabric.claim_cell(Coordinates(1, 1), &claim3);
        assert_eq!(fabric.count_overlapping_cells(), 1);
    }
}
