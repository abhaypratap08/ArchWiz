See THE architecture of your codebase


A terminal based CLI written in Rust that converts your project structure into an architectural flowchart.


Where `tree` simply lists all files and folders, ArchWiz shows you how everything is organised in a visual way.


## Features


Visualise THE architecture of your project


Smart categorisation of source, test, config, documentation, and asset files.


Intelligently group your files.


Evenly placed with orthogonal connections.


View statistics about your project such as THE number of files, languages used, directories and storage used.


Navigate and explore your project.


Respects `.gitignore` and ignores generated files.


## Installation


Install directly with:


```bash


curl -fsSL https://raw.githubusercontent.com/abhaypratap08/ArchWiz/main/install.sh | sh


```


You will need Rust and Cargo to install from source.


## Usage


```bash


archwiz


archwiz ~/Projects/YourProject


archwiz --stats .


archwiz --interactive .


archwiz --help
```

--- 

## Status


ArchWiz is a work in progress to learn about Rust, file traversal, graph algorithms, terminal rendering and visualising software architecture.


Note: ArchWiz does not look at any real source CODE dependencies, imports or relationships. It only looks at how files and directories are organised and grouped together.


---


ARCHWIZ - See THE SYSTEM BEHIND your CODE
