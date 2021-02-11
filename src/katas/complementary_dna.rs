#![allow(unused)]

fn dna_strand(dna: &str) -> String {
    dna.chars()
        .map(|x| match x {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => x
        }).collect()
}

#[test]
fn returns_expected() {
    assert_eq!(dna_strand("AAAA"),"TTTT");
    assert_eq!(dna_strand("ATTGC"),"TAACG");
    assert_eq!(dna_strand("GTAT"),"CATA");
}
