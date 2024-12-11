# Générateur d'horaire de l'AEP v2

## 🛠️ Objectif du projet

Le Générateur d'horaire de l'AEP v2 est une refonte complète du générateur d'horaire étudiant, conçu pour être rapide, flexible et accessible. Ce projet modernise une base de code vieille de 15 ans et optimise l'expérience utilisateur en adoptant des technologies modernes.

## ❓ FAQ

### Pourquoi une refonte complète ?

L'ancien générateur, en maintenance depuis plus de 15 ans, était obsolète avec une base de code mêlant 5 langages différents. Une refonte était essentielle pour garantir la maintenabilité et améliorer les performances.

### Comment est-il aussi rapide ?

- **Traitement local** : Les horaires sont générés directement dans le navigateur via WebAssembly, éliminant les délais liés aux serveurs.
- **Optimisation intelligente** : Seuls les meilleurs horaires sont calculés, grâce à des algorithmes comme le *branch and bound* pour réduire l'espace de recherche et respecter les contraintes définies.

### Où puis-je soumettre des suggestions ou rapports de bugs ?

- Utilisez le bouton **Signaler un bug** pour soumettre vos idées ou signaler un problème.
- Contributions bienvenues ! Rejoignez-nous si vous avez des compétences en Rust.

## 👥 Crédits

- **Développement** : Marc-Antoine Manningham (front-end & back-end)
- **Design** : Raphael Salvas, Achille Saint-Hillier, Sunnee Chevalier et Gabriel Billard, inspirés par leur maquette Figma.


Voici une version mise à jour du README, incluant des instructions pour l'auto-hébergement avec Docker :

## ⚙️ Auto-hébergement avec Docker

### Prérequis

1. **Docker** : Assurez-vous que Docker est installé. [Installation Docker](https://docs.docker.com/get-docker/)
2. **Docker Compose** : (optionnel, selon votre configuration).

### Instructions

1. **Téléchargez et exécutez l'image Docker** :
   ```bash
   docker pull git.step.polymtl.ca/Lemark/aep-schedule-generator:latest
   docker run -d -p 8080:8080 --name aep-schedule-generator git.step.polymtl.ca/Lemark/aep-schedule-generator:latest
   ```

2. **Accédez à l'application** :
   Ouvrez votre navigateur à l'adresse [http://localhost:8080](http://localhost:8080).

3. **Configurer avec Docker Compose** (facultatif) :
   Créez un fichier `docker-compose.yml` :
   ```yaml
   version: '3'
   services:
     aep-schedule-generator:
       image: git.step.polymtl.ca/Lemark/aep-schedule-generator:latest
       ports:
         - "8080:8080"
   ```
   Lancez les services :
   ```bash
   docker-compose up -d
   ```
