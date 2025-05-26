use serde::Deserialize;
use std::fs;
use std::io::{self, Write};


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
    inventaire: Vec<ObjetStatique>,
    force: u32, // Force du joueur
    agilite: u32, // Agilité du joueur
    intelligence: u32, // Intelligence du joueur
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

fn afficher_carte(lieux: &[Lieu]) {
    println!("\n========== CARTE DU MONDE ==========");
    for lieu in lieux {
        println!("🗺️  {} [{}]", lieu.nom, lieu.id);
        if lieu.connections.is_empty() {
            println!("   └─ Aucune connexion.");
        } else {
            for (i, conn) in lieu.connections.iter().enumerate() {
                let symbole = if i == lieu.connections.len() - 1 { "└─" } else { "├─" };
                println!("   {} {} → {}", symbole, conn.orientation, conn.destination);
            }
        }
        println!("-------------------------------------");
    }
    println!("=====================================\n");
}

fn afficher_stats(joueur: &Joueur) {
    println!("--- Statistiques du joueur ---");
    println!("Force       : {}", joueur.force);
    println!("Agilité     : {}", joueur.agilite);
    println!("Intelligence: {}", joueur.intelligence);
}

fn mini_jeu_devinette() {
    use rand::Rng;
    let secret = rand::rng().random_range(1..=10);
    println!("Je pense à un nombre entre 1 et 10. Devine !");
    let mut essais = 0;
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entre un nombre !");
                continue;
            }
        };
        essais += 1;
        if guess == secret {
            println!("Bravo ! Trouvé en {} essais.", essais);
            break;
        } else if guess < secret {
            println!("C'est plus grand !");
        } else {
            println!("C'est plus petit !");
        }
    }
}

fn mini_jeu_pile_ou_face() {
    use rand::Rng;
    println!("Pile ou face ? (pile/face)");
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).unwrap();
    let choix = choix.trim().to_lowercase();
    let tirage = if rand::rng().random_bool(0.5) { "pile" } else { "face" };    println!("Résultat : {}", tirage);
    if choix == tirage {
        println!("Gagné !");
    } else {
        println!("Perdu !");
    }
}

fn mini_jeu_calcul() {
    use rand::Rng;
    let a = rand::rng().random_range(1..=10);
    let b = rand::rng().random_range(1..=10);
    println!("Combien font {} + {} ?", a, b);
    let mut reponse = String::new();
    io::stdin().read_line(&mut reponse).unwrap();
    let reponse: i32 = match reponse.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Ce n'est pas un nombre !");
            return;
        }
    };
    if reponse == a + b {
        println!("Bonne réponse !");
    } else {
        println!("Faux ! La bonne réponse était {}.", a + b);
    }
}

fn main() {
    let data = fs::read_to_string("data.json").expect("Impossible de lire le fichier");
    let mut objets: Vec<Objet> = serde_json::from_str(&data).expect("Erreur de parsing JSON");

    // Séparer les objets de type Joueur et Lieu
    let mut lieux: Vec<Lieu> = Vec::new();
    let mut joueurs: Vec<Joueur> = Vec::new();

    for obj in &objets {
        match obj {
            Objet::Joueur(joueur) => joueurs.push(Joueur {
                position: joueur.position.clone(),
                inventaire: joueur.inventaire.clone(),
                force: joueur.force,
                agilite: joueur.agilite,
                intelligence: joueur.intelligence,
            }),
            Objet::Lieu(lieu) => lieux.push(Lieu {
                id: lieu.id.clone(),
                nom: lieu.nom.clone(),
                connections: lieu.connections.clone(),
            }),
            _ => {}
        }
    }

    // Boucle de jeu interactive
    loop {
        println!("\n--- Menu du jeu ---");
        println!("1. Se déplacer");
        println!("2. Ramasser les objets");
        println!("3. Parler/Combattre un PNJ");
        println!("4. Voir l'inventaire");
        println!("5. Voir la description du lieu");
        println!("6. Afficher la carte du monde");
        println!("7. Afficher les statistiques du joueur");
        println!("8. Mini-jeux amusants");
        println!("9. Quitter");
        print!("Votre choix : ");
        io::stdout().flush().unwrap();

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).unwrap();
        let choix = choix.trim();

        match choix {
            "1" => {
                // Déplacement
                if let Some(joueur) = joueurs.get_mut(0) {
                    println!("Dans quelle direction ? (N/S/E/O)");
                    let mut dir = String::new();
                    io::stdin().read_line(&mut dir).unwrap();
                    let dir = dir.trim();
                    move_joueur(joueur, dir, &lieux);
                    // Mettre à jour la position du joueur dans objets
                    for obj in &mut objets {
                        if let Objet::Joueur(j) = obj {
                            j.position = joueur.position.clone();
                        }
                    }
                }
            }
            "2" => {
                // Ramasser les objets
                capture_objets_statiques(&mut objets);
                // Mettre à jour l'inventaire du joueur dans joueurs
                for obj in &objets {
                    if let Objet::Joueur(j) = obj {
                        if let Some(joueur) = joueurs.get_mut(0) {
                            joueur.inventaire = j.inventaire.clone();
                        }
                    }
                }
            }
            "3" => {
                // Parler/Combattre un PNJ
                println!("Nom du PNJ ?");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).unwrap();
                let nom = nom.trim();
                interact(&objets, nom);
            }
            "4" => {
                // Inventaire
                if let Some(joueur) = joueurs.get(0) {
                    let noms_inventaire: Vec<&String> = joueur.inventaire.iter().map(|o| &o.nom).collect();
                    println!("Inventaire : {:?}", noms_inventaire);
                }
            }
            "5" => {
                // Description du lieu
                if let Some(joueur) = joueurs.get(0) {
                    let pos = &joueur.position;
                    if let Some(lieu) = lieux.iter().find(|l| &l.id == pos) {
                        println!("Vous êtes à : {} - {}", lieu.nom, lieu.id);
                        println!("{}", lieu.nom);
                        println!("Connexions :");
                        for conn in &lieu.connections {
                            println!("  -> {} vers {}", conn.orientation, conn.destination);
                        }
                    }
                }
                show_objects_at_player_position(&objets);
            }
            "6" => {
                // Afficher la carte du monde
                afficher_carte(&lieux);
            }
            "7" => {
                // Afficher les statistiques du joueur
                if let Some(joueur) = joueurs.get(0) {
                    afficher_stats(joueur);
                }
            }
            "8" => {
                loop {
                    println!("\n--- Mini-jeux ---");
                    println!("1. Devinette");
                    println!("2. Pile ou face");
                    println!("3. Calcul mental");
                    println!("4. Retour au menu principal");
                    print!("Votre choix : ");
                    io::stdout().flush().unwrap();

                    let mut jeu_choix = String::new();
                    io::stdin().read_line(&mut jeu_choix).unwrap();
                    let jeu_choix = jeu_choix.trim();

                    match jeu_choix {
                        "1" => mini_jeu_devinette(),
                        "2" => mini_jeu_pile_ou_face(),
                        "3" => mini_jeu_calcul(),
                        "4" => break,
                        _ => println!("Choix invalide."),
                    }
                }
            }
            "9" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}



