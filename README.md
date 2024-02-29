# entropyscan-rs

Entropy scanner for threat hunting. Also, a teaching project.

The project is broken into separate "Stages" of development.

## Stage 0: MVP

This is the minimum viable product, in which we implement our entropy calculation algorithm, and point it to a file to calculate. Very bare-bones, but this is where we start.

## Stage 1: Recursion!

Now we add the ability to handle entire directories, by recursively collecting viable targets (true files) from our parent path argument. We also begin to use `PathBuf` correctly.

## Stage 2: Minimum Entropy

As our goal is to find suspiciously entropic files, it might be useful to be able to set a minimum entropy, under which the tool does not report. In this stage, this is handled by a positional argument.

Also, now that we have two arguments, we include a little `usage()` function, but this is quickly going to become annoying. I wonder if there's a better way to handle CLI args in Rust...

## Stage 3: Clap!

Before we add another feature, we need to clean up our CLI option management. It's time to use [clap](https://github.com/clap-rs/clap) to simplify our CLI configuraiton. This constitutes a major rebuild, but it will pay dividends in the long run.

## Stage 4: Modules

A big leap! In this stage, we refactor our code to get the logic out of `main.rs` and into a separate module. This lays the groundwork for easily adding new features.

## Stage 5: Structs And a Lot of Math

We want to add some new features to our scanner, but to do that, we need to start thinking about the data more formally. In this stage, we a `FileEntropy` struct that helps us contain useful information in one place.

But the biggest change here is in the addition of the `stats` module, which adds statistical calculations about our discovered files. Although not yet used in the program, getting this code down is a major change for this stage. This includes an `IQR` struct to help us contain data about the interquartile range for outliers.

## Stage 6: Subcommands

We laid the groundwork in Stage 5 to provide statistical information about our scan target: average and median entropies, variance, and even outliers based on the IQR method. This output is so much different than the standard listing, it should probably be a separate command. Luckily, we've already implemented Clap, so refactoring to subcommands will be relatively easy. We will have to write the code to print the stats, but that's no biggie, given our nice clean functions.

## Stage 7: Creature Comforts

We've added all the functionality we really wanted, so now we get to think about making our user interface a little nicer. In particular, it'd be nice if the output was nicer than just tab-separated fields. Let's make the default output a nice visual table. To accomplish this, we'll add another dependency: the [tabled](https://github.com/zhiburt/tabled) library.

## Stage 8: Formats

Our last order of business is to provide the user some options for output. The default table is nice, but we may want to make the data usable in other tools. For this, we'll provide 2 other options: CSV and JSON outputs, gated by the `-f` or `--format` options. This will be available for both `stats` and `scan` subcommands.

This introduces an invaluable library: `serde-json`.