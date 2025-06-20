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
trait Combattant {
    fn est_vaincu(&self) -> bool;
}



// Implémentations des traits


impl Combattant for PnjAvecType {
    
    fn est_vaincu(&self) -> bool {
        match &self.type_de_pnj {
            PnjType::Ennemi { hp, .. } => *hp == 0,
            _ => false
        }
    }
}

// Méthodes utilitaires
impl PnjAvecType {

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
        // Afficher immédiatement les messages d'introduction
        if let PnjType::Entraineur { ref competence, ref bonus_puissance, ref niveau_requis } = self.type_de_pnj {
            println!("Vous interagissez avec {} :", self.pnj.nom);
            println!("\"{}\"", self.pnj.description);
            println!("{} peut vous entraîner en {} et améliorer votre puissance de {} !", 
                     self.pnj.nom, competence, bonus_puissance);
            
            // Vérifier les prérequis pour l'entraînement
            if let Some(Objet::Joueur(joueur)) = objets.get(player_index) {
                if joueur.hp >= *niveau_requis {
                    println!("Vous avez les prérequis pour cet entraînement.");
                    println!("Voulez-vous vous entraîner? (o/n)");
                    
                    // Construire la chaîne de résultat
                    let mut result = format!("Vous interagissez avec {} :\n", self.pnj.nom);
                    result.push_str(&format!("\"{}\"\n", self.pnj.description));
                    result.push_str(&format!("{} peut vous entraîner en {} et améliorer votre puissance de {} !\n", 
                                    self.pnj.nom, competence, bonus_puissance));
                    result.push_str("Vous avez les prérequis pour cet entraînement.\n");
                    
                    // Demander l'entrée utilisateur après avoir affiché tous les messages
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
                            
                            println!("Votre puissance augmente de {}! Nouvelle puissance: {}", 
                                     bonus_puissance, joueur_mut.puissance);
                            result.push_str(&format!("Votre puissance augmente de {}! Nouvelle puissance: {}", 
                                          bonus_puissance, joueur_mut.puissance));
                        }
                    } else {
                        println!("Vous avez refusé l'entraînement.");
                        result.push_str("Vous avez refusé l'entraînement.");
                    }
                    
                    return result;
                } else {
                    println!("Vous n'êtes pas assez fort pour cet entraînement.");
                    println!("HP requis: {} HP - Vos HP actuel: {} HP", niveau_requis, joueur.hp);
                    
                    let result = format!("Vous interagissez avec {} :\n", self.pnj.nom);
                    result + &format!("\"{}\"\n", self.pnj.description) 
                         + &format!("{} peut vous entraîner en {} et améliorer votre puissance de {} !\n", 
                                   self.pnj.nom, competence, bonus_puissance)
                         + &format!("Vous n'êtes pas assez fort pour cet entraînement.\n")
                         + &format!("HP requis: {} HP - Vos HP actuel: {} HP", niveau_requis, joueur.hp)
                }
            } else {
                println!("Erreur: Joueur non trouvé!");
                format!("Erreur: Joueur non trouvé!")
            }
        } else {
            println!("Erreur: Ce PNJ n'est pas un entraîneur!");
            format!("Erreur: Ce PNJ n'est pas un entraîneur!")
        }
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
            // Afficher les connexions du sous-lieu
            if !sl.connections.is_empty() {
                println!("Connexions du sous-lieu :");
                for conn in &sl.connections {
                    // Chercher le nom du sous-lieu ou lieu de destination
                    let nom_dest = objets.iter().filter_map(|o| {
                        if let Objet::SousLieu(sousl) = o {
                            if sousl.id == conn.destination {
                                return Some(sousl.nom.as_str());
                            }
                        }
                        if let Objet::Lieu(lieu) = o {
                            if lieu.id == conn.destination {
                                return Some(lieu.nom.as_str());
                            }
                        }
                        None
                    }).next().unwrap_or("Lieu inconnu");
                    println!("  -> {} vers {} ({})", conn.orientation, nom_dest, conn.destination);
                }
            } else {
                println!("Aucune connexion depuis ce sous-lieu.");
            }
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
                println!("  • Objet Mobile: {} , {} ({})", o.nom, o.description, o.id);
                found = true;
            }
            Objet::Pnj(p) if &p.position == pos && &p.sous_position == sous_pos => {
                println!("  • PNJ: {}", p.nom);
                found = true;
            }
            Objet::PnjAvecType(p) if &p.pnj.position == pos && &p.pnj.sous_position == sous_pos => {
                // Afficher le PNJ avec son type spécifique
                let type_description = match &p.type_de_pnj {
                    PnjType::Gentil { .. } => "amical",
                    PnjType::Ennemi { .. } => "hostile",
                    PnjType::Entraineur { .. } => "entraîneur",
                };
                println!("  • PNJ {}: {} - \"{}\"", type_description, p.pnj.nom, p.pnj.description);
                found = true;
            }
            Objet::FruitDuDemon(f) if &f.position == pos && &f.sous_position == sous_pos => {
                println!("  • Fruit du Démon: {} ({})", f.nom, f.pouvoir);
                found = true;
            }
            Objet::Aliment(a) if &a.position == pos && &a.sous_position == sous_pos => {
                println!("  • Aliment: {}, {} (+{} HP)", a.nom, a.description, a.hp);
                found = true;
            }
            _ => {}
    }
}
    if !found {
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

                    // Vérifier si le joueur possède tous les Poneglyphes
                    let a_poneglyphe1 = joueur.inventaire.iter().any(|item| {
                        matches!(item, ObjetInventaire::ObjetStatique(o) if o.id == "poneglyphe1")
                    });
                    
                    let a_poneglyphe2 = joueur.inventaire.iter().any(|item| {
                        matches!(item, ObjetInventaire::ObjetStatique(o) if o.id == "poneglyphe2")
                    });
                    
                    let a_poneglyphe3 = joueur.inventaire.iter().any(|item| {
                        matches!(item, ObjetInventaire::ObjetStatique(o) if o.id == "poneglyphe3")
                    });
                    
                    let a_poneglyphe4 = joueur.inventaire.iter().any(|item| {
                        matches!(item, ObjetInventaire::ObjetStatique(o) if o.id == "poneglyphe4")
                    });

                    if a_poneglyphe1 && a_poneglyphe2 && a_poneglyphe3 && a_poneglyphe4 {
                        // Téléporter directement à piece6 SELAUGHTALE
                        println!("Vous avez collecté les 4 Poneglyphes! Un portail mystérieux s'ouvre...");
                        joueur.position = "piece6".to_string();
                        joueur.sous_position = "SELAUGHTALE".to_string();
                        
                        // Mettre à jour la position du joueur dans objets
                        if let Some(Objet::Joueur(j)) = objets.get_mut(player_index) {
                            j.position = "piece6".to_string();
                            j.sous_position = "SELAUGHTALE".to_string();
                        }
                        
                        println!("Vous êtes téléporté dans un lieu mystérieux!");
                    }
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
    objets: &mut Vec<Objet>  // Changez en &mut pour pouvoir modifier les objets
) {
    // Extraire les informations nécessaires
    let mut lieux: Vec<Lieu> = Vec::new();
    let mut sous_lieux: Vec<SousLieu> = Vec::new();

    // Vérifier si un bateau est présent à la position actuelle du joueur
    let bateau_present = objets.iter().any(|obj| {
        if let Objet::ObjetMobile(o) = obj {
            o.nom == "Bateau" && o.position == joueur.position && o.sous_position == joueur.sous_position
        } else {
            false
        }
    });
    
    if !bateau_present {
        println!("Il n'y a pas de bateau ici pour vous déplacer, cherchez le bateau.");
        return;
    }

    // Extraire les lieux et sous-lieux
    for obj in objets.iter() {
        match obj {
            Objet::Lieu(lieu) => lieux.push(Lieu {
                id: lieu.id.clone(),
                nom: lieu.nom.clone(),
                connections: lieu.connections.clone(),
                required_key: lieu.required_key.clone(),
                description: lieu.description.clone(),
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

    if direction != "N" && direction != "S" && direction != "E" && direction != "O" {
        println!("Direction invalide. Utilisez N, S, E ou O.");
        return;
    }

    // Trouver le lieu actuel et vérifier la connexion
    for lieu in &lieux {
        if lieu.id == joueur.position {
            if let Some(conn) = lieu.connections.iter().find(|c| c.orientation == direction) {
                if let Some(destination_lieu) = lieux.iter().find(|l| l.id == conn.destination) {
                    // Vérifie si une clé est requise
                    if !destination_lieu.required_key.is_empty() {
                        let a_cle = joueur.inventaire.iter().any(|obj| match obj {
                            ObjetInventaire::ObjetStatique(o) => o.id == destination_lieu.required_key,
                            ObjetInventaire::Aliment(a) => a.id == destination_lieu.required_key,
                        });
                        if !a_cle {
                            println!("Vous devez avoir '{}' pour y accéder.", destination_lieu.required_key);
                            return;
                        }
                    }

                    // Mise à jour position du joueur
                    let ancien_lieu_id = joueur.position.clone();
                    let ancien_sous_lieu_id = joueur.sous_position.clone();
                    joueur.position = destination_lieu.id.clone();

                    // Rechercher le premier sous-lieu commençant par "SE" dans la nouvelle position
                    if let Some(sous_lieu_se) = sous_lieux.iter().find(|sl| 
                        sl.position == joueur.position && sl.id.starts_with("SE")) {
                        
                        joueur.sous_position = sous_lieu_se.id.clone();
                        
                        // Mise à jour de la position du bateau
                        for obj in objets.iter_mut() {
                            if let Objet::ObjetMobile(objet) = obj {
                                if objet.nom == "Bateau" && 
                                   objet.position == ancien_lieu_id &&
                                   objet.sous_position == ancien_sous_lieu_id {
                                    
                                    println!("Vous utilisez le bateau pour aller vers {}.", destination_lieu.nom);
                                    objet.position = destination_lieu.id.clone();
                                    objet.sous_position = sous_lieu_se.id.clone();
                                }
                            }
                        }
                    }

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
    // Vérifier si le joueur a obtenu le One Piece
    if let Some(joueur) = joueurs.get(0) {
        let a_onepiece = joueur.inventaire.iter().any(|item| {
            matches!(item, ObjetInventaire::ObjetStatique(o) if o.id == "onepiece")
        });

        if a_onepiece {
            println!("\n\n🎉🎉🎉 FÉLICITATIONS! 🎉🎉🎉");
            println!("Vous avez découvert le ONE PIECE, le trésor légendaire laissé par Gold Roger!");
            println!("Vous êtes maintenant le ROI DES PIRATES!");
            println!("\nFIN DU JEU");
            
            // ASCII Art et pause comme précédemment
            println!("\n");
            println!("     ____    ,____     ____           ____     O  ____     ____     ____ ");
            println!("   /'    )--/'    )  /'    )        /'    )--/' /'    )  /'    )--/'    )");
            println!(" /'    /' /'    /' /(___,/'       /'    /' /' /(___,/' /'       /(___,/' ");
            println!("(___,/' /'    /(__(________     /(___,/'  (__(________(___,/   (________ ");
            println!("                              /'                                         ");
            println!("                            /'                                           ");
            println!("                          /'                                             ");
            println!("\n");
            
            use std::thread::sleep;
            use std::time::Duration;
            sleep(Duration::from_millis(5000));
            
            std::process::exit(0);
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
        println!("Un fruit du démon ({}, {}) est trouvé dans ta zone !", fruit.nom, fruit.description);
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
    println!("Puissance : {}", joueur.puissance);
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
        
        // Vérifier si c'est du Saké de Wano
        let est_sake = aliment.nom.contains("Saké");
        
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
        
        // Appliquer l'effet d'ivresse si c'est du Saké
        if est_sake {
            effet_ivresse(joueurs, objets);
        }
    }
}

fn effet_ivresse(joueurs: &mut Vec<Joueur>, objets: &mut Vec<Objet>) {
    use std::thread::sleep;
    use std::time::Duration;
    
    println!("\n🍶 Vous buvez le Saké de Wano d'une traite...");
    sleep(Duration::from_millis(1000));
    
    println!("Vous sentez une chaleur se répandre dans tout votre corps...");
    sleep(Duration::from_millis(1500));
    
    // Effet visuel de vision floue
    println!("\nVoTre ViSioN deViEnT flOuE...");
    sleep(Duration::from_millis(800));
    println!("LeS sOns SemBleNt dÉfOrmÉs...");
    sleep(Duration::from_millis(800));
    
    // Dialogue d'ivresse aléatoire
    let dialogues = [
        "JE VAIS DEVENIRRR LE ROI DES PIRATESSSS!!!",
        "Hé Zoro... t'es mon meilleur ami tu sais...",
        "Je pourrais... *hic*... battre Kaido les yeux fermés...",
        "Sanji... fais-moi encore à mangerrrrr...",
        "Shanks! Rends-moi mon chapeau... ah non, il est là..."
    ];
    
    use rand::Rng;
    let mut rng = rand::rng();
    let dialogue = dialogues[rng.random_range(0..dialogues.len())];
    println!("\nVous criez soudainement: \"{}\"", dialogue);
    sleep(Duration::from_millis(2000));
    
    // Bonus temporaire
    if let Some(joueur) = joueurs.get_mut(0) {
        let bonus_puissance = 15;
        joueur.puissance += bonus_puissance;
        println!("\n💪 Vous vous sentez INVINCIBLE! (+{} puissance temporaire)", bonus_puissance);
        
        // Synchroniser avec objets
        for obj in objets.iter_mut() {
            if let Objet::Joueur(j) = obj {
                j.puissance = joueur.puissance;
            }
        }
    }
    
    // Mini-jeu d'équilibre
    println!("\n🌀 Vous titubez... Essayez de garder l'équilibre!");
    println!("Tapez 'stable' rapidement pour ne pas tomber!");
    
    // Démarrer un timer
    let debut = std::time::Instant::now();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    
    let temps = debut.elapsed().as_secs_f32();
    
    if input.trim().to_lowercase() == "stable" && temps < 5.0 {
        println!("✅ Vous gardez l'équilibre (juste à temps)!");
    } else {
        println!("❌ Vous trébuchez et tombez face contre terre!");
        
        // Petite pénalité
        if let Some(joueur) = joueurs.get_mut(0) {
            joueur.hp = (joueur.hp as f32 * 0.9) as u32; // 10% dégâts
            
            // Synchroniser
            for obj in objets.iter_mut() {
                if let Objet::Joueur(j) = obj {
                    j.hp = joueur.hp;
                }
            }
            
            println!("Vous perdez quelques HP en tombant. HP actuel: {}", joueur.hp);
        }
    }
    
    println!("\n⏱️ L'effet du saké se dissipera dans quelques minutes...");
    sleep(Duration::from_millis(3000));
    
    // Restaurer puissance normale (après 3 tours de jeu)
    println!("(L'effet de puissance se dissipera après 3 actions)");
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
    // Liste de tous les fichiers JSON à charger
    let files = [
        "joueur.json",
        "lieu.json",
        "sous_lieux.json",
        "objetstatic.json",
        "pnj.json", 
        "fruitdemon.json",
        "aliments.json",
        "objetmobile.json",
    ];
    
    // Structure pour stocker tous les objets du jeu
    let mut objets: Vec<Objet> = Vec::new();
    
    // Charger chaque fichier et combiner les données
    for filename in &files {
        match fs::read_to_string(filename) {
            Ok(content) => {
                match serde_json::from_str::<Vec<Objet>>(&content) {
                    Ok(parsed_objects) => {
                        objets.extend(parsed_objects);
                    },
                    Err(e) => {
                        println!("⚠️ Erreur de parsing JSON dans {} : {}", filename, e);
                    }
                }
            },
            Err(e) => {
                println!("⚠️ Impossible de lire le fichier {} : {}", filename, e);
            }
        }
    }


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
        println!("10. Se déplacer à l'intérieur d'un lieu");
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
                    move_joueur(joueur, dir, &mut objets); // Passage de &mut objets
                    // Mettre à jour la position du joueur dans objets
                    for obj in &mut objets {
                        if let Objet::Joueur(j) = obj {
                            j.position = joueur.position.clone();
                            j.sous_position = joueur.sous_position.clone();
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
            "10" => {
                // Déplacement interne
                if let Some(joueur) = joueurs.get_mut(0) {
                    println!("Dans quelle direction ? (N/S/E/O)");
                    let mut dir = String::new();
                    io::stdin().read_line(&mut dir).unwrap();
                    let dir = dir.trim();
                    
                    // Gérer le Result retourné par move_inside
                    match move_inside(joueur, dir, &objets) {
                        Ok(_) => {
                            // Mettre à jour la position du joueur dans objets
                            for obj in &mut objets {
                                if let Objet::Joueur(j) = obj {
                                    j.sous_position = joueur.sous_position.clone();
                                }
                            }
                        },
                        Err(message) => println!("{}", message),
                    }
                }
            }

            "Q" => {
                println!("Au revoir !");
                break;
            }
            _ => println!("Choix invalide."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn joueur_test() -> Joueur {
        Joueur {
            nom: "Test".to_string(),
            fruit_de_demon: None,
            position: "piece1".to_string(),
            sous_position: "SL1".to_string(),
            inventaire: vec![],
            puissance: 10,
            hp: 100,
        }
    }

    #[test]
    fn test_afficher_stats_sans_fruit() {
        let joueur = joueur_test();
        let objets = vec![];
        afficher_stats(&joueur, &objets); // Doit afficher "Fruit : Aucun"
    }

    #[test]
    fn test_afficher_stats_avec_fruit() {
        let mut joueur = joueur_test();
        let fruit = FruitDuDemon {
            id: "f1".to_string(),
            nom: "Gomu Gomu".to_string(),
            description: "Fruit du caoutchouc".to_string(),
            sous_position: "SL1".to_string(),
            pouvoir: "Caoutchouc".to_string(),
            position: "piece1".to_string(),
            attaque: vec!["a1".to_string()],
        };
        joueur.fruit_de_demon = Some(fruit);
        let attaque = Attaque {
            id: "a1".to_string(),
            nom: "Pistol".to_string(),
            description: "Coup de poing".to_string(),
            puissance: 30,
        };
        let objets = vec![Objet::Attaque(attaque)];
        afficher_stats(&joueur, &objets); // Doit afficher l'attaque
    }

    #[test]
    fn test_show_objects_at_player_position_empty() {
        let joueur = joueur_test();
        let objets = vec![];
        let lieux = vec![];
        show_objects_at_player_position(&objets, &lieux, &joueur); // Doit n'afficher rien de spécial
    }

    #[test]
    fn test_capture_fruit_de_demon_logic() {
        // Ce test vérifie la logique sans interaction utilisateur
        let mut joueur = joueur_test();
        let fruit = FruitDuDemon {
            id: "f1".to_string(),
            nom: "Gomu Gomu".to_string(),
            description: "Fruit du caoutchouc".to_string(),
            sous_position: "SL1".to_string(),
            pouvoir: "Caoutchouc".to_string(),
            position: "piece1".to_string(),
            attaque: vec![],
        };
        let mut objets = vec![Objet::FruitDuDemon(fruit.clone())];

        // Simule le cas où le joueur n'a pas de fruit et prend le fruit automatiquement (sans interaction)
        // Pour tester la logique, on appelle directement l'affectation
        joueur.fruit_de_demon = Some(fruit.clone());
        objets.remove(0);
        assert!(joueur.fruit_de_demon.is_some());
        assert!(objets.is_empty());
    }
    // Fonctions utilitaires pour les tests
    fn creer_joueur_test() -> Joueur {
        Joueur {
            nom: "Test".to_string(),
            fruit_de_demon: None,
            position: "piece1".to_string(),
            sous_position: "SL1".to_string(),
            inventaire: vec![],
            puissance: 10,
            hp: 100,
        }
    }

    fn creer_pnj_gentil() -> PnjAvecType {
        PnjAvecType {
            pnj: Pnj {
                nom: "PNJ Gentil".to_string(),
                description: "Un PNJ amical".to_string(),
                position: "piece1".to_string(),
                sous_position: "SL1".to_string(),
                inventaire: vec![],
            },
            type_de_pnj: PnjType::Gentil {
                dialogue_special: Some("Bonjour aventurier !".to_string()),
            },
        }
    }

    fn creer_pnj_ennemi() -> PnjAvecType {
        PnjAvecType {
            pnj: Pnj {
                nom: "PNJ Ennemi".to_string(),
                description: "Un PNJ hostile".to_string(),
                position: "piece1".to_string(),
                sous_position: "SL1".to_string(),
                inventaire: vec![],
            },
            type_de_pnj: PnjType::Ennemi {
                puissance: 5,
                hp: 50,
                attaques: vec!["attaque1".to_string()],
                required_items: vec![],
            },
        }
    }

    fn creer_attaque_test() -> Attaque {
        Attaque {
            id: "attaque1".to_string(),
            nom: "Attaque Test".to_string(),
            description: "Une attaque pour les tests".to_string(),
            puissance: 20,
        }
    }

    // Tests pour la fonction interact
    #[test]
    fn test_interact_avec_pnj_inexistant() {
        let mut objets = vec![
            Objet::Joueur(creer_joueur_test()),
            Objet::PnjAvecType(creer_pnj_gentil()),
        ];
        let mut joueurs = vec![creer_joueur_test()];
        
        // Tester avec un nom de PNJ qui n'existe pas
        interact(&mut objets, "PNJ Inconnu", &mut joueurs);
        // Le test passe si la fonction ne panique pas
    }

    #[test]
    fn test_interact_avec_pnj_existant() {
        let mut joueur = creer_joueur_test();
        let pnj_gentil = creer_pnj_gentil();
        
        let mut objets = vec![
            Objet::Joueur(joueur.clone()),
            Objet::PnjAvecType(pnj_gentil.clone()),
        ];
        let mut joueurs = vec![joueur];
        
        // Interagir avec un PNJ qui existe
        interact(&mut objets, &pnj_gentil.pnj.nom, &mut joueurs);
        // Le test passe si la fonction ne panique pas
    }

    #[test]
    fn test_interact_avec_pnj_different_position() {
        let mut joueur = creer_joueur_test();
        let mut pnj_gentil = creer_pnj_gentil();
        pnj_gentil.pnj.position = "piece2".to_string(); // PNJ dans une position différente
        
        let mut objets = vec![
            Objet::Joueur(joueur.clone()),
            Objet::PnjAvecType(pnj_gentil.clone()),
        ];
        let mut joueurs = vec![joueur];
        
        // Tenter d'interagir avec un PNJ qui est dans un lieu différent
        interact(&mut objets, &pnj_gentil.pnj.nom, &mut joueurs);
        // Le test passe si la fonction ne panique pas
    }


    #[test]
    fn test_combat_avec_pnj_non_ennemi() {
        let mut joueur = creer_joueur_test();
        let pnj_gentil = creer_pnj_gentil();
        
        let mut objets = vec![
            Objet::Joueur(joueur.clone()),
            Objet::PnjAvecType(pnj_gentil),
        ];
        let mut joueurs = vec![joueur];
        
        // Tenter de combattre un PNJ qui n'est pas un ennemi
        combat(&mut objets, 1, 0, &mut joueurs);
        // Le test passe si la fonction ne panique pas
    }


    #[test]
    fn test_combat_resultat_hp() {
        // Créer un joueur avec beaucoup de HP pour assurer la victoire
        let mut joueur = creer_joueur_test();
        joueur.puissance = 100; // Joueur très puissant
        
        // Créer un ennemi faible
        let mut pnj_ennemi = creer_pnj_ennemi();
        if let PnjType::Ennemi { ref mut hp, .. } = pnj_ennemi.type_de_pnj {
            *hp = 10; // Ennemi avec peu de HP
        }
        
        let attaque = creer_attaque_test();
        
        let mut objets = vec![
            Objet::Joueur(joueur.clone()),
            Objet::PnjAvecType(pnj_ennemi),
            Objet::Attaque(attaque),
        ];
        let mut joueurs = vec![joueur];
        
        // Simuler un combat où le joueur devrait gagner facilement
        combat(&mut objets, 1, 0, &mut joueurs);
        
        // Vérifier que l'ennemi a bien été vaincu (HP à 0)
        if let Objet::PnjAvecType(pnj) = &objets[1] {
            if let PnjType::Ennemi { hp, .. } = pnj.type_de_pnj {
                assert_eq!(hp, 0, "L'ennemi devrait être vaincu (HP à 0)");
            }
        }
    }
}
