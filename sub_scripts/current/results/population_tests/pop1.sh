#!/bin/sh

##Place PBS directives here
#PBS -N pop1
#PBS -l nodes=1:ppn=36
#PBS -l walltime=72:00:00
#PBS -M ahansen2@trinity.edu
#PBS -m ae

##Place Linux commands to run on the remote node here
time ~/thesis/genetic_painting/target/release/genetic_painting --file 'image.jpg' --iterations 5000 --population 300 --selector parmaximize --strokes 5000 --strokewidth 20 -v 1 -m 5 -M 300  --maxcurve 5
