# entropyscan-rs

Entropy scanner for threat hunting. Also, a teaching project.

The project is broken into separate "Stages" of development.

## Stage 0: MVP

This is the minimum viable product, in which we implement our entropy calculation algorithm, and point it to a file to calculate. Very bare-bones, but this is where we start.

## Stage 1: Recursion!

Now we add the ability to handle entire directories, by recursively collecting viable targets (true files) from our parent path argument. We also begin to use `PathBuf` correctly.
