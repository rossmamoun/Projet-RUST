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





fn main() {
    let data = fs::read_to_string("data.json").expect("Impossible de lire le fichier");
    let mut objets: Vec<Objet> = serde_json::from_str(&data).expect("Erreur de parsing JSON");


    

    // Séparer les objets de type Joueur et Lieu
    let mut lieux: Vec<Lieu> = Vec::new();
    let mut joueurs: Vec<Joueur> = Vec::new();

    for obj in  objets {
        match obj {
            Objet::Joueur(joueur) => joueurs.push(joueur),  // Récupère les joueurs
            Objet::Lieu(lieu) => lieux.push(lieu),  // Récupère les lieux
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


     // TEST::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
    if let Some(joueur) = joueurs.get_mut(0) {  // On prend le premier joueur

        println!("Lieu actuel du joueur  {}", joueur.position);
        move_joueur(joueur, "E", &lieux);  // Exemple de déplacement vers la direction "E"
        println!("Lieu actuel du joueur  {}", joueur.position);
        move_joueur(joueur, "P", &lieux);  // Exemple de déplacement vers la direction "E"
        println!("Lieu actuel du joueur  {}", joueur.position);

    }
    
}



