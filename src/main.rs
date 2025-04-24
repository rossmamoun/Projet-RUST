use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Connection {
    orientation: String,
    destination: String,
}

#[derive(Debug, Deserialize)]
struct ObjetStatique {
    id: String,
    nom: String,
    description: String,
    position: String,
}

#[derive(Debug, Deserialize)]
struct ObjetMobile {
    id: String,
    nom: String,
    description: String,
    position: String,
}

#[derive(Debug, Deserialize)]
struct Joueur {
    position: String,
    inventaire: Vec<ObjetStatique>
}

#[derive(Debug, Deserialize)]
struct Pnj {
    nom: String,
    description: String,
    position: String,
}

#[derive(Debug, Deserialize)]
struct Lieu {
    id: String,
    nom: String,
    connections: Vec<Connection>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Objet {
    #[serde(rename = "ObjetMobile")]
    ObjetMobile(ObjetMobile),
    
    #[serde(rename = "ObjetStatique")]
    ObjetStatique(ObjetStatique),
    
    #[serde(rename = "Pnj")]
    Pnj(Pnj),
    
    #[serde(rename = "Joueur")]
    Joueur(Joueur),
    
    #[serde(rename = "lieu")]
    Lieu(Lieu)
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
            },
            Objet::Pnj(p) => println!("PNJ : {} à la position {}", p.nom, p.position),
            Objet::ObjetStatique(o) => println!("Objet Statique : {} ({})", o.nom, o.id),
            Objet::ObjetMobile(o) => println!("Objet Mobile : {} ({})", o.nom, o.id),
        }
    }
}
