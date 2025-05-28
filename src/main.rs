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
    #[serde(default)]
    attaques: Vec<String>,
    #[serde(default)]
    is_entraineur: bool, // Indique si c'est un PNJ entraîneur
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

fn show_objects_at_player_position(objets: &[Objet], lieux: &[Lieu], joueur: &Joueur) {
    let pos = &joueur.position;
    let sous_pos = &joueur.sous_position;

    // Afficher le lieu principal
    if let Some(lieu) = lieux.iter().find(|l| &l.id == pos) {
        println!("Vous êtes à : {} - {}", lieu.nom, lieu.id);
        println!("{}", lieu.description);
        println!("Connexions :");
        for conn in &lieu.connections {
            // Chercher le nom du lieu de destination
            let nom_dest = lieux.iter()
                .find(|l| l.id == conn.destination)
                .map(|l| l.nom.as_str())
                .unwrap_or("Lieu inconnu");
            println!("  -> {} vers {} ({})", conn.orientation, nom_dest, conn.destination);
        }
    }

    // Afficher le sous-lieu si présent
    let mut souslieu_trouve = false;
    for obj in objets {
        if let Objet::SousLieu(sl) = obj {
            if &sl.position == pos && &sl.id == sous_pos {
                println!("\nSous-lieu : {} - {}", sl.nom, sl.id);
                println!("{}", sl.description);
                souslieu_trouve = true;
            }
        }
    }
    if !souslieu_trouve {
        println!("\nAucun sous-lieu spécifique ici.");
    }

    // Afficher les objets et PNJ du sous-lieu
    println!("\nDans ce sous-lieu, vous trouvez :");
    let mut found = false;
    for obj in objets {
        match obj {
            Objet::ObjetStatique(o) if &o.position == pos && &o.sous_position == sous_pos => {
                println!("  • Objet Statique: {} ({})", o.nom, o.id);
                found = true;
            }
            Objet::ObjetMobile(o) if &o.position == pos && &o.sous_position == sous_pos => {
                println!("  • Objet Mobile: {} ({})", o.nom, o.id);
                found = true;
            }
            Objet::Pnj(p) if &p.position == pos && &p.sous_position == sous_pos => {
                println!("  • PNJ: {}", p.nom);
                found = true;
            }
            _ => {}
        }
    }
    if !found {
        println!("  Rien d'autre ici.");
    }
}

fn interact(objets: &mut Vec<Objet>, pnj_name: &str, joueurs: &mut Vec<Joueur>) {
    // Trouver position du joueur et son index
    let mut player_position = None;
    let mut player_index = None;
    
    for (i, obj) in objets.iter().enumerate() {
        if let Objet::Joueur(joueur) = obj {
            player_position = Some(joueur.position.clone());
            player_index = Some(i);
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
            if p.nom.to_lowercase() == pnj_name.to_lowercase() && p.position == player_position {
                if p.is_enemy {
                    // Logique de combat existante
                    println!("🔥 COMBAT! Vous affrontez {} !", p.nom);
                    println!("{}: {}", p.nom, p.description);
                    
                    // Vérifier les objets requis
                    if let Some(Objet::Joueur(joueur)) = objets.get(player_index) {
                        let mut has_all_items = true;
                        
                        for item_id in &p.required_items {
                            if !joueur.inventaire.iter().any(|i| &i.id == item_id) {
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
                                            
                                            println!("→ Objet '{}' ajouté à votre inventaire !", objet_final.nom);
                                            joueur.inventaire.push(objet_final);
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
                    for obj in objets {
                        if let Objet::Joueur(j) = obj {
                            if let Some(joueur) = joueurs.get_mut(0) {
                                joueur.inventaire = j.inventaire.clone();
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

    // Add synchronization at the end
    for obj in objets {
        if let Objet::Joueur(j) = obj {
            if let Some(joueur) = joueurs.get_mut(0) {
                joueur.inventaire = j.inventaire.clone();
            }
        }
    }
}

fn capture_fruit_de_demon(objets: &mut Vec<Objet>, joueur: &mut Joueur) {
    // Chercher un fruit du démon dans la même sous_position
    if let Some((idx, fruit)) = objets.iter().enumerate().find_map(|(i, obj)| {
        if let Objet::FruitDuDemon(f) = obj {
            if f.sous_position == joueur.sous_position {
                return Some((i, f.clone()));
            }
        }
        None
    }) {
        println!("Un fruit du démon ({}) est trouvé dans ta zone !", fruit.nom);
        match &joueur.fruit_de_demon {
            None => {
                println!("Vous n'avez pas de fruit du démon. Voulez-vous le manger ? (o/n)");
                let mut reponse = String::new();
                io::stdin().read_line(&mut reponse).unwrap();
                let reponse = reponse.trim().to_lowercase();
                if reponse == "o" || reponse == "oui" {
                    joueur.fruit_de_demon = Some(fruit);
                    objets.remove(idx);
                    println!("Vous avez mangé le fruit du démon !");
                } else {
                    println!("Vous avez ignoré le fruit du démon.");
                }
            }
            Some(fruit_actuel) => {
                println!("Vous avez déjà le fruit '{}'. Voulez-vous l'échanger avec '{}' ? (o/n)", fruit_actuel.nom, fruit.nom);
                let mut reponse = String::new();
                io::stdin().read_line(&mut reponse).unwrap();
                let reponse = reponse.trim().to_lowercase();
                if reponse == "o" || reponse == "oui" {
                    // Remettre l'ancien fruit dans les objets
                    objets.push(Objet::FruitDuDemon(fruit_actuel.clone()));
                    joueur.fruit_de_demon = Some(fruit);
                    objets.remove(idx);
                    println!("Vous avez échangé votre fruit du démon !");
                } else {
                    println!("Vous gardez votre fruit actuel.");
                }
            }
        }
    } else {
        println!("Aucun fruit du démon trouvé dans votre zone.");
    }
}

fn entrainement(objets: &mut [Objet], joueur: &mut Joueur) {
    // Chercher le premier PNJ entraîneur dans la même sous_position
    let entraineur = objets.iter().find_map(|obj| {
        if let Objet::Pnj(pnj) = obj {
            if pnj.sous_position == joueur.sous_position && pnj.is_entraineur {
                Some(pnj)
            } else {
                None
            }
        } else {
            None
        }
    });

    if let Some(pnj) = entraineur {
        println!("{} : \"Salut {} ! Prêt pour un nouvel entraînement ? Aujourd'hui, on va travailler ta force avec un exercice spécial !\"", pnj.nom, joueur.nom);
        println!("Voulez-vous commencer l'entraînement ? (o/n)");
        let mut reponse = String::new();
        io::stdin().read_line(&mut reponse).unwrap();
        let reponse = reponse.trim().to_lowercase();
        if reponse == "o" || reponse == "oui" {
            println!("{} : \"Super ! Fais 20 pompes virtuelles ! ... C'est bien, tu progresses !\"", pnj.nom);
            joueur.puissance += 10;
            println!("Votre puissance augmente de 10 ! ({} points)", joueur.puissance);

            // Si le joueur a un fruit, interaction spéciale
            if let Some(fruit) = &mut joueur.fruit_de_demon {
                println!("{} : \"Tu as un fruit du démon ! On va aussi entraîner tes attaques spéciales. Prêt pour un exercice de maîtrise du pouvoir '{}' ? (o/n)\"", pnj.nom, fruit.nom);
                let mut rep_fruit = String::new();
                io::stdin().read_line(&mut rep_fruit).unwrap();
                let rep_fruit = rep_fruit.trim().to_lowercase();
                if rep_fruit == "o" || rep_fruit == "oui" {
                    println!("{} : \"Concentre ton énergie... et lance une attaque !\"", pnj.nom);
                    for attaque_id in &fruit.attaque {
                        for obj in objets.iter_mut() {
                            if let Objet::Attaque(attaque) = obj {
                                if &attaque.id == attaque_id {
                                    attaque.puissance += 10;
                                    println!("L'attaque '{}' gagne +10 puissance ({} points) !", attaque.nom, attaque.puissance);
                                }
                            }
                        }
                    }
                    println!("Tes attaques de fruit du démon sont renforcées !");
                } else {
                    println!("{} : \"Dommage, tu t'entraîneras une prochaine fois sur tes pouvoirs.\"", pnj.nom);
                }
            }
            println!("Entraînement terminé !");
        } else {
            println!("Vous refusez l'entraînement.");
        }
    } else {
        println!("Aucun entraîneur n'est présent dans votre zone.");
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

fn afficher_stats(joueur: &Joueur, objets: &[Objet]) {
    println!("--- Statistiques du joueur ---");
    println!("Nom         : {}", joueur.nom);
    println!("Puissance   : {}", joueur.puissance);
    match &joueur.fruit_de_demon {
        Some(fruit) => {
            println!("Fruit       : {} ({})", fruit.nom, fruit.pouvoir);
            println!("Attaques    :");
            for attaque_id in &fruit.attaque {
                if let Some(Objet::Attaque(attaque)) = objets.iter().find(|obj| {
                    matches!(obj, Objet::Attaque(a) if &a.id == attaque_id)
                }) {
                    println!("  • {} (puissance: {}): {}", attaque.nom, attaque.puissance, attaque.description);
                } else {
                    println!("  • Attaque inconnue: {}", attaque_id);
                }
            }
        }
        None => println!("Fruit       : Aucun"),
    }
    println!("HP       : {}", joueur.hp);
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

    // Demander le nom du joueur
    println!("Bienvenue dans One Piece ! Quel est ton nom ?");
    let mut nom_joueur = String::new();
    io::stdin().read_line(&mut nom_joueur).unwrap();
    let nom_joueur = nom_joueur.trim();

    // Mettre à jour le nom du joueur dans la structure Joueur
    if let Some(joueur) = joueurs.get_mut(0) {
        joueur.nom = nom_joueur.to_string();

        // Chercher un fruit du démon dans la même sous_position
        if let Some((idx, fruit)) = objets.iter().enumerate().find_map(|(i, obj)| {
            if let Objet::FruitDuDemon(f) = obj {
                if f.sous_position == joueur.sous_position {
                    return Some((i, f.clone()));
                }
            }
            None
        }) {
            println!("Un fruit du démon ({}) est trouvé dans ta zone ! Voulez-vous le manger ? (o/n)", fruit.nom);
            let mut reponse = String::new();
            io::stdin().read_line(&mut reponse).unwrap();
            let reponse = reponse.trim().to_lowercase();
            if reponse == "o" || reponse == "oui" {
                joueur.fruit_de_demon = Some(fruit);
                objets.remove(idx); // Retirer le fruit de la liste des objets
                println!("Vous avez mangé le fruit du démon !");
            } else {
                println!("Vous avez ignoré le fruit du démon.");
            }
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
        println!("6. Capturer un fruit du démon");
        println!("7. Afficher les statistiques du joueur");
        println!("8. S'entraîner avec un PNJ entraîneur");
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
                    let noms_inventaire: Vec<&String> = joueur.inventaire.iter().map(|o| &o.nom).collect();
                    println!("Inventaire : {:?}", noms_inventaire);
                }
            }
            "5" => {
                 // Description du lieu, sous-lieu et objets/PNJ du sous-lieu
                if let Some(joueur) = joueurs.get(0) {
                    show_objects_at_player_position(&objets, &lieux, joueur);
                }
            }
            "6" => {
                // Capturer un fruit du démon
                if let Some(joueur) = joueurs.get_mut(0) {
                    capture_fruit_de_demon(&mut objets, joueur);
                }
            },
             "7" => {
                // Afficher les statistiques du joueur
                if let Some(joueur) = joueurs.get(0) {
                    afficher_stats(joueur, &objets);
                }
            }
            "8" => {
                // Test de l'entraînement
                if let Some(joueur) = joueurs.get_mut(0) {
                    entrainement(&mut objets, joueur);
                }
            },
            "9" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}