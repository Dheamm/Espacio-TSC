use rand::Rng;

static ADJECTIVES: &[&str] = &[
    "Valiente", "Reflexivo", "Empático", "Sereno", "Curioso",
    "Comprometido", "Resiliente", "Atento", "Solidario", "Crítico",
];

static NOUNS: &[&str] = &[
    "Trabajador", "Estudiante", "Practicante", "Acompañante", "Observador",
];

pub fn generate_alias() -> String {
    let mut rng = rand::thread_rng();
    let adj = ADJECTIVES[rng.gen_range(0..ADJECTIVES.len())];
    let noun = NOUNS[rng.gen_range(0..NOUNS.len())];
    let num: u16 = rng.gen_range(10..99);
    format!("{}{}{}", adj, noun, num)
}
