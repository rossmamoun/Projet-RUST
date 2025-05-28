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
    sous_position: String,
    inventaire: Vec<ObjetInventaire>,  // On utilise ObjetInventaire au lieu de ObjetStatique
    puissance: u32,
    hp: u32
}


#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type_inventaire")]
enum ObjetInventaire {
    #[serde(rename = "objet")]
    ObjetStatique(ObjetStatique),
    
    #[serde(rename = "aliment")]
    Aliment(Aliment)
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
    // Trouver la position et sous-position du joueur
    let mut player_position = None;
    let mut player_sous_position = None;

    for obj in objets {
        if let Objet::Joueur(joueur) = obj {
            player_position = Some(&joueur.position);
            player_sous_position = Some(&joueur.sous_position);
            break;
        }
    }

    let (player_position, player_sous_position) = match (player_position, player_sous_position) {
        (Some(pos), Some(sous_pos)) => (pos, sous_pos),
        _ => {
            println!("Aucun joueur trouvé!");
            return;
        }
    };

    // Trouver le nom du lieu et du sous-lieu pour un affichage plus agréable
    let lieu_nom = objets.iter()
        .find_map(|obj| {
            if let Objet::Lieu(l) = obj {
                if &l.id == player_position {
                    Some(&l.nom)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(player_position);

    let sous_lieu_nom = objets.iter()
        .find_map(|obj| {
            if let Objet::SousLieu(sl) = obj {
                if &sl.id == player_sous_position {
                    Some(&sl.nom)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(player_sous_position);

    println!("Vous êtes à : {} - {}", lieu_nom, sous_lieu_nom);
    println!("À la position {} vous trouvez:", sous_lieu_nom);

    let mut found_something = false;

    for obj in objets {
        match obj {
            Objet::ObjetStatique(os) if os.sous_position == *player_sous_position => {
                println!("  • Objet Statique: {} ({})", os.nom, os.id);
                found_something = true;
            },
            Objet::ObjetMobile(o) if o.sous_position == *player_sous_position => {
                println!("  • Objet Mobile: {} ({})", o.nom, o.id);
                found_something = true;
            },
            Objet::Pnj(p) if p.sous_position == *player_sous_position => {
                println!("  • PNJ: {}", p.nom);
                found_something = true;
            },
            Objet::Aliment(a) if a.sous_position == *player_sous_position => {
                println!("  • Aliment: {} (+{} HP)", a.nom, a.hp);
                found_something = true;
            },
            _ => {}
        }
    }

    if !found_something {
        println!("  Rien d'autre ici.");
    }
}

fn interact(objets: &mut Vec<Objet>, pnj_name: &str, joueurs: &mut Vec<Joueur>) {
    // Trouver position du joueur et son index
    let mut player_position = None;
    let mut player_sous_position = None;
    let mut player_index = None;
    
    for (i, obj) in objets.iter().enumerate() {
        if let Objet::Joueur(joueur) = obj {
            player_position = Some(joueur.position.clone());
            player_sous_position = Some(joueur.sous_position.clone());
            player_index = Some(i);
            break;
        }
    }
    
    let (player_position, player_sous_position) = match (player_position, player_sous_position) {
        (Some(pos), Some(sous_pos)) => (pos, sous_pos),
        _ => {
            println!("Aucun joueur trouvé!");
            return;
        }
    };
    
    let player_index = match player_index {
        Some(idx) => idx,
        None => {
            println!("Index du joueur non trouvé!");
            return;
        }
    };
    
    // Chercher le PNJ et son index
    for (i, obj) in objets.iter().enumerate() {
        if let Objet::Pnj(p) = obj {
            if p.nom.to_lowercase() == pnj_name.to_lowercase() && 
               p.position == player_position && 
               p.sous_position == player_sous_position {
                
                if p.is_enemy {
                    // Logique de combat existante
                    println!("🔥 COMBAT! Vous affrontez {} !", p.nom);
                    println!("{}: {}", p.nom, p.description);
                    
                    // Vérifier les objets requis
                    if let Some(Objet::Joueur(joueur)) = objets.get(player_index) {
                        let mut has_all_items = true;
                        
                        for item_id in &p.required_items {
                            let item_found = joueur.inventaire.iter().any(|inv_item| {
                                match inv_item {
                                    ObjetInventaire::ObjetStatique(obj) => &obj.id == item_id,
                                    ObjetInventaire::Aliment(alim) => &alim.id == item_id,
                                }
                            });
                            
                            if !item_found {
                                has_all_items = false;
                                break;
                            }
                        }
                        
                        if has_all_items {
                            println!("Victoire! Vous avez vaincu {} grâce à votre équipement!", p.nom);
                        } else {
                            println!("Défaite! Vous n'avez pas l'équipement nécessaire.");
                        }
                    }
                } else {
                    // Interaction amicale
                    println!("Vous interagissez avec {} :", p.nom);
                    println!("\"{}\"", p.description);
                    
                    // Vérifier si le PNJ a des objets dans son inventaire
                    if let Objet::Pnj(pnj) = &objets[i] {
                        if !pnj.inventaire.is_empty() {
                            // Récupérer l'ID de l'objet
                            let objet_id = &pnj.inventaire[0];
                            
                            // Trouver l'objet correspondant à cet ID
                            let mut objet_trouve = None;
                            for obj in objets.iter() {
                                if let Objet::ObjetStatique(o) = obj {
                                    if o.id == *objet_id {
                                        objet_trouve = Some(o.clone());
                                        break;
                                    }
                                }
                            }
                            
                            if let Some(objet) = objet_trouve {
                                println!("\n{} vous propose un objet : {}", p.nom, objet.nom);
                                println!("Description : {}", objet.description);
                                println!("\nVoulez-vous le prendre? (o/n)");
                                
                                let mut reponse = String::new();
                                io::stdin().read_line(&mut reponse).expect("Erreur de lecture");
                                let reponse = reponse.trim().to_lowercase();
                                
                                if reponse == "o" || reponse == "oui" {
                                    // Supprimer l'ID de l'inventaire du PNJ
                                    if let Some(Objet::Pnj(pnj)) = objets.get_mut(i) {
                                        if !pnj.inventaire.is_empty() {
                                            pnj.inventaire.remove(0);
                                        }
                                        
                                        // Ajouter l'objet à l'inventaire du joueur
                                        if let Some(Objet::Joueur(joueur)) = objets.get_mut(player_index) {
                                            // Modifier l'objet pour qu'il soit dans l'inventaire
                                            let mut objet_final = objet.clone();
                                            objet_final.position = "inventaire".to_string();
                                            
                                            // Convertir en ObjetInventaire::ObjetStatique
                                            let inv_obj = ObjetInventaire::ObjetStatique(objet_final);
                                            
                                            println!("→ Objet '{}' ajouté à votre inventaire !", objet.nom);
                                            joueur.inventaire.push(inv_obj);
                                            
                                            // Synchronisation avec joueurs
                                            if let Some(joueur_local) = joueurs.get_mut(0) {
                                                joueur_local.inventaire = joueur.inventaire.clone();
                                            }
                                        }
                                    }
                                } else {
                                    println!("Vous avez refusé l'objet.");
                                }
                            } else {
                                println!("{} a un objet, mais impossible de le trouver dans le monde.", p.nom);
                            }
                        } else {
                            println!("{} n'a rien à vous offrir.", p.nom);
                        }
                    }
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

fn capture_objets_statiques(objets: &mut Vec<Objet>, joueurs: &mut Vec<Joueur>) {
    let mut player_index = None;
    let mut objets_a_ajouter = vec![];

    // Trouver le joueur et sa sous_position
    let mut player_sous_position = String::new();
    for (i, obj) in objets.iter().enumerate() {
        if let Objet::Joueur(joueur) = obj {
            player_sous_position = joueur.sous_position.clone();
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

    // Trouver tous les objets statiques ET les aliments au sous-lieu du joueur
    objets.retain(|obj| {
        match obj {
            Objet::ObjetStatique(o) if o.sous_position == player_sous_position => {
                println!("→ Objet '{}' capturé dans le sous-lieu !", o.nom);
                
                // Convertir en ObjetInventaire::ObjetStatique
                let inv_obj = ObjetInventaire::ObjetStatique(o.clone());
                objets_a_ajouter.push(inv_obj);
                false // retirer de la liste globale
            },
            Objet::Aliment(a) if a.sous_position == player_sous_position => {
                println!("→ Aliment '{}' (+{} HP) capturé dans le sous-lieu !", a.nom, a.hp);
                
                // Convertir en ObjetInventaire::Aliment
                let inv_obj = ObjetInventaire::Aliment(a.clone());
                objets_a_ajouter.push(inv_obj);
                false // retirer de la liste globale
            },
            _ => true,
        }
    });

    // Ajouter les objets capturés à l'inventaire du joueur
    if let Objet::Joueur(joueur) = &mut objets[player_index] {
        joueur.inventaire.extend(objets_a_ajouter);
    }

    // Synchronisation avec la structure joueurs
    for obj in objets {
        if let Objet::Joueur(j) = obj {
            if let Some(joueur) = joueurs.get_mut(0) {
                joueur.inventaire = j.inventaire.clone();
            }
        }
    }
}

fn consommer_aliment(joueurs: &mut Vec<Joueur>, objets: &mut Vec<Objet>) {
    if let Some(joueur) = joueurs.get_mut(0) {
        if joueur.hp >= 100 {
            println!("🛑 Vous avez déjà tous vos HP (100). Impossible de consommer un aliment !");
            return;
        }
        
        // Chercher un aliment dans l'inventaire (uniquement Aliment)
        if let Some((index, item)) = joueur.inventaire.iter().enumerate().find(|(_, inv_obj)| {
            matches!(inv_obj, ObjetInventaire::Aliment(_))
        }) {
            // Obtenir le nom et les HP directement depuis l'aliment
            if let ObjetInventaire::Aliment(aliment) = &item {
                let nom = &aliment.nom;
                let gain_hp = aliment.hp;
                
                let hp_avant = joueur.hp;
                joueur.hp = (joueur.hp + gain_hp).min(100);
                let hp_gagne = joueur.hp - hp_avant;
                
                println!("🍽️ Vous consommez : {}", nom);
                println!("❤️  Vous regagnez {} HP ! HP actuel : {}", hp_gagne, joueur.hp);
                joueur.inventaire.remove(index);

                // Synchronisation avec la liste globale d'objets
                for obj in objets.iter_mut() {
                    if let Objet::Joueur(j) = obj {
                        j.inventaire = joueur.inventaire.clone();
                        j.hp = joueur.hp;
                    }
                }
            }
        } else {
            println!("Vous n'avez pas d'aliment à consommer !");
        }
    }
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
                nom: joueur.nom.clone(),
                fruit_de_demon: joueur.fruit_de_demon.clone(),
                position: joueur.position.clone(),
                sous_position: joueur.sous_position.clone(),
                inventaire: joueur.inventaire.clone(),
                puissance: joueur.puissance,
                hp: joueur.hp
            }),
            Objet::Lieu(lieu) => lieux.push(Lieu {
                id: lieu.id.clone(),
                nom: lieu.nom.clone(),
                description: lieu.description.clone(),
                connections: lieu.connections.clone(),
                required_key: lieu.required_key.clone()
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
        print!("8. Mini-jeux amusants\n");
        println!("9. Consommer un aliment");
        println!("Q. Quitter");
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
                capture_objets_statiques(&mut objets, &mut joueurs);
                // Mettre à jour l'inventaire du joueur dans joueurs
            }
            "3" => {
                // Parler/Combattre un PNJ
                println!("Nom du PNJ ?");
                let mut nom = String::new();
                io::stdin().read_line(&mut nom).unwrap();
                let nom = nom.trim();
                interact(&mut objets, nom, &mut joueurs);  // Maintenant avec &mut
            }
            "4" => {
                // Inventaire
                if let Some(joueur) = joueurs.get(0) {
                    println!("Inventaire :");
                    if joueur.inventaire.is_empty() {
                        println!("  (vide)");
                    } else {
                        for item in &joueur.inventaire {
                            match item {
                                ObjetInventaire::Aliment(a) => {
                                    println!("  • 🍖 Aliment: {} (+{} HP)", a.nom, a.hp);
                                },
                                ObjetInventaire::ObjetStatique(o) => {
                                    println!("  • 📦 Objet: {}", o.nom);
                                }
                            }
                        }
                    }
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
                consommer_aliment(&mut joueurs, &mut objets);
            }
            "Q" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}