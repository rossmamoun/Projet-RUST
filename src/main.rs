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
    #[serde(default)]
    is_enemy: bool,
    #[serde(default)]
    required_items: Vec<String> // IDs des objets requis pour vaincre
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

fn show_objects_at_player_position(objets: &[Objet]) {
    // Find the player and get their position
    let mut player_position = None;
    
    for obj in objets {
        if let Objet::Joueur(joueur) = obj {
            player_position = Some(&joueur.position);
            break;
        }
    }
    
    let player_position = match player_position {
        Some(pos) => pos,
        None => {
            println!("Aucun joueur trouvé!");
            return;
        }
    };
    
    println!("À la position {} vous trouvez:", player_position);
    
    // Find objects at player's position
    let mut found_something = false;
    
    for obj in objets {
        match obj {
            Objet::ObjetStatique(o) if o.position == *player_position => {
                println!("  • Objet Statique: {} ({})", o.nom, o.id);
                found_something = true;
            },
            Objet::ObjetMobile(o) if o.position == *player_position => {
                println!("  • Objet Mobile: {} ({})", o.nom, o.id);
                found_something = true;
            },
            Objet::Pnj(p) if p.position == *player_position => {
                println!("  • PNJ: {}", p.nom);
                found_something = true;
            },
            _ => {}
        }
    }
    
    if !found_something {
        println!("  Rien d'autre ici.");
    }
}

fn interact(objets: &[Objet], pnj_name: &str) {
    // Trouver position et inventaire du joueur
    let mut player_position = None;
    let mut player_inventory = None;
    
    for obj in objets {
        if let Objet::Joueur(joueur) = obj {
            player_position = Some(&joueur.position);
            player_inventory = Some(&joueur.inventaire);
            break;
        }
    }
    
    let player_position = match player_position {
        Some(pos) => pos,
        None => {
            println!("Aucun joueur trouvé!");
            return;
        }
    };
    
    let player_inventory = match player_inventory {
        Some(inv) => inv,
        None => {
            println!("Inventaire non trouvé!");
            return;
        }
    };
    
    // Chercher le PNJ
    for obj in objets {
        if let Objet::Pnj(p) = obj {
            if p.nom.to_lowercase() == pnj_name.to_lowercase() && p.position == *player_position {
                if p.is_enemy {
                    println!("🔥 COMBAT! Vous affrontez {} !", p.nom);
                    
                    // Vérifier les objets requis
                    let mut has_all_items = true;
                    let mut missing_items = Vec::new();
                    
                    for item_id in &p.required_items {
                        if !player_inventory.iter().any(|i| &i.id == item_id) {
                            has_all_items = false;
                            missing_items.push(item_id);
                        }
                    }
                    
                    if has_all_items {
                        println!("Victoire! Vous avez vaincu {} grâce à votre équipement!", p.nom);
                        // Ici: code pour récompenser le joueur
                    } else {
                        println!("Défaite! Vous n'avez pas l'équipement nécessaire.");
                    }
                } else {
                    // Interaction normale
                    println!("Vous interagissez avec {} :", p.nom);
                    println!("\"{}\"", p.description);
                }
                return;
            }
        }
    }
    
    println!("Vous ne voyez pas {} ici.", pnj_name);
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
    show_objects_at_player_position(&objets);

    interact(&objets, "Crocodile");
}
