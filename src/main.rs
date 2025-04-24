use serde::Deserialize;
use std::fs;
use std::io;

#[derive(Debug, Deserialize)]
struct Connection {
    orientation: String,
    destination: String,
}

#[derive(Debug, Deserialize, Clone)]
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
                println!("  • PNJ: {} - {}", p.nom, p.description);
                found_something = true;
            },
            _ => {}
        }
    }
    
    if !found_something {
        println!("  Rien d'autre ici.");
    }
}

fn capture_objets_statiques(objets: &mut Vec<Objet>) {
    let mut player_index = None;
    let mut objets_a_ajouter = vec![];

    // Trouver le joueur et sa position
    let mut player_position = String::new();
    for (i, obj) in objets.iter().enumerate() {
        if let Objet::Joueur(joueur) = obj {
            player_position = joueur.position.clone();
            player_index = Some(i);
            break;
        }
    }

    let player_index = match player_index {
        Some(index) => index,
        None => {
            println!("Aucun joueur trouvé !");
            return;
        }
    };

    // Trouver tous les objets statiques à la position du joueur
    objets.retain(|obj| {
        match obj {
            Objet::ObjetStatique(o) if o.position == player_position => {
                println!("→ Objet '{}' capturé !", o.nom);
                objets_a_ajouter.push(o.clone());
                false // retirer de la liste globale
            },
            _ => true,
        }
    });

    // Ajouter les objets capturés à l'inventaire du joueur
    if let Objet::Joueur(joueur) = &mut objets[player_index] {
        joueur.inventaire.extend(objets_a_ajouter);
    }
}

fn move_joueur(objets: &mut Vec<Objet>, direction: &str) {
    // Trouver la position actuelle du joueur
    let mut joueur_position = None;
    let mut joueur_index = None;

    for (i, obj) in objets.iter().enumerate() {
        if let Objet::Joueur(j) = obj {
            joueur_position = Some(j.position.clone());
            joueur_index = Some(i);
            break;
        }
    }

    let joueur_position = match joueur_position {
        Some(pos) => pos,
        None => {
            println!("Aucun joueur trouvé !");
            return;
        }
    };

    let joueur_index = joueur_index.unwrap();

    // Trouver le lieu correspondant
    let mut nouvelle_position = None;

    for obj in objets.iter() {
        if let Objet::Lieu(lieu) = obj {
            if lieu.id == joueur_position {
                // Chercher la destination dans la direction demandée
                for conn in &lieu.connections {
                    if conn.orientation == direction {
                        nouvelle_position = Some(conn.destination.clone());
                        break;
                    }
                }
            }
        }
    }

    match nouvelle_position {
        Some(dest) => {
            if let Objet::Joueur(joueur) = &mut objets[joueur_index] {
                joueur.position = dest.clone();
                println!("Le joueur se déplace vers la pièce '{}'", dest);
            }
        },
        None => println!("Aucune sortie vers la direction '{}'", direction),
    }
}


fn main() {
    // Lire le fichier JSON
    let data = fs::read_to_string("data.json").expect("Impossible de lire le fichier");

    // Désérialiser en une liste d'objets
    let mut objets: Vec<Objet> = serde_json::from_str(&data).expect("Erreur de parsing JSON");

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
    capture_objets_statiques(&mut objets);
    // Afficher uniquement le joueur et son inventaire après capture
    for obj in &objets {
        if let Objet::Joueur(j) = obj {
            let noms_inventaire: Vec<&String> = j.inventaire.iter().map(|o| &o.nom).collect();
            println!(
                "Joueur à la position : {:?}, inventaire : {:?}",
                j.position,
                noms_inventaire
            );
        }
        
    }
    show_objects_at_player_position(&objets);

   
    
    move_joueur(&mut objets, "E");
    
    // Afficher les objets dans la nouvelle position
    show_objects_at_player_position(&objets);
    
}