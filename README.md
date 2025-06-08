# One Piece - Jeu de Rôle Textuel en Rust

Bienvenue dans **One Piece**, un jeu d’aventure textuel inspiré de l’univers du manga. Parcourez les îles, combattez des ennemis emblématiques, trouvez le One Piece et devenez le Roi des Pirates !

## Installation

1. **Prérequis**  
   - [Rust](https://www.rust-lang.org/tools/install) (stable)
   - Avoir tous les fichiers de données JSON (`joueur.json`, `lieu.json`, `sous_lieux.json`, `objetstatic.json`, `pnj.json`, `fruitdemon.json`, `aliments.json`, `objetmobile.json`) dans le dossier racine du projet.

2. **Compilation**
   ```bash
   cargo build 
   ```

3. **Lancement du jeu**
   ```bash
   cargo run 
   ```

---

## Objectif du jeu

Votre but est de parcourir les îles, affronter les personnages de l’univers One Piece, collecter les 4 Poneglyphes et enfin découvrir le "One Piece" pour devenir le Roi des Pirates.

---

## Manuel d’utilisation

### Démarrage

- Au lancement, choisissez votre nom de pirate.
- Si un fruit du démon est présent au point de départ, le jeu vous proposera de le manger.

### Menu du jeu

À chaque tour, le menu suivant s’affiche :

1. **Se déplacer**
   - Saisir la direction (N/S/E/O) pour changer d’île (si le bateau est à votre position et que vous possédez la clé/l’objet requis).
2. **Ramasser les objets**
   - Récupérez les objets statiques ou aliments présents dans la zone.
   - Si vous trouvez le "One Piece", le jeu se termine et vous gagnez !
3. **Parler/Combattre un PNJ**
   - Indiquez le nom d’un PNJ présent pour interagir, déclencher une discussion, un entraînement ou un combat.
   - Certains combats nécessitent des objets spécifiques dans votre inventaire.
4. **Voir l’inventaire**
   - Liste tous vos objets et aliments.
5. **Voir la description du lieu**
   - Affiche le lieu, les sous-lieux, objets et PNJ présents autour de vous.
6. **Capturer un fruit du démon**
   - Si un fruit du démon est disponible dans la zone, vous pouvez le manger pour obtenir de nouveaux pouvoirs.
7. **Afficher les statistiques du joueur**
   - Affiche votre HP, puissance, fruit du démon et attaques spéciales.
8. **Mini-jeux amusants**
   - Devinette, pile ou face, calcul mental… pour faire une pause !
9. **Consommer un aliment**
   - Restaure vos HP (uniquement si vos HP sont < 100).
10. **Se déplacer à l’intérieur d’un lieu**
    - Déplacez-vous entre les sous-zones d’une île (N/S/E/O).
Q. **Quitter**
    - Sauvegarde non implémentée (la partie sera perdue à la fermeture).

---

## Conseils de jeu et règles spéciales

- **Déplacement entre les îles** : Le bateau doit être stationné dans votre sous-zone et vous devez posséder l’objet-clé de la prochaine île (boussole, clé, map, poissonkoi…).
- **Combats** : Certains ennemis sont imbattables sans entraînement ou objet spécial (ex : Crocodile sans eau, Doflamingo sans épée, Akainu/Kaido sans entraînement).
- **Entraînement** : Certains PNJ (ex : Rayleigh, Hyogoro) peuvent vous entraîner si vos HP sont suffisants.
- **Consommation d’aliments** : Impossible de manger si vos HP sont déjà à 100.
- **Gérer son inventaire** : Les objets clés permettent d’accéder à de nouvelles zones ou de gagner des combats importants.

---

## Exemple de déroulement

1. Explorez les sous-zones d’Alabasta, parlez à Vivi pour obtenir l’eau.
2. Combattez Crocodile avec l’eau ➔ obtenez la boussole et le 1er poneglyphe.
3. Naviguez vers Water 7, trouvez les indices, obtenez la clé Marineford…
4. Continuez l’aventure sur chaque île, récupérez les objets-clés et affrontez les boss.
5. Rassemblez les 4 poneglyphes pour débloquer Laugh Tale et tentez de trouver le One Piece !

---

## Astuces techniques

- Le jeu se joue entièrement en ligne de commande.
- Utilisez uniquement des entrées clavier simples (lettres, chiffres).
- Si vous bloquez, vérifiez votre inventaire et votre position/sous-zone.
- Les fruits du démon et leurs attaques spéciales sont essentiels pour battre certains boss.

---

## Limitations connues

- Il n’y a pas de sauvegarde automatique.
- Les mini-jeux sont optionnels et n’influencent pas la progression principale.
- Toutes les interactions sont en français.

---
