pub struct Cell {
    coordinate: (i32, i32),
    claim: CellClaim,
}

impl Cell {
    fn new(coordinate: (i32, i32)) -> Self {
        let claim = CellClaim { claimants: vec![] };
        Self { coordinate, claim }
    }

    fn add_claim(&mut self, id: Id) {
        self.claim.claimants.push(id);
    }

    fn overlapping_claims(&self) -> bool {
        self.claim.claimants.len() >= 2
    }
}

struct CellClaim {
    claimants: Vec<Id>,
}

struct Id(String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_overlapping() {
        let mut cell = Cell::new((1, 1));
        cell.add_claim(Id("1".into()));
        assert!(!cell.overlapping_claims());
        cell.add_claim(Id("2".into()));
        assert!(cell.overlapping_claims());
    }
}
