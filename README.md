# kantan
## goals

Create a simple command line interface for automatically starting a generic project.
In general something like `kantan init myProject` would start an interface with which to configure a new project.
I could then select the template(language, build tool) that I wanted to use for that project. 
For example - Selecting the C template would make a directory structure specialized for C and using Make for builds. It would also provide a generic makefile too.
Choosing rust would have a different file structure and use Cargo for building

The tool would then run any other steps, like initializng git or creating a venv or whatever else needs to happen.

Potentially, I could also use something like `kantan package myProject <operating_system>` to create a package to be installed on a particular system. 
This is a secondary goal once the first part is robust. 

My primary goal is to have a fairly generic organizational scheme for all of my projects to keep developmentstreamlined and efficient. Kantan will know how to tweak my generic schema to be optimized for one language or another. Multi language features should be an extension to this, either by seperate templates or by modularizing things such that it can create projects within projects.

As a secondary goal, I basically want to abstract all the different tools and steps to building and packaging a project into one streamlined interface. 
