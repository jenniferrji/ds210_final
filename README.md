# ds210_final
DS210 Final Project: Marvel Universe Network Analysis
Purpose
For my DS210 final project, I decided to try and create a graph that represents the intricate connections between Marvel characters and their appearances. In the end, I settled on creating a degree distributions graph. My final goal was to use network analysis to uncover the hidden links and relationships among the many characters in the Marvel Universe, including comics, films and TV shows alike. 

Dataset
I began this adventure by sourcing data from Kaggle, specifically choosing the 'edges.csv' file. To make this data more user-friendly for my purposes, I converted it from its original CSV format into a TXT file. This dataset served as the foundation of my project, providing the crucial connections between characters that I sought to analyze.
Dataset Link: https://www.kaggle.com/datasets/csanhueza/the-marvel-universe-social-network

Implementation
I chose to use HashMap and Hashset through Rust. First, I used HashMap to effectively group characters together based on their comic appearances. I chose to use HashSet in order to ensure that each and every character would be represented as a unique entry in order to maintain the accuracy of my analysis. In order to run the project, you must first run “cargo build” to compile the code, and then follow it with “cargo run” in order to see the output. 
For my graph, I focused on two main computations. The first one I chose was determining the degree distribution of the vertices and the second one was calculating the number of neighbors each character has at a distance of 2. The first part of the output provides an insightful look into the social network, by calculating the number of different marvel characters connected to each other within two degrees of separation. For instance, if the output read "Hero: DEATHWATCH/STEPHAN L, Neighbors at distance 2: 1737," it means that the character DEATHWATCH/STEPHAN L had connections at a distance 2 with 1737 other characters. Secondly, if the output says “Characters with 83 connections: 13,” it means that 13 characters in the social network have exactly 83 connections to other characters. My code also outputs the minimum number of connections for a hero, which was 1, and maximum number of connections for a hero, which was 1345. This means that every character is connected to another character, since the minimum number is 1. It also shows that one of the characters is connected to 1345 other characters, showing how deep the Marvel Universe storyline runs.

Results 
The results from this project provide a look into the structure of the Marvel Universe and the interactions with the many characters in the world. For example, it provides insight to the central characters in the universe based on the connections between other characters. It also indicates that some supporting characters may have more of an influence than initially presumed in the storyline. It provides new information about the influence of the characters. It is interesting to see the storyline turned into numbers. 
