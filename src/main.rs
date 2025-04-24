use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
struct ObjetMobile {
    id: String,
    nom: String,
    description: String,
    position: String,
}

#[derive(Debug, Deserialize, Clone)]
struct Joueur {
    position: String,
    inventaire: Vec<ObjetStatique>
}

#[derive(Debug, Deserialize, Clone)]
struct Pnj {
    nom: String,
    description: String,
    position: String,
    #[serde(default)]
    is_enemy: bool,
    #[serde(default)]
    required_items: Vec<String> // IDs des objets requis pour vaincre
}

#[derive(Debug, Deserialize, Clone)]
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


fn move_joueur(joueur: &mut Joueur, direction: &str, lieux: &Vec<Lieu>) {
    if(direction != "N" && direction != "S" && direction != "E" && direction != "O") {
        println!("Direction invalide. Utilisez N, S, E ou O.");
        return;
    }
    // Parcourir les lieux pour trouver celui où le joueur se trouve
    for lieu in lieux {
        if lieu.id == joueur.position {
            // Chercher la connexion qui correspond à la direction
            if let Some(conn) = lieu.connections.iter().find(|&conn| conn.orientation == direction) {
                joueur.position = conn.destination.clone();
                println!("Déplacement du joueur vers {}", conn.destination);
                return; 
            } else {
                println!("Le joueur ne peut pas aller dans cette direction car il n'y a pas de destination.");
                return;  
            }
        }
    }
    println!("Le joueur ne se trouve dans aucun lieu valide.");
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





fn main() {

    let data = fs::read_to_string("data.json").expect("Impossible de lire le fichier");
    let mut objets: Vec<Objet> = serde_json::from_str(&data).expect("Erreur de parsing JSON");

    // Séparer les objets de type Joueur et Lieu
    let mut lieux: Vec<Lieu> = Vec::new();
    let mut joueurs: Vec<Joueur> = Vec::new();

    for obj in &objets {  // Use a reference here instead
        match obj {
            Objet::Joueur(joueur) => joueurs.push(Joueur {
                position: joueur.position.clone(),
                inventaire: joueur.inventaire.clone(),
            }),
            Objet::Lieu(lieu) => lieux.push(Lieu {
                id: lieu.id.clone(),
                nom: lieu.nom.clone(),
                connections: lieu.connections.clone(),
            }),
            _ => {}
        }
    }

    

     // Affichage des joueurs
     println!("Joueurs:");
     for joueur in &joueurs {
         println!(" Position: {}", joueur.position);
     }
 
     // Affichage des lieux
     println!("\nLieux:");
     for lieu in &lieux {
         println!("ID: {}, Nom: {}, Connexions:", lieu.id, lieu.nom);
         for conn in &lieu.connections {
             println!("  -> {} vers {}", conn.orientation, conn.destination);
         }
     }

     interact(&objets, "Crocodile"); 

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
     // TEST::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
    if let Some(joueur) = joueurs.get_mut(0) {  // On prend le premier joueur

        println!("Lieu actuel du joueur  {}", joueur.position);
        move_joueur(joueur, "E", &lieux);  // Exemple de déplacement vers la direction "E"
        println!("Lieu actuel du joueur  {}", joueur.position);
        move_joueur(joueur, "P", &lieux);  // Exemple de déplacement vers la direction "E"
        println!("Lieu actuel du joueur  {}", joueur.position);

    }

    show_objects_at_player_position(&objets);


    
}



