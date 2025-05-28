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
    sous_position:String,
    is_key: bool, // Indique si c'est une clé pour un lieu
}

#[derive(Debug, Deserialize, Clone)]
struct Aliment {
    id: String,
    nom: String,
    description: String,
    position: String,
    sous_position:String,
    hp: u32, // Points de vie restaurés
}

#[derive(Debug, Deserialize, Clone)]
struct ObjetMobile {
    id: String,
    nom: String,
    description: String,
    position: String,
    sous_position:String,
}

#[derive(Debug, Deserialize, Clone)]
struct Attaque {
    id: String,
    nom: String,
    description: String,
    puissance: u32,
}

#[derive(Debug, Deserialize, Clone)]
struct FruitDuDemon {
    id: String,
    nom: String,
    description: String,
    sous_position:String,
    pouvoir: String,
    position: String, 
    attaque: Vec<String>,
}


#[derive(Debug, Deserialize, Clone)]
struct Joueur {
    nom: String,
    fruit_de_demon: Option<FruitDuDemon>,
    position: String,
    sous_position:String,
    inventaire: Vec<ObjetStatique>,
    puissance: u32,
    hp: u32
}

#[derive(Debug, Deserialize, Clone)]
struct Pnj {
    nom: String,
    description: String,
    position: String,
    sous_position:String,
    #[serde(default)]
    is_enemy: bool,
    #[serde(default)]
    required_items: Vec<String>, // IDs des objets requis pour vaincre
    inventaire: Vec<String>,
    puissance: u32, 
    hp: u32, 
    attaques: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Lieu {
    id: String,
    nom: String,
    description: String,
    connections: Vec<Connection>,
    required_key: String, // Clé requise pour accéder à ce lieu
}

#[derive(Debug, Deserialize, Clone)]
struct SousLieu {
    id: String,
    nom: String,
    position: String,
    description: String,
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

    #[serde(rename = "FruitDuDemon")]
    FruitDuDemon(FruitDuDemon),
    
    #[serde(rename = "aliment")]
    Aliment(Aliment),
    
    #[serde(rename = "souslieu")]
    SousLieu(SousLieu),
    
    #[serde(rename = "lieu")]
    Lieu(Lieu),

    #[serde(rename = "Attaque")]
    Attaque(Attaque),
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


/// Déplace le joueur dans une direction donnée, si une connexion existe.
fn move_inside(
    joueur: &mut Joueur,
    orientation: &str,
    objets: &Vec<Objet>,
) -> Result<(), String> {

    let mut sous_lieux: Vec<SousLieu> = Vec::new();

    // Extraire tous les sous-lieux distincts à partir des objets
    for obj in objets {
        match obj {
            Objet::SousLieu(sous_lieu) => sous_lieux.push(SousLieu {
                nom: sous_lieu.nom.clone(),
                position: sous_lieu.position.clone(),
                description: sous_lieu.description.clone(),
                id: sous_lieu.id.clone(),
                connections: sous_lieu.connections.clone(),
            }),
            
            
            _ => {}
        }
    }

    // Ensuite tu peux réutiliser la logique d’avant
    let current_sous_lieu = sous_lieux
        .iter()
        .find(|sl| sl.id == joueur.sous_position && sl.position == joueur.position);

    if let Some(sous_lieu) = current_sous_lieu {
        if let Some(conn) = sous_lieu.connections.iter().find(|c| c.orientation == orientation) {
            if sous_lieux
                .iter()
                .any(|sl| sl.id == conn.destination && sl.position == joueur.position)
            {
                joueur.sous_position = conn.destination.clone();
                println!(
                    "Le joueur se déplace vers le sous-lieu {} ({})",
                    conn.destination, orientation
                );
                return Ok(());
            } else {
                return Err(format!(
                    "La destination {} n'existe pas dans ce lieu.",
                    conn.destination
                ));
            }
        } else {
            return Err(format!("Pas de connexion vers {}", orientation));
        }
    }

    Err("Sous-lieu actuel introuvable.".to_string())
}




fn move_joueur(
    joueur: &mut Joueur,
    direction: &str,
    lieux: &Vec<Lieu>,
    objets_mobiles: &mut Vec<ObjetMobile>,
    objets_statiques: &mut Vec<ObjetStatique>,
    objets: &Vec<Objet>
) {
    if direction != "N" && direction != "S" && direction != "E" && direction != "O" {
        println!("Direction invalide. Utilisez N, S, E ou O.");
        return;
    }

    let bateau_present = objets_mobiles.iter().any(|o| o.nom == "Bateau" && o.position == joueur.position && o.sous_position == joueur.sous_position);
    if !bateau_present {
        println!("Il n'y a pas de bateau ici pour vous déplacer, cherchez le bateau.");
        return;
    }


    for lieu in lieux {
        if lieu.id == joueur.position {
            if let Some(conn) = lieu.connections.iter().find(|c| c.orientation == direction) {
                if let Some(destination_lieu) = lieux.iter().find(|l| l.id == conn.destination) {
                    // Vérifie si une clé est requise
                    if !destination_lieu.required_key.is_empty() {
                        let a_cle = joueur.inventaire.iter().any(|obj| obj.id == destination_lieu.required_key);
                        if !a_cle {
                            // 🔍 Chercher l’objet statique correspondant à la clé
                            if let Some(objet) = objets_statiques.iter().find(|o| o.id == destination_lieu.required_key) {
                                println!(
                                    "Vous avez besoin de la clé '{}' pour entrer dans ce lieu !\n→ Description : {}",
                                    objet.nom, objet.description
                                );
                            } else {
                                println!(
                                    "Clé requise non trouvée dans la base d'objets : ID '{}'",
                                    destination_lieu.required_key
                                );
                            }
                            return;
                        }
                    }

                    joueur.position =  destination_lieu.id.clone();
                    let mut sous_lieux: Vec<SousLieu> = Vec::new();

                        // Extraire tous les sous-lieux distincts à partir des objets
                    for obj in objets {
                        match obj {
                            Objet::SousLieu(sous_lieu) => sous_lieux.push(SousLieu {
                                nom: sous_lieu.nom.clone(),
                                position: sous_lieu.position.clone(),
                                description: sous_lieu.description.clone(),
                                id: sous_lieu.id.clone(),
                                connections: sous_lieu.connections.clone(),
                            }),
            
            
                        _ => {}

                        }
                    }

                    // Rechercher le premier sous-lieu commençant par "SE" dans la nouvelle position
                    if let Some(sous_lieu_se) = sous_lieux.iter().find(|sl| sl.position == joueur.position && sl.id.starts_with("SE")) {
                        joueur.sous_position = sous_lieu_se.id.clone();
                        for objet in objets_mobiles.iter_mut() {
                            if objet.nom == "Bateau" && objet.position == lieu.id {
                                objet.position = destination_lieu.id.clone();
                                objet.sous_position = sous_lieu_se.id.clone();
                                break;
                            }
                        }
                    }

                    

                    //joueur.sous_position = destination_lieu.id.clone(); // Mettre à jour la sous-position du joueur
                    println!("Déplacement vers {}", destination_lieu.nom);
                    return;
                }
            } else {
                println!("Aucune connexion dans cette direction !");
                return;
            }
        }
    }

    println!("Lieu actuel invalide !");
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
    println!("puissance    : {}", joueur.puissance);
    println!("HP          : {}", joueur.hp);
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
    let mut objets_mobiles: Vec<ObjetMobile> = Vec::new();
    let mut objets_statiques: Vec<ObjetStatique> = Vec::new();
    let mut sous_lieux: Vec<SousLieu> = Vec::new();

    for obj in &objets {
        match obj {
            Objet::Joueur(joueur) => joueurs.push(Joueur {
                nom: joueur.nom.clone(),
                position: joueur.position.clone(),
                inventaire: joueur.inventaire.clone(),
                hp: joueur.hp,
                puissance: joueur.puissance,
                sous_position: joueur.sous_position.clone(),
                fruit_de_demon: joueur.fruit_de_demon.clone(),
            }),
            Objet::Lieu(lieu) => lieux.push(Lieu {
                id: lieu.id.clone(),
                nom: lieu.nom.clone(),
                connections: lieu.connections.clone(),
                required_key: lieu.required_key.clone(),
                description: lieu.description.clone(),
            }),
            Objet::ObjetMobile(objet) => objets_mobiles.push(ObjetMobile {
                nom: objet.nom.clone(),
                position: objet.position.clone(),
                description: objet.description.clone(),
                id: objet.id.clone(),
                sous_position: objet.sous_position.clone(),
            }),
            Objet::ObjetStatique(objet) => objets_statiques.push(ObjetStatique {
                    nom: objet.nom.clone(),
                    position: objet.position.clone(),
                    description: objet.description.clone(),
                    id: objet.id.clone(),
                    sous_position: objet.sous_position.clone(),
                    is_key: objet.is_key,
                }),
            Objet::SousLieu(sous_lieu) => sous_lieux.push(SousLieu {
                nom: sous_lieu.nom.clone(),
                position: sous_lieu.position.clone(),
                description: sous_lieu.description.clone(),
                id: sous_lieu.id.clone(),
                connections: sous_lieu.connections.clone(),
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
        println!("9. Se deplacer dans l'ile");
        println!("10. Quitter");
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
                    move_joueur(joueur, dir, &lieux, &mut objets_mobiles, &mut objets_statiques, &objets);
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
                        println!("Vous êtes à : {} - {} - {}", lieu.nom, lieu.id, joueur.sous_position);
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
                // Déplacement
                if let Some(joueur) = joueurs.get_mut(0) {
                    println!("Dans quelle direction ? (N/S/E/O)");
                    let mut dir = String::new();
                    io::stdin().read_line(&mut dir).unwrap();
                    let dir = dir.trim();
                    move_inside(joueur, dir, &objets);
                    // Mettre à jour la position du joueur dans objets
                    for obj in &mut objets {
                        if let Objet::Joueur(j) = obj {
                            j.sous_position = joueur.sous_position.clone();
                        }
                    }
                }
            }
            "10" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}



