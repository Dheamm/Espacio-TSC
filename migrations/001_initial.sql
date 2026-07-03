CREATE TABLE IF NOT EXISTS mood_checkins (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mood TEXT NOT NULL,
    emoji TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    alias TEXT NOT NULL,
    content TEXT NOT NULL,
    parent_id INTEGER REFERENCES posts(id) ON DELETE CASCADE,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS recursos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    file_url TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_posts_parent ON posts(parent_id);
CREATE INDEX IF NOT EXISTS idx_recursos_category ON recursos(category);

INSERT INTO recursos (title, description, category, file_url) VALUES
    ('Guía de Entrevista Clínica', 'Protocolo estructurado para la primera entrevista con el consultante.', 'Entrevista', '/recursos/entrevista-clinica.pdf'),
    ('Formato de Anamnesis', 'Plantilla oficial para levantamiento de historia de vida.', 'Formatos Burocráticos', '/recursos/anamnesis.pdf'),
    ('Manejo de Crisis en Contexto Clínico', 'Procedimientos ante situaciones de crisis psicosocial.', 'Manejo de Casos', '/recursos/manejo-crisis.pdf'),
    ('Derivación Interinstitucional', 'Formularios y protocolos de derivación a redes de apoyo.', 'Formatos Burocráticos', '/recursos/derivacion.pdf'),
    ('Técnicas de Escucha Activa', 'Herramientas prácticas para fortalecer el vínculo profesional.', 'Entrevista', '/recursos/escucha-activa.pdf'),
    ('Genograma y Ecomapa', 'Guía para construcción e interpretación de herramientas sistémicas.', 'Manejo de Casos', '/recursos/genograma.pdf');
