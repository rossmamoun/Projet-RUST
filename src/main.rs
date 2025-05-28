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
    inventaire: Vec<ObjetInventaire>,
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
    inventaire: Vec<String>,
}

// Enum pour les différents types de PNJ avec leurs attributs spécifiques
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
enum PnjType {
    #[serde(rename = "Ennemi")]
    Ennemi {
        puissance: u32,
        hp: u32,
        attaques: Vec<String>,
        required_items: Vec<String>,
    },
    #[serde(rename = "Gentil")]
    Gentil {
        dialogue_special: Option<String>,
    },
    #[serde(rename = "Entraineur")]
    Entraineur {
        competence: String,
        bonus_puissance: u32,
        niveau_requis: u32,
    },
}

// Structure combinée
#[derive(Debug, Clone, Deserialize)]
struct PnjAvecType {
    pnj: Pnj,
    type_de_pnj: PnjType,
}

// Traits pour les comportements spécifiques
trait Interactif {
    fn interagir(&self, joueur: &mut Joueur) -> String;
}

trait Combattant {
    fn attaquer(&self, joueur: &mut Joueur) -> (u32, String);
    fn est_vaincu(&self) -> bool;
}

trait Formateur {
    fn entrainer(&self, joueur: &mut Joueur) -> bool;
    fn competence(&self) -> &str;
}

// Implémentations des traits
impl Interactif for PnjAvecType {
    fn interagir(&self, joueur: &mut Joueur) -> String {
        match &self.type_de_pnj {
            PnjType::Ennemi { .. } => {
                format!("{} vous regarde avec hostilité!", self.pnj.nom)
            },
            PnjType::Gentil { dialogue_special } => {
                if let Some(dialogue) = dialogue_special {
                    format!("{} dit: \"{}\"", self.pnj.nom, dialogue)
                } else {
                    format!("{} vous salue amicalement.", self.pnj.nom)
                }
            },
            PnjType::Entraineur { competence, .. } => {
                format!("{} propose de vous entraîner en {}.", self.pnj.nom, competence)
            }
        }
    }
}

impl Combattant for PnjAvecType {
    fn attaquer(&self, joueur: &mut Joueur) -> (u32, String) {
        match &self.type_de_pnj {
            PnjType::Ennemi { puissance, hp, attaques, .. } => {
                if *hp == 0 {
                    return (0, format!("{} est vaincu et ne peut pas attaquer.", self.pnj.nom));
                }
                
                // Attaque avec la première attaque ou attaque de base
                let degats = *puissance;
                let message = if !attaques.is_empty() {
                    format!("{} utilise {} et inflige {} dégâts!", 
                            self.pnj.nom, attaques[0], degats)
                } else {
                    format!("{} attaque et inflige {} dégâts!", self.pnj.nom, degats)
                };
                
                (degats, message)
            },
            _ => (0, format!("{} ne peut pas attaquer!", self.pnj.nom))
        }
    }
    
    fn est_vaincu(&self) -> bool {
        match &self.type_de_pnj {
            PnjType::Ennemi { hp, .. } => *hp == 0,
            _ => false
        }
    }
}

impl Formateur for PnjAvecType {
    fn entrainer(&self, joueur: &mut Joueur) -> bool {
        match &self.type_de_pnj {
            PnjType::Entraineur { bonus_puissance, niveau_requis, .. } => {
                if joueur.hp >= *niveau_requis {
                    joueur.puissance += *bonus_puissance;
                    true
                } else {
                    false
                }
            },
            _ => false
        }
    }
    
    fn competence(&self) -> &str {
        match &self.type_de_pnj {
            PnjType::Entraineur { competence, .. } => competence,
            _ => ""
        }
    }
}

// Méthodes utilitaires
impl PnjAvecType {
    fn est_ennemi(&self) -> bool {
        matches!(self.type_de_pnj, PnjType::Ennemi { .. })
    }
    
    fn est_entraineur(&self) -> bool {
        matches!(self.type_de_pnj, PnjType::Entraineur { .. })
    }
    
    fn est_gentil(&self) -> bool {
        matches!(self.type_de_pnj, PnjType::Gentil { .. })
    }
    
    fn set_hp(&mut self, new_hp: u32) {
        if let PnjType::Ennemi { ref mut hp, .. } = self.type_de_pnj {
            *hp = new_hp;
        }
    }

    // Méthode principale d'interaction qui va router vers la fonction spécifique
    fn interact_with_player(&mut self, objets: &mut Vec<Objet>, player_index: usize, joueurs: &mut Vec<Joueur>) -> String {
        match &self.type_de_pnj {
            PnjType::Ennemi { .. } => self.interact_as_ennemi(objets, player_index, joueurs),
            PnjType::Gentil { .. } => self.interact_as_gentil(objets, player_index, joueurs),
            PnjType::Entraineur { .. } => self.interact_as_entraineur(objets, player_index, joueurs),
        }
    }

    // Interaction spécifique pour les PNJ ennemis
    fn interact_as_ennemi(&mut self, objets: &mut Vec<Objet>, player_index: usize, joueurs: &mut Vec<Joueur>) -> String {
        if self.est_vaincu() {
            return format!("{} est déjà vaincu.", self.pnj.nom);
        }

        let mut result = format!("🔥 COMBAT! Vous affrontez {} !", self.pnj.nom);
        
        // Vérifier si le joueur a les objets requis
        if let Some(Objet::Joueur(joueur)) = objets.get(player_index) {
            if let PnjType::Ennemi { ref required_items, .. } = self.type_de_pnj {
                let mut has_all_items = true;
                
                for item_id in required_items {
                    if !joueur.inventaire.iter().any(|i| match i {
                        ObjetInventaire::ObjetStatique(o) => &o.id == item_id,
                        ObjetInventaire::Aliment(a) => &a.id == item_id,
                    }) {
                        has_all_items = false;
                        break;
                    }
                }
                
                if has_all_items {
                    // Trouver l'index du PNJ actuel dans la liste des objets
                    let mut pnj_index = None;
                    for (i, obj) in objets.iter().enumerate() {
                        if let Objet::PnjAvecType(pnj) = obj {
                            if pnj.pnj.nom == self.pnj.nom && pnj.pnj.position == self.pnj.position {
                                pnj_index = Some(i);
                                break;
                            }
                        }
                    }
                    
                    if let Some(i) = pnj_index {
                        // Appel à la fonction combat avec les bons indices
                        combat(objets, i, player_index, joueurs);
                        
                        // Vérifier si le combat a été gagné
                        if let Some(Objet::PnjAvecType(pnj)) = objets.get(i) {
                            if pnj.est_vaincu() {
                                // Mettre à jour notre PNJ local pour refléter la victoire
                                if let PnjType::Ennemi { ref mut hp, .. } = self.type_de_pnj {
                                    *hp = 0;
                            }
                            result.push_str("\nVous avez remporté le combat!");
                        } else {
                            result.push_str("\nVous n'avez pas vaincu l'ennemi.");
                        }
                    }
                } else {
                    result.push_str("\nErreur: PNJ introuvable dans la liste des objets.");
                }
            } else {
                result.push_str("\nDéfaite! Vous n'avez pas l'équipement nécessaire.");
                
                // Le joueur perd des points de vie
                if let Some(Objet::Joueur(joueur_mut)) = objets.get_mut(player_index) {
                    if joueur_mut.hp >= 10 {
                        joueur_mut.hp -= 10;
                    } else {
                        joueur_mut.hp = 0;
                    }
                    
                    // Synchronisation immédiate
                    if let Some(joueur) = joueurs.get_mut(0) {
                        joueur.hp = joueur_mut.hp;
                    }
                    
                    result.push_str(&format!("\nVous perdez 10 points de vie. HP restants: {}", joueur_mut.hp));
                }
            }
        }
    }
    
    result
}

    // Interaction spécifique pour les PNJ gentils
    fn interact_as_gentil(&mut self, objets: &mut Vec<Objet>, player_index: usize, joueurs: &mut Vec<Joueur>) -> String {
        // Immédiatement afficher les messages de base et le dialogue spécial s'il existe
        println!("Vous interagissez avec {} :", self.pnj.nom);
        println!("\"{}\"", self.pnj.description);
        
        // Ajouter le dialogue spécial s'il existe
        if let PnjType::Gentil { ref dialogue_special } = self.type_de_pnj {
            if let Some(dialogue) = dialogue_special {
                println!("Message spécial: \"{}\"", dialogue);
            }
        }

        // Construire aussi la chaîne de résultat pour le retour de fonction
        let mut result = format!("Vous interagissez avec {} :\n", self.pnj.nom);
        result.push_str(&format!("\"{}\"\n", self.pnj.description));
        
        // Ajouter le dialogue spécial à la chaîne de résultat
        if let PnjType::Gentil { ref dialogue_special } = self.type_de_pnj {
            if let Some(dialogue) = dialogue_special {
                result.push_str(&format!("Message spécial: \"{}\"\n", dialogue));
            }
        }
        
        // Gérer les objets à offrir
        if !self.pnj.inventaire.is_empty() {
            let objet_id = &self.pnj.inventaire[0];
            
            // Trouver l'objet correspondant
            let mut objet_trouve = None;
            for obj in objets.iter() {
                if let Objet::ObjetStatique(o) = obj {
                    if &o.id == objet_id {
                        objet_trouve = Some(o.clone());
                        break;
                    }
                }
            }
            
            if let Some(objet) = objet_trouve {
                println!("\n{} vous propose un objet : {}", self.pnj.nom, objet.nom);
                println!("Description : {}", objet.description);
                println!("\nVoulez-vous le prendre? (o/n)");
                
                // Maintenant demander l'entrée utilisateur après avoir affiché tous les messages
                let mut reponse = String::new();
                io::stdin().read_line(&mut reponse).expect("Erreur de lecture");
                let reponse = reponse.trim().to_lowercase();
                
                // Ajouter à la chaîne de résultat
                result.push_str(&format!("\n{} vous propose un objet : {}\n", self.pnj.nom, objet.nom));
                result.push_str(&format!("Description : {}\n", objet.description));
                
                if reponse == "o" || reponse == "oui" {
                    // Supprimer l'objet de l'inventaire du PNJ
                    self.pnj.inventaire.remove(0);
                    
                    // Ajouter l'objet à l'inventaire du joueur
                    if let Some(Objet::Joueur(joueur)) = objets.get_mut(player_index) {
                        let mut objet_final = objet.clone();
                        objet_final.position = "inventaire".to_string();
                        
                        joueur.inventaire.push(ObjetInventaire::ObjetStatique(objet_final));
                        
                        // Synchroniser avec le vecteur joueurs
                        if let Some(joueur_vec) = joueurs.get_mut(0) {
                            joueur_vec.inventaire = joueur.inventaire.clone();
                        }
                        
                        println!("→ Objet '{}' ajouté à votre inventaire !", objet.nom);
                        result.push_str(&format!("\n→ Objet '{}' ajouté à votre inventaire !", objet.nom));
                    }
                } else {
                    println!("Vous avez refusé l'objet.");
                    result.push_str("\nVous avez refusé l'objet.");
                }
            } else {
                println!("{} a un objet, mais impossible de le trouver dans le monde.", self.pnj.nom);
                result.push_str(&format!("\n{} a un objet, mais impossible de le trouver dans le monde.", self.pnj.nom));
            }
        } else {
            println!("{} n'a rien à vous offrir.", self.pnj.nom);
            result.push_str(&format!("\n{} n'a rien à vous offrir.", self.pnj.nom));
        }
        
        result
    }

    // Interaction spécifique pour les PNJ entraîneurs
    fn interact_as_entraineur(&mut self, objets: &mut Vec<Objet>, player_index: usize, joueurs: &mut Vec<Joueur>) -> String {
        let mut result = String::new();
        
        if let PnjType::Entraineur { ref competence, ref bonus_puissance, ref niveau_requis } = self.type_de_pnj {
            result.push_str(&format!("{} peut vous entraîner en {} et améliorer votre puissance de {}!\n", 
                            self.pnj.nom, competence, bonus_puissance));
            
            if let Some(Objet::Joueur(joueur)) = objets.get(player_index) {
                if joueur.hp >= *niveau_requis {
                    result.push_str("Voulez-vous vous entraîner? (o/n)");
                    
                    let mut reponse = String::new();
                    io::stdin().read_line(&mut reponse).expect("Erreur de lecture");
                    let reponse = reponse.trim().to_lowercase();
                    
                    if reponse == "o" || reponse == "oui" {
                        // Augmenter la puissance du joueur
                        if let Some(Objet::Joueur(joueur_mut)) = objets.get_mut(player_index) {
                            joueur_mut.puissance += *bonus_puissance;
                            
                            // Synchroniser avec le vecteur joueurs
                            if let Some(joueur_vec) = joueurs.get_mut(0) {
                                joueur_vec.puissance = joueur_mut.puissance;
                            }
                            
                            result.push_str(&format!("\nVotre puissance augmente de {}! Nouvelle puissance: {}", 
                                            bonus_puissance, joueur_mut.puissance));
                        }
                    } else {
                        result.push_str("\nVous avez refusé l'entraînement.");
                    }
                } else {
                    result.push_str(&format!("\nVous n'êtes pas assez fort pour cet entraînement. Niveau requis: {} HP", niveau_requis));
                }
            }
        }
        
        result
    }
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

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
enum Objet {
    #[serde(rename = "ObjetMobile")]
    ObjetMobile(ObjetMobile),
    
    #[serde(rename = "ObjetStatique")]
    ObjetStatique(ObjetStatique),
    
    #[serde(rename = "Pnj")]
    Pnj(Pnj),

    #[serde(rename = "PnjAvecType")]
    PnjAvecType(PnjAvecType),
    
    #[serde(rename = "Joueur")]
    Joueur(Joueur),

    #[serde(rename = "FruitDuDemon")]
    FruitDuDemon(FruitDuDemon),
    
    #[serde(rename = "Aliment")]
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
            Objet::ObjetStatique(os) if os.position == *player_position => {
                println!("  • Objet Statique: {} ({})", os.nom, os.id);
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

fn combat(objets: &mut Vec<Objet>, pnj_index: usize, player_index: usize, joueurs: &mut Vec<Joueur>) {
    // Get a clone of the PNJ with type
    let pnj_avec_type = if let Some(Objet::PnjAvecType(p)) = objets.get(pnj_index) {
        p.clone()
    } else {
        println!("PNJ introuvable!");
        return;
    };
    
    // Use player information directly from joueurs collection
    let joueur = if let Some(j) = joueurs.get(0) {
        j.clone()
    } else {
        println!("Joueur introuvable dans joueurs!");
        return;
    };
    
    // Extraire les attributs de l'ennemi depuis PnjType
    let (mut pnj_hp, pnj_puissance, pnj_attaques) = match &pnj_avec_type.type_de_pnj {
        PnjType::Ennemi { hp, puissance, attaques, .. } => (*hp, *puissance, attaques.clone()),
        _ => {
            println!("Ce PNJ n'est pas un ennemi!");
            return;
        }
    };
    
    println!("⚔️ COMBAT: {} VS {} ⚔️", joueur.nom, pnj_avec_type.pnj.nom);
    println!("{} - HP: {} | Puissance: {}", joueur.nom, joueur.hp, joueur.puissance);
    println!("{} - HP: {} | Puissance: {}", pnj_avec_type.pnj.nom, pnj_hp, pnj_puissance);
    
    // Récupérer les attaques du joueur via son fruit du démon
    let mut attaques_joueur: Vec<Attaque> = Vec::new();
    if let Some(fruit) = &joueur.fruit_de_demon {
        for attaque_id in &fruit.attaque {
            if let Some(Objet::Attaque(attaque)) = objets.iter().find(|obj| {
                matches!(obj, Objet::Attaque(a) if &a.id == attaque_id)
            }) {
                attaques_joueur.push(attaque.clone());
            }
        }
    }
    
    // Récupérer les attaques du PNJ
    let mut attaques_pnj: Vec<Attaque> = Vec::new();
    for attaque_id in &pnj_attaques {
        if let Some(Objet::Attaque(attaque)) = objets.iter().find(|obj| {
            matches!(obj, Objet::Attaque(a) if &a.id == attaque_id)
        }) {
            attaques_pnj.push(attaque.clone());
        }
    }
    
    let mut joueur_hp = joueur.hp;
    
    // Boucle de combat
    while pnj_hp > 0 && joueur_hp > 0 {
        println!("\n--- Tour de combat ---");
        println!("{} - HP: {}", joueur.nom, joueur_hp);
        println!("{} - HP: {}", pnj_avec_type.pnj.nom, pnj_hp);
        
        // Afficher les attaques du joueur
        if let Some(fruit) = &joueur.fruit_de_demon {
            println!("\nAttaques disponibles (Fruit: {}):", fruit.nom);
            
            if attaques_joueur.is_empty() {
                println!("Aucune attaque disponible avec ce fruit.");
                println!("1. Attaque normale - Puissance: {}", joueur.puissance);
            } else {
                for (i, attaque) in attaques_joueur.iter().enumerate() {
                    println!("{}. {} - Puissance: {} - {}", 
                            i + 1, attaque.nom, attaque.puissance, attaque.description);
                }
            }
        } else {
            println!("\nAttaque basique disponible:");
            println!("1. Attaque normale - Puissance: {}", joueur.puissance);
        }
        
        // Demander au joueur de choisir une attaque
        println!("\nChoisissez votre attaque (numéro):");
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        
        let choix_index: usize = match choix.trim().parse::<usize>() {
            Ok(num) if num > 0 && num <= attaques_joueur.len() => num - 1,
            _ => {
                println!("Choix invalide! Attaque 1 utilisée.");
                0 // Par défaut, utiliser la première attaque ou l'attaque normale
            }
        };
        
        // Calcul des dégâts
        let degats_joueur = if !attaques_joueur.is_empty() {
            joueur.puissance + attaques_joueur[choix_index].puissance
        } else {
            joueur.puissance
        };
        
        // Le joueur attaque le PNJ
        pnj_hp = if pnj_hp > degats_joueur { pnj_hp - degats_joueur } else { 0 };
        
        println!("\n{} utilise {} et inflige {} points de dégâts!", 
                joueur.nom, 
                if !attaques_joueur.is_empty() { &attaques_joueur[choix_index].nom } else { "attaque normale" }, 
                degats_joueur);
        
        // Vérifier si le PNJ est vaincu
        if pnj_hp == 0 {
            println!("\n🎉 Victoire! {} a été vaincu!", pnj_avec_type.pnj.nom);
            break;
        }
        
        // Le PNJ contre-attaque
        let degats_pnj;
        let nom_attaque: &String;
        let attaque_normale = String::from("attaque normale");

        if !attaques_pnj.is_empty() {
            // Utiliser la première attaque du PNJ
            let attaque_choisie = &attaques_pnj[0];  // On prend toujours la première attaque
            
            // Calcul des dégâts = puissance de base du PNJ + puissance de l'attaque
            degats_pnj = pnj_puissance + attaque_choisie.puissance;
            nom_attaque = &attaque_choisie.nom;
            
            println!("{} utilise {} et inflige {} points de dégâts!", 
                    pnj_avec_type.pnj.nom, nom_attaque, degats_pnj);
        } else {
            // Si le PNJ n'a pas d'attaques, il utilise une attaque normale
            degats_pnj = pnj_puissance;
            nom_attaque = &attaque_normale;
            
            println!("{} utilise une attaque normale et inflige {} points de dégâts!", 
                    pnj_avec_type.pnj.nom, degats_pnj);
        }
        
        joueur_hp = if joueur_hp > degats_pnj { joueur_hp - degats_pnj } else { 0 };
        
        // Mettre à jour les HP du joueur dans joueurs immédiatement
        if let Some(j) = joueurs.get_mut(0) {
            j.hp = joueur_hp;
        }
        
        // Mettre à jour les HP du joueur dans objets immédiatement
        if let Some(Objet::Joueur(j)) = objets.get_mut(player_index) {
            j.hp = joueur_hp;
        }
        
        // Vérifier si le joueur est vaincu
        if joueur_hp == 0 {
            println!("\n💀 Défaite! Vous avez été vaincu par {}!", pnj_avec_type.pnj.nom);
            break;
        }
        
        // Attendre que le joueur appuie sur Entrée pour continuer
        println!("\nAppuyez sur Entrée pour continuer...");
        let mut attente = String::new();
        io::stdin().read_line(&mut attente).expect("Erreur de lecture");
    }
    
    // Check if player won the combat
    if pnj_hp == 0 {
        println!("Vous avez vaincu {}! Vous récupérez ses objets.", pnj_avec_type.pnj.nom);
        
        // Get PNJ's inventory items
        let mut objets_a_transferer = Vec::new();
        
        // First, find all object IDs in the PNJ's inventory and corresponding objects
        for objet_id in &pnj_avec_type.pnj.inventaire {
            for obj in objets.iter() {
                if let Objet::ObjetStatique(o) = obj {
                    if &o.id == objet_id {
                        let mut objet_clone = o.clone();
                        objet_clone.position = "inventaire".to_string();
                        objets_a_transferer.push(objet_clone);
                        println!("→ Objet '{}' récupéré!", o.nom);
                        break;
                    }
                }
            }
        }
        
        // Now clear PNJ's inventory and transfer objects to player
        if let Some(Objet::PnjAvecType(pnj_mut)) = objets.get_mut(pnj_index) {
            pnj_mut.pnj.inventaire.clear(); // Remove all items from PNJ
        }
        
        // Add the objects to player's inventory
        if let Some(Objet::Joueur(joueur)) = objets.get_mut(player_index) {
            joueur.inventaire.extend(objets_a_transferer.into_iter().map(ObjetInventaire::ObjetStatique));
        }
        
        // Synchronize with joueurs vector
        if let Some(joueur_obj) = objets.get(player_index) {
            if let Objet::Joueur(j) = joueur_obj {
                if let Some(joueur) = joueurs.get_mut(0) {
                    joueur.inventaire = j.inventaire.clone();
                }
            }
        }
    }

    // Mettre à jour les HP dans l'objet PNJ original
    if let Some(Objet::PnjAvecType(pnj_mut)) = objets.get_mut(pnj_index) {
        if let PnjType::Ennemi { ref mut hp, .. } = pnj_mut.type_de_pnj {
            *hp = pnj_hp;
        }
    }
    
    // Mettre à jour les HP du joueur dans les objets
    if let Some(Objet::Joueur(joueur_mut)) = objets.get_mut(player_index) {
        joueur_mut.hp = joueur_hp;
    }
    
    // S'assurer que les joueurs sont synchronisés une dernière fois
    if let Some(j) = joueurs.get_mut(0) {
        j.hp = joueur_hp;
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
        if let Objet::PnjAvecType(p) = obj {
            if p.pnj.nom.to_lowercase() == pnj_name.to_lowercase() && p.pnj.position == player_position {
                // Cloner le PNJ pour interaction
                let mut pnj_clone = p.clone();
                
                // Utiliser la méthode d'interaction spécifique au type
                let result = pnj_clone.interact_with_player(objets, player_index, joueurs);
                println!("{}", result);
                
                // Mettre à jour le PNJ dans la liste des objets
                if let Some(Objet::PnjAvecType(pnj_mut)) = objets.get_mut(i) {
                    *pnj_mut = pnj_clone;
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
    let mut objets_disponibles = Vec::new();

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

    // Collecter tous les objets dans le sous-lieu du joueur
    for obj in objets.iter() {
        match obj {
            Objet::ObjetStatique(o) if o.sous_position == player_sous_position => {
                objets_disponibles.push((
                    "ObjetStatique",
                    o.id.clone(),
                    format!("Objet: {}", o.nom)
                ));
            },
            Objet::Aliment(a) if a.sous_position == player_sous_position => {
                objets_disponibles.push((
                    "Aliment",
                    a.id.clone(),
                    format!("Aliment: {} (+{} HP)", a.nom, a.hp)
                ));
            },
            _ => {}
        }
    }

    if objets_disponibles.is_empty() {
        println!("Aucun objet à ramasser dans ce sous-lieu.");
        return;
    }

    // Afficher les options
    println!("Objets disponibles:");
    println!("0. Tout ramasser");
    for (i, (_, _, desc)) in objets_disponibles.iter().enumerate() {
        println!("{}. {}", i + 1, desc);
    }
    
    println!("Que voulez-vous ramasser? (0-{})", objets_disponibles.len());
    let mut choix = String::new();
    io::stdin().read_line(&mut choix).expect("Erreur de lecture");
    let choix: usize = match choix.trim().parse() {
        Ok(num) if num <= objets_disponibles.len() => num,
        _ => {
            println!("Choix invalide. Rien n'a été ramassé.");
            return;
        }
    };

    let mut ids_a_capturer = Vec::new();
    if choix == 0 {
        // Ramasser tous les objets
        for (_, id, _) in &objets_disponibles {
            ids_a_capturer.push(id.clone());
        }
    } else {
        // Ramasser un seul objet
        ids_a_capturer.push(objets_disponibles[choix - 1].1.clone());
    }

    // Collecter les objets à capturer et les convertir en ObjetInventaire
    let mut objets_a_ajouter = vec![];
    
    // On va maintenant retenir les objets qui ne sont pas capturés
    objets.retain(|obj| {
        match obj {
            Objet::ObjetStatique(o) if ids_a_capturer.contains(&o.id) => {
                println!("→ Objet '{}' capturé dans le sous-lieu !", o.nom);
                objets_a_ajouter.push(ObjetInventaire::ObjetStatique(o.clone()));
                false // Retirer cet objet
            },
            Objet::Aliment(a) if ids_a_capturer.contains(&a.id) => {
                println!("→ Aliment '{}' (+{} HP) capturé dans le sous-lieu !", a.nom, a.hp);
                objets_a_ajouter.push(ObjetInventaire::Aliment(a.clone()));
                false // Retirer cet objet
            },
            _ => true, // Garder cet objet
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

fn afficher_stats(joueur: &Joueur, objets: &[Objet]) {
    println!("--- Statistiques du joueur ---");
    println!("Nom         : {}", joueur.nom);
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

fn consommer_aliment(joueurs: &mut Vec<Joueur>, objets: &mut Vec<Objet>) {
    if let Some(joueur) = joueurs.get_mut(0) {
        if joueur.hp >= 100 {
            println!("🛑 Vous avez déjà tous vos HP (100). Impossible de consommer un aliment !");
            return;
        }
        
        // Collecter tous les aliments dans l'inventaire
        let mut aliments = Vec::new();
        for (i, item) in joueur.inventaire.iter().enumerate() {
            if let ObjetInventaire::Aliment(a) = item {
                aliments.push((i, a));
            }
        }
        
        if aliments.is_empty() {
            println!("Vous n'avez pas d'aliment à consommer !");
            return;
        }
        
        // Afficher les options
        println!("Aliments disponibles:");
        for (i, (_, a)) in aliments.iter().enumerate() {
            println!("{}. {} (+{} HP)", i + 1, a.nom, a.hp);
        }
        
        println!("Que voulez-vous consommer? (1-{})", aliments.len());
        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Erreur de lecture");
        let choix: usize = match choix.trim().parse() {
            Ok(num) if num >= 1 && num <= aliments.len() => num,
            _ => {
                println!("Choix invalide. Rien n'a été consommé.");
                return;
            }
        };
        
        // Consommer l'aliment choisi
        let (index, aliment) = &aliments[choix - 1];
        let hp_avant = joueur.hp;
        joueur.hp = (joueur.hp + aliment.hp).min(100);
        let hp_gagne = joueur.hp - hp_avant;
        
        println!("🍽️ Vous consommez : {}", aliment.nom);
        println!("❤️  Vous regagnez {} HP ! HP actuel : {}", hp_gagne, joueur.hp);
        joueur.inventaire.remove(*index);
        
        // Synchronisation avec la liste globale d'objets
        for obj in objets.iter_mut() {
            if let Objet::Joueur(j) = obj {
                j.inventaire = joueur.inventaire.clone();
                j.hp = joueur.hp;
            }
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
        println!("8. Mini-jeux amusants");
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
                } else {
                    println!("Aucun joueur trouvé !");
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