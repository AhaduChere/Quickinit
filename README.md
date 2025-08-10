# Quickinit

A command-line tool I'm developing to streamline project structuring and learn Rust.
## Installation 
### Download binary 
[GitHub Releases](https://github.com/AhaduChere/Quickinit/releases/latest)
### Move the binary to a directory in system PATH like:
```bash
sudo mv Quickinit /usr/local/bin/
```
### or
```bash
sudo mv Quickinit ~/.local/bin
```


## Usage
0. **Pre-Check**  
   - NPM  
   - Cargo  
   - Essential packages (create-vite, TailwindCSS)  
1. **Enter project name**  
2. **Choose language**  
   - **Rust** → *Create Rust project*  
   - **JavaScript**  
     - **Choose framework**  
        - **React**  
          - **Use TailwindCSS?**  
            - Yes → *Create React project with TailwindCSS*  
            - No → *Create React project without TailwindCSS*  
        - **Vue**  
          - **Use TailwindCSS?**  
            - Yes → *Create Vue project with TailwindCSS*  
            - No → *Create Vue project without TailwindCSS* 

## Requirements
- **Cargo**
- **NPM**
  - **create-vite**
  - **TailwindCSS**
 
## Future Plans  
- Replace React with Next.js  
- Replace Vue with Nuxt.js
- Only check dependencies when needed

## License
This project is licensed under the MIT License.
