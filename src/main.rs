use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Joueur {
    r#type: String,
    position: String,
}

#[derive(Debug, Deserialize)]
struct Connection {
    orientation: String,
    destination: String,
}

#[derive(Debug, Deserialize)]
struct Lieu {
    r#type: String,
    id: String,
    nom: String,
    connections: Vec<Connection>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Objet {
    Joueur(Joueur),
    Lieu(Lieu),
}

fn main() {
    // Lire le fichier JSON
    let data = fs::read_to_string("data.json").expect("Impossible de lire le fichier");

    // Désérialiser en une liste d'objets
    let objets: Vec<Objet> = serde_json::from_str(&data).expect("Erreur de parsing JSON");

    // Afficher le résultat
    for obj in &objets {
        match obj {
            Objet::Joueur(j) => println!("Joueur à la position : {:?}", j.position),
            Objet::Lieu(l) => {
                println!("Lieu : {} ({})", l.nom, l.id);
                for conn in &l.connections {
                    println!("  -> {} vers {}", conn.orientation, conn.destination);
                }
            }
        }
    }
}
