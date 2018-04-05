#!/bin/sh

##Place PBS directives here
#PBS -N balloons
#PBS -l nodes=1:ppn=36
#PBS -l walltime=72:00:00
#PBS -M ahansen2@trinity.edu
#PBS -m ae

##Place Linux commands to run on the remote node here
time ~/thesis/genetic_painting/target/release/genetic_painting --file 'hotairballoons.jpg' --iterations 5000 --population 30 --selector parmaximize --strokes 5000 --strokewidth 15 -v 1 -m 4 -M 25  --maxcurve 5 &> /users/ahansen2/balloons.out
